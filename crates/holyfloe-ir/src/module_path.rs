//! CEM module path -> import specifier, matched against `package.json` `exports`.

use serde_json::Value;

use crate::{Config, PackageInfo};

const EXTENSIONS: [&str; 8] = [".tsx", ".ts", ".mts", ".cts", ".jsx", ".js", ".mjs", ".cjs"];

fn strip_root(path: &str) -> &str {
    path.strip_prefix("./")
        .or_else(|| path.strip_prefix('/'))
        .unwrap_or(path)
}

/// Identity of a module regardless of how the analyzer spelled it:
/// `/src/x/fx-base.js` and `src/x/fx-base.ts` are the same module.
pub(crate) fn normalize(path: &str) -> &str {
    let path = strip_root(path);
    EXTENSIONS
        .iter()
        .find_map(|ext| path.strip_suffix(ext))
        .unwrap_or(path)
}

/// The package an import specifier points into: `@scope/pkg/x.js` -> `@scope/pkg`, `lit` -> `lit`.
/// The analyzer writes the full specifier as a reference's `package`.
pub(crate) fn package_name(specifier: &str) -> &str {
    let segments = if specifier.starts_with('@') { 2 } else { 1 };
    match specifier.match_indices('/').nth(segments - 1) {
        Some((i, _)) => &specifier[..i],
        None => specifier,
    }
}

/// A module named inside the manifest entry of module `from` (a re-export, a superclass, a
/// type reference), as a package-root path. `./` and `../` are relative to `from`: the analyzer
/// keeps re-export specifiers verbatim (Lion: `../components/accordion/src/LionAccordion.js`).
/// `/x` and `x` are already root-relative.
pub(crate) fn resolve_ref(from: &str, reference: &str) -> String {
    if !(reference.starts_with("./") || reference.starts_with("../")) {
        return strip_root(reference).to_owned();
    }
    let mut parts: Vec<&str> = strip_root(from).split('/').collect();
    parts.pop(); // the file name
    for segment in reference.split('/') {
        match segment {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            s => parts.push(s),
        }
    }
    parts.join("/")
}

/// The published file a source module compiles to: `.ts` -> `.js`, then `moduleRewrite`.
pub(crate) fn output_path(path: &str, cfg: &Config) -> String {
    let path = strip_root(path);
    let mut out = [
        (".mts", ".mjs"),
        (".cts", ".cjs"),
        (".tsx", ".js"),
        (".ts", ".js"),
    ]
    .iter()
    .find_map(|(from, to)| path.strip_suffix(from).map(|stem| format!("{stem}{to}")))
    .unwrap_or_else(|| path.to_owned());
    // Keys and values are normalized like paths, so `./src/` works as well as `src/`.
    let rewrite = cfg
        .module_rewrite
        .iter()
        .map(|(from, to)| (strip_root(from), strip_root(to)))
        .filter(|(from, _)| out.starts_with(from))
        .max_by_key(|(from, _)| from.len());
    if let Some((from, to)) = rewrite {
        out = format!("{to}{}", &out[from.len()..]);
    }
    out
}

/// `Err` carries the output path that matched no export, for the error message.
pub(crate) fn specifier(path: &str, pkg: &PackageInfo, cfg: &Config) -> Result<String, String> {
    let file = output_path(path, cfg);
    let Some(exports) = &pkg.exports else {
        // No `exports`: every file is importable.
        return Ok(format!("{}/{file}", pkg.name));
    };
    match subpath(exports, &file) {
        Some(sub) if sub == "." => Ok(pkg.name.clone()),
        Some(sub) => Ok(format!("{}/{}", pkg.name, strip_root(&sub))),
        None => Err(file),
    }
}

