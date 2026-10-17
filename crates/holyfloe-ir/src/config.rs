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
}

#[derive(Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum BindingsMode {
    #[default]
    Infer,
    ExplicitOnly,
}

#[derive(Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(default)]
pub struct ComponentOverride {
    /// DOM event name -> base prop name (no framework prefix), e.g. `"myChange": "MyChangeCamel"`.
    pub events: BTreeMap<String, String>,
}
