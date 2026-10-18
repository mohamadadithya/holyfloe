//! Inputs besides the manifest: `package.json` and the `holyfloe.config.json` subset the IR reads.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// The fields of `package.json` the IR needs. Deserializes straight from the file.
#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
pub struct PackageInfo {
    pub name: String,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub exports: Option<serde_json::Value>,
}

/// Unknown keys (`cem`, `outDir`, `react`, ...) are ignored here; other layers read them.
#[derive(Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase", default)]
pub struct Config {
    /// Path prefix pairs applied after `.ts` -> `.js`, e.g. `{"src/": "dist/"}`. Longest prefix wins.
    pub module_rewrite: BTreeMap<String, String>,
    pub bindings: BindingsMode,
    /// Not supported yet: non-null is `E010`.
    pub tag_formatter: Option<serde_json::Value>,
    /// Not supported yet: non-null is `E010`.
    pub component_name_formatter: Option<serde_json::Value>,
    /// Keyed by tag name.
    pub overrides: BTreeMap<String, ComponentOverride>,
    /// Tag globs to skip, e.g. `internal-*`. `*` matches any run of characters.
    pub exclude: Vec<String>,
}

impl Config {
    pub(crate) fn excludes(&self, tag: &str) -> bool {
        self.exclude.iter().any(|pattern| glob(pattern, tag))
    }
}

fn glob(pattern: &str, text: &str) -> bool {
    let mut parts = pattern.split('*');
    let first = parts.next().unwrap_or_default();
    let Some(mut rest) = text.strip_prefix(first) else {
        return false;
    };
    let parts: Vec<_> = parts.collect();
    let Some((last, middle)) = parts.split_last() else {
        return rest.is_empty(); // no `*`: exact match
    };
    for part in middle {
        match rest.find(part) {
            Some(i) => rest = &rest[i + part.len()..],
            None => return false,
        }
    }
    rest.ends_with(last)
}

#[cfg(test)]
mod tests {
    use super::glob;

    #[test]
    fn globs() {
        assert!(glob("internal-*", "internal-row"));
        assert!(!glob("internal-*", "x-internal-row"));
        assert!(glob("*-row", "x-row"));
        assert!(glob("x-*-*", "x-a-b"));
        assert!(glob("x-a", "x-a"));
        assert!(!glob("x-a", "x-ab"));
        assert!(!glob("x-*a*a", "x-a"));
        assert!(glob("*", "anything"));
    }
}

#[derive(Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum BindingsMode {
    #[default]
    Infer,
    ExplicitOnly,
}

#[derive(Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase", default)]
pub struct ComponentOverride {
    /// DOM event name -> base prop name (no framework prefix), e.g. `"myChange": "MyChangeCamel"`.
    pub events: BTreeMap<String, String>,
    /// Props to leave out of every wrapper. The manifest cannot tell a stale JSDoc `@property`
    /// (Web Awesome `wa-drawer` `modal`) from a real untyped field, and `El['prop']` fails on it.
    pub exclude_props: Vec<String>,
}