/// The export subpath (`.`, `./x.js`) whose target is `file`. Exact entries beat `*` patterns,
/// and longer pattern prefixes beat shorter ones. A candidate only counts if Node, resolving
/// it forward, lands on `file` again: that honours `null` exclusions and shadowing keys.
fn subpath(exports: &Value, file: &str) -> Option<String> {
    let entries: Vec<(&str, &Value)> = match exports {
        Value::Object(map) if map.keys().all(|k| k.starts_with('.')) => {
            map.iter().map(|(k, v)| (k.as_str(), v)).collect()
        }
        // A string or a condition object is shorthand for `{ ".": ... }`.
        other => vec![(".", other)],
    };
    let mut candidates = Vec::new();
    for (key, value) in &entries {
        let Some(target) = target(value).map(strip_root) else {
            continue;
        };
        match (key.split_once('*'), target.split_once('*')) {
            (None, None) if target == file => candidates.push((usize::MAX, key.to_string())),
            (Some((key_pre, key_post)), Some((pre, post)))
                if file.len() >= pre.len() + post.len()
                    && file.starts_with(pre)
                    && file.ends_with(post) =>
            {
                let star = &file[pre.len()..file.len() - post.len()];
                candidates.push((pre.len(), format!("{key_pre}{star}{key_post}")));
            }
            _ => {}
        }
    }
    // Stable sort: on equal rank the first entry in `exports` wins.
    candidates.sort_by_key(|c| std::cmp::Reverse(c.0));
    candidates
        .into_iter()
        .map(|(_, sub)| sub)
        .find(|sub| resolve(&entries, sub).as_deref() == Some(file))
}

/// Node's forward lookup: an exact key, else the pattern key with the longest prefix before
/// `*`. `None` when the chosen key's target is `null` (excluded) or there is no match.
fn resolve(entries: &[(&str, &Value)], sub: &str) -> Option<String> {
    if let Some((_, value)) = entries.iter().find(|(key, _)| *key == sub) {
        return target(value).map(|t| strip_root(t).to_owned());
    }
    let (star, value) = entries
        .iter()
        .filter_map(|(key, value)| {
            let (pre, post) = key.split_once('*')?;
            let matches =
                sub.len() >= pre.len() + post.len() && sub.starts_with(pre) && sub.ends_with(post);
            matches.then(|| {
                (
                    pre.len(),
                    key.len(),
                    &sub[pre.len()..sub.len() - post.len()],
                    value,
                )
            })
        })
        .max_by_key(|(pre_len, key_len, _, _)| (*pre_len, *key_len))
        .map(|(_, _, star, value)| (star, value))?;
    target(value).map(|t| strip_root(t).replacen('*', star, 1))
}

