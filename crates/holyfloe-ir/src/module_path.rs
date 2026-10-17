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
    let rewrite = cfg
        .module_rewrite
        .iter()
        .filter(|(from, _)| out.starts_with(from.as_str()))
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
/// and longer pattern prefixes beat shorter ones.
fn subpath(exports: &Value, file: &str) -> Option<String> {
    let entries: Vec<(&str, &Value)> = match exports {
        Value::Object(map) if map.keys().all(|k| k.starts_with('.')) => {
            map.iter().map(|(k, v)| (k.as_str(), v)).collect()
        }
        // A string or a condition object is shorthand for `{ ".": ... }`.
        other => vec![(".", other)],
    };
    let mut best: Option<(usize, String)> = None;
    for (key, value) in entries {
        let Some(target) = target(value).map(strip_root) else {
            continue;
        };
        let candidate = match (key.split_once('*'), target.split_once('*')) {
            (None, None) if target == file => (usize::MAX, key.to_owned()),
            (Some((key_pre, key_post)), Some((pre, post)))
                if file.len() >= pre.len() + post.len()
                    && file.starts_with(pre)
                    && file.ends_with(post) =>
            {
                let star = &file[pre.len()..file.len() - post.len()];
                (pre.len(), format!("{key_pre}{star}{key_post}"))
            }
            _ => continue,
        };
        if best.as_ref().is_none_or(|(rank, _)| candidate.0 > *rank) {
            best = Some(candidate);
        }
    }
    best.map(|(_, sub)| sub)
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
    fn normalize_absorbs_analyzer_spellings() {
        // OQ-015: superclass.module vs inheritedFrom.module.
        assert_eq!(
            normalize("/src/x/fx-base.js"),
            normalize("src/x/fx-base.ts")
        );
        assert_eq!(normalize("./src/a.tsx"), "src/a");
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
