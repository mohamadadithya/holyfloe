//! Re-run `generate` when the manifest or config changes.

use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use notify::{EventKind, RecursiveMode};
use notify_debouncer_full::{DebounceEventResult, Debouncer, RecommendedCache, new_debouncer};

use crate::{Error, Input, Options, Run, generate, is_source, real};

/// Stops watching when dropped.
pub struct Watcher(#[allow(dead_code)] Debouncer<notify::RecommendedWatcher, RecommendedCache>);

/// Calls `on_run` with a fresh `generate` after each change to the manifest (or a Lit source
/// file under the Lit directory, outside `out` and `node_modules`) or the config. It
/// does not run `generate` first: call it after this returns, so no change is missed. The parent
/// directories are watched, not the files: the analyzer rewrites the manifest in place, but
/// editors save by renaming a temp file over the original, and a file watch loses it then.
// shortcut: dependency manifests are not watched; restart to pick up a changed node_modules
// manifest.
pub fn watch(
    opts: Options,
    mut on_run: impl FnMut(Result<Vec<Run>, Error>) + Send + 'static,
) -> Result<Watcher, Error> {
    let (manifest, lit_dir) = match &opts.input {
        Input::Cem(path) => (Some(path), None),
        Input::Lit(dir) => (
            None,
            Some(fs::canonicalize(dir).map_err(|e| Error::at(dir, e))?),
        ),
    };
    let out = real(&opts.out);
    let mut files: Vec<PathBuf> = Vec::new();
    for path in manifest.into_iter().chain(&opts.config) {
        let abs = std::path::absolute(path).map_err(|e| Error::at(path, e))?;
        let dir = abs.parent().unwrap_or(&abs);
        // Events carry real paths (`/private/var/...` on macOS).
        let dir = fs::canonicalize(dir).map_err(|e| Error::at(dir, e))?;
        files.push(dir.join(abs.file_name().unwrap_or_default()));
    }
    let watched = files.clone();
    let lit = lit_dir.clone();
    let is_lit_source = move |p: &std::path::Path| {
        lit.as_ref().is_some_and(|dir| {
            p.starts_with(dir)
                && !p.starts_with(&out)
                && !p.components().any(|c| c.as_os_str() == "node_modules")
                && is_source(p)
        })
    };
    let mut debouncer = new_debouncer(
        Duration::from_millis(100),
        None,
        move |res: DebounceEventResult| match res {
            Ok(events) => {
                // Access events are skipped: `generate` itself reads the manifest.
                let hit = events.iter().any(|ev| {
                    !matches!(ev.kind, EventKind::Access(_))
                        && ev
                            .paths
                            .iter()
                            .any(|p| watched.contains(p) || is_lit_source(p))
                });
                if hit {
                    on_run(generate(&opts));
                }
            }
            Err(errors) => {
                for e in errors {
                    on_run(Err(Error::plain(format!("watch: {e}"))));
                }
            }
        },
    )
    .map_err(|e| Error::plain(format!("watch: {e}")))?;
    if let Some(dir) = &lit_dir {
        debouncer
            .watch(dir, RecursiveMode::Recursive)
            .map_err(|e| Error::at(dir, e))?;
    }
    let mut dirs: Vec<_> = files
        .iter()
        .filter_map(|f| f.parent())
        .filter(|d| lit_dir.as_ref().is_none_or(|l| !d.starts_with(l)))
        .collect();
    dirs.dedup();
    for dir in dirs {
        debouncer
            .watch(dir, RecursiveMode::NonRecursive)
            .map_err(|e| Error::at(dir, e))?;
    }
    Ok(Watcher(debouncer))
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;

    use super::*;
    use crate::Framework;

    #[test]
    fn reruns_on_manifest_change() {
        let root = std::env::temp_dir().join(format!("holyfloe-watch-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("lib")).unwrap();
        fs::write(root.join("package.json"), r#"{"name": "w"}"#).unwrap();
        let manifest = |desc: &str| {
            format!(
                r#"{{"schemaVersion": "1.0.0", "modules": [{{"kind": "javascript-module", "path": "lib/x-a.js",
                "declarations": [{{"kind": "class", "name": "XA", "tagName": "x-a", "customElement": true,
                "description": "{desc}"}}],
                "exports": [{{"kind": "js", "name": "XA", "declaration": {{"name": "XA", "module": "lib/x-a.js"}}}}]}}]}}"#
            )
        };
        let cem = root.join("custom-elements.json");
        fs::write(&cem, manifest("one")).unwrap();
        let opts = || Options {
            input: Input::Cem(cem.clone()),
            out: root.join("out"),
            frameworks: vec![Framework::React],
            config: None,
            react_mode: None,
        };
        let (tx, rx) = mpsc::channel();
        let _w = watch(opts(), move |r| tx.send(r).unwrap()).unwrap();
        generate(&opts()).unwrap();

        // In place, as the analyzer writes it; then a rename over it, as editors save.
        // A platform may report one write as several events, so wait for the run that has it.
        let next = |desc: &str| loop {
            let _ = rx.recv_timeout(Duration::from_secs(10)).unwrap();
            let out = fs::read_to_string(root.join("out/react/XA.tsx")).unwrap();
            if out.contains(desc) {
                break;
            }
        };
        fs::write(&cem, manifest("two")).unwrap();
        next("two");
        let tmp = root.join("tmp.json");
        fs::write(&tmp, manifest("three")).unwrap();
        fs::rename(&tmp, &cem).unwrap();
        next("three");
        fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn reruns_on_lit_source_change() {
        let root = std::env::temp_dir().join(format!("holyfloe-watch-lit-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(root.join("package.json"), r#"{"name": "w"}"#).unwrap();
        let source = |desc: &str| {
            format!(
                "/** {desc} */\nexport class XA extends HTMLElement {{}}\ncustomElements.define('x-a', XA);\n"
            )
        };
        let file = root.join("src/x-a.ts");
        fs::write(&file, source("one")).unwrap();
        // `out` inside the watched directory: writing wrappers must not trigger runs.
        let opts = || Options {
            input: Input::Lit(root.join("src")),
            out: root.join("src/out"),
            frameworks: vec![Framework::React],
            config: None,
            react_mode: None,
        };
        let (tx, rx) = mpsc::channel();
        let _w = watch(opts(), move |r| tx.send(r).unwrap()).unwrap();
        generate(&opts()).unwrap();
        fs::write(&file, source("two")).unwrap();
        loop {
            let _ = rx.recv_timeout(Duration::from_secs(10)).unwrap();
            let out = fs::read_to_string(root.join("src/out/react/XA.tsx")).unwrap();
            if out.contains("two") {
                break;
            }
        }
        // Settled: no further runs from the wrappers that run wrote.
        std::thread::sleep(Duration::from_millis(500));
        let extra: Vec<_> = rx.try_iter().filter_map(Result::ok).collect();
        assert!(
            extra
                .iter()
                .all(|runs| runs.iter().all(|r| r.report.written.is_empty())),
            "{extra:?}"
        );
        fs::remove_dir_all(&root).unwrap();
    }
}