/// Resolve a target, preferring the `import` condition over `default`.
fn target(value: &Value) -> Option<&str> {
    match value {
        Value::String(s) => Some(s),
        Value::Object(map) => ["import", "default"]
            .iter()
            .find_map(|c| map.get(*c).and_then(target)),
        Value::Array(items) => items.iter().find_map(target),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn pkg(exports: Value) -> PackageInfo {
        PackageInfo {
            name: "@acme/ui".into(),
            version: None,
            exports: Some(exports),
        }
    }

    fn cfg() -> Config {
        Config {
            module_rewrite: [("src/".into(), "dist/".into())].into(),
            ..Config::default()
        }
    }

    #[test]
    fn package_names() {
        assert_eq!(
            package_name("@holyfloe/fixtures-dep/focusable-mixin.js"),
            "@holyfloe/fixtures-dep"
        );
        assert_eq!(
            package_name("@holyfloe/fixtures-dep"),
            "@holyfloe/fixtures-dep"
        );
        assert_eq!(package_name("lit"), "lit");
        assert_eq!(package_name("lit/decorators.js"), "lit");
    }

    #[test]
    fn normalize_absorbs_analyzer_spellings() {
        // D-055: superclass.module vs inheritedFrom.module.
        assert_eq!(
            normalize("/src/x/fx-base.js"),
            normalize("src/x/fx-base.ts")
        );
        assert_eq!(normalize("./src/a.tsx"), "src/a");
    }

    #[test]
    fn references_resolve_relative_to_their_module() {
        assert_eq!(
            resolve_ref("exports/accordion.js", "../components/a/A.js"),
            "components/a/A.js"
        );
        assert_eq!(resolve_ref("src/a/b.ts", "./c.js"), "src/a/c.js");
        assert_eq!(resolve_ref("src/a/b.ts", "/src/x.js"), "src/x.js");
        assert_eq!(resolve_ref("src/a/b.ts", "src/x.ts"), "src/x.ts");
    }

    #[test]
    fn output_path_swaps_extension_then_rewrites() {
        assert_eq!(output_path("/src/a/b.ts", &cfg()), "dist/a/b.js");
        assert_eq!(output_path("./src/a.mts", &cfg()), "dist/a.mjs");
        assert_eq!(output_path("lib/a.js", &cfg()), "lib/a.js");
    }

    #[test]
    fn longest_rewrite_prefix_wins() {
        let cfg = Config {
            module_rewrite: [
                ("src/".into(), "dist/".into()),
                ("src/legacy/".into(), "old/".into()),
            ]
            .into(),
            ..Config::default()
        };
        assert_eq!(output_path("src/legacy/a.ts", &cfg), "old/a.js");
    }

    #[test]
    fn pattern_export() {
        let p = pkg(json!({ "./*": "./dist/*" }));
        assert_eq!(
            specifier("src/fx-button/fx-button.ts", &p, &cfg()),
            Ok("@acme/ui/fx-button/fx-button.js".into())
        );
    }

    #[test]
    fn exact_export_beats_pattern_and_conditions_prefer_import() {
        let p = pkg(json!({
            ".": { "types": "./dist/index.d.ts", "import": "./dist/index.js" },
            "./button": { "default": "./dist/button.js" },
            "./*": "./dist/*"
        }));
        assert_eq!(specifier("src/index.ts", &p, &cfg()), Ok("@acme/ui".into()));
        assert_eq!(
            specifier("src/button.ts", &p, &cfg()),
            Ok("@acme/ui/button".into())
        );
        assert_eq!(
            specifier("src/other.ts", &p, &cfg()),
            Ok("@acme/ui/other.js".into())
        );
    }

    #[test]
    fn string_exports_and_suffix_patterns() {
        let p = pkg(json!("./dist/index.js"));
        assert_eq!(specifier("src/index.ts", &p, &cfg()), Ok("@acme/ui".into()));
        let p = pkg(json!({ "./components/*.js": "./dist/c/*.js" }));
        assert_eq!(
            specifier("src/c/a.ts", &p, &cfg()),
            Ok("@acme/ui/components/a.js".into())
        );
    }

    #[test]
    fn null_exclusions_and_shadowing_keys_are_honoured() {
        let p = pkg(json!({
            "./*": "./dist/*",
            "./internal/*": null,
            "./button.js": "./dist/legacy-button.js"
        }));
        assert_eq!(
            specifier("src/internal/a.ts", &p, &cfg()),
            Err("dist/internal/a.js".into())
        );
        // `./button.js` is taken by an exact key, so dist/button.js is not importable.
        assert_eq!(
            specifier("src/button.ts", &p, &cfg()),
            Err("dist/button.js".into())
        );
        assert_eq!(
            specifier("src/legacy-button.ts", &p, &cfg()),
            Ok("@acme/ui/button.js".into())
        );
    }

    #[test]
    fn rewrite_keys_may_start_with_dot_slash() {
        let cfg = Config {
            module_rewrite: [("./src/".into(), "./dist/".into())].into(),
            ..Config::default()
        };
        assert_eq!(output_path("src/a.ts", &cfg), "dist/a.js");
    }

    #[test]
    fn unmatched_reports_output_path() {
        let p = pkg(json!({ ".": "./dist/index.js" }));
        assert_eq!(specifier("src/a.ts", &p, &cfg()), Err("dist/a.js".into()));
    }

    #[test]
    fn no_exports_allows_deep_imports() {
        let p = PackageInfo {
            name: "x".into(),
            ..PackageInfo::default()
        };
        assert_eq!(specifier("src/a.ts", &p, &cfg()), Ok("x/dist/a.js".into()));
    }
}
