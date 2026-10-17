//! Serde types for Custom Elements Manifest 2.1.0 (typify-generated, checked in).

#[allow(clippy::all)]
mod generated;

pub use generated::*;

/// Top-level manifest. Written by hand: the schema root has no `title`,
/// so typify only emits the `definitions`.
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Package {
    pub schema_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme: Option<String>,
    pub modules: Vec<JavaScriptModule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<PackageDeprecated>,
}

/// `true`, or the reason the package is deprecated.
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum PackageDeprecated {
    Boolean(bool),
    String(String),
}
