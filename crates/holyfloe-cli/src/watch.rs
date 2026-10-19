//! Re-run `generate` when the manifest or config changes.

use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use notify::{EventKind, RecursiveMode};
use notify_debouncer_full::{DebounceEventResult, Debouncer, RecommendedCache, new_debouncer};

use crate::{Error, Options, Run, generate};

/// Stops watching when dropped.
pub struct Watcher(#[allow(dead_code)] Debouncer<notify::RecommendedWatcher, RecommendedCache>);

/// Calls `on_run` with a fresh `generate` after each change to `opts.cem` or `opts.config`. It
/// does not run `generate` first: call it after this returns, so no change is missed. The parent
/// directories are watched, not the files, because the analyzer replaces the manifest and a
/// file watch can lose it on rename.
// shortcut: dependency manifests are not watched; restart to pick up a changed node_modules
// manifest.
pub fn watch(
    opts: Options,
    mut on_run: impl FnMut(Result<Vec<Run>, Error>) + Send + 'static,
) -> Result<Watcher, Error> {
    let mut files: Vec<PathBuf> = Vec::new();
    for path in std::iter::once(&opts.cem).chain(&opts.config) {
        let abs = std::path::absolute(path).map_err(|e| Error::at(path, e))?;
        let dir = abs.parent().unwrap_or(&abs);
        // Events carry real paths (`/private/var/...` on macOS).
        let dir = fs::canonicalize(dir).map_err(|e| Error::at(dir, e))?;
        files.push(dir.join(abs.file_name().unwrap_or_default()));
    }
    let watched = files.clone();
    let mut debouncer = new_debouncer(
        Duration::from_millis(100),
        None,
        move |res: DebounceEventResult| match res {
            Ok(events) => {
                // Access events are skipped: `generate` itself reads the manifest.
                let hit = events.iter().any(|ev| {
                    !matches!(ev.kind, EventKind::Access(_))
                        && ev.paths.iter().any(|p| watched.contains(p))
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
    let mut dirs: Vec<_> = files.iter().filter_map(|f| f.parent()).collect();
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
            cem: cem.clone(),
            out: root.join("out"),
            frameworks: vec![Framework::React],
            config: None,
            react_mode: None,
        };
        let (tx, rx) = mpsc::channel();
        let _w = watch(opts(), move |r| tx.send(r).unwrap()).unwrap();
        generate(&opts()).unwrap();

        // Replace the file the way the analyzer does: write elsewhere, rename over it.
        let tmp = root.join("tmp.json");
        fs::write(&tmp, manifest("two")).unwrap();
        fs::rename(&tmp, &cem).unwrap();
        let runs = rx.recv_timeout(Duration::from_secs(10)).unwrap().unwrap();
        assert!(!runs[0].report.written.is_empty(), "{runs:?}");
        let out = fs::read_to_string(root.join("out/react/XA.tsx")).unwrap();
        assert!(out.contains("two"), "{out}");
        fs::remove_dir_all(&root).unwrap();
    }
}
