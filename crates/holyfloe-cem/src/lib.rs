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

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("invalid manifest: {0}")]
    Json(#[from] serde_json::Error),
    #[error(
        "{module}: `{name}` looks like a custom element but does not match the CEM schema: {source}"
    )]
    Element {
        module: String,
        name: String,
        source: serde_json::Error,
    },
}

/// Parse a manifest. Use this, not `serde_json::from_str`: the declarations enum is untagged,
/// so an element that misses one required field would silently become a plain
/// `ClassDeclaration` and lose its tag. Here that is an error.
///
/// Other declarations and exports that match no schema variant are dropped instead of failing
/// the manifest: published manifests include bundled dependency code (Lion ships minified Lit
/// classes whose `superclass` has no `name`, and Popper exports with no declaration name), and
/// those never produce output. `custom-element-definition` exports carry tags, so stay strict.
pub fn parse(text: &str) -> Result<Package, ParseError> {
    let mut value: serde_json::Value = serde_json::from_str(text)?;
    let modules = value["modules"].as_array_mut().into_iter().flatten();
    for module in modules {
        let path = module["path"].as_str().unwrap_or_default().to_owned();
        if let Some(exports) = module
            .get_mut("exports")
            .and_then(serde_json::Value::as_array_mut)
        {
            let mut checked = Vec::with_capacity(exports.len());
            for export in exports.drain(..) {
                let tag = export["kind"] == "custom-element-definition";
                match serde_json::from_value::<JavaScriptModuleExportsItem>(export.clone()) {
                    Ok(_) => checked.push(export),
                    Err(source) if tag => {
                        return Err(ParseError::Element {
                            module: path,
                            name: export["name"].as_str().unwrap_or_default().to_owned(),
                            source,
                        });
                    }
                    Err(_) => {}
                }
            }
            *exports = checked;
        }
        let Some(decls) = module
            .get_mut("declarations")
            .and_then(serde_json::Value::as_array_mut)
        else {
            continue;
        };
        let mut checked = Vec::with_capacity(decls.len());
        for decl in decls.drain(..) {
            let element = decl["customElement"] == true || decl.get("tagName").is_some();
            let result = if !element {
                serde_json::from_value::<JavaScriptModuleDeclarationsItem>(decl.clone()).map(drop)
            } else if decl["kind"] == "mixin" {
                serde_json::from_value::<CustomElementMixinDeclaration>(decl.clone()).map(drop)
            } else {
                serde_json::from_value::<CustomElementDeclaration>(decl.clone()).map(drop)
            };
            match result {
                Ok(()) => checked.push(decl),
                Err(source) if element => {
                    return Err(ParseError::Element {
                        module: path,
                        name: decl["name"].as_str().unwrap_or_default().to_owned(),
                        source,
                    });
                }
                Err(_) => {}
            }
        }
        *decls = checked;
    }
    Ok(serde_json::from_value(value)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn manifest(decl: &str) -> String {
        format!(
            r#"{{ "schemaVersion": "1.0.0", "modules": [{{ "kind": "javascript-module",
                "path": "src/a.js", "declarations": [{decl}] }}] }}"#
        )
    }

    #[test]
    fn event_without_type_still_parses_as_an_element() {
        let pkg = parse(&manifest(
            r#"{ "kind": "class", "name": "A", "customElement": true, "tagName": "x-a",
                 "events": [{ "name": "x-show" }] }"#,
        ))
        .unwrap();
        assert!(matches!(
            pkg.modules[0].declarations[0],
            JavaScriptModuleDeclarationsItem::CustomElementDeclaration(_)
        ));
    }

    #[test]
    fn malformed_element_is_an_error_not_a_plain_class() {
        let text = manifest(
            r#"{ "kind": "class", "name": "A", "customElement": true, "tagName": "x-a",
                 "slots": [{ "description": "no name" }] }"#,
        );
        // Plain serde accepts it as a ClassDeclaration and drops the tag.
        assert!(matches!(
            serde_json::from_str::<Package>(&text).unwrap().modules[0].declarations[0],
            JavaScriptModuleDeclarationsItem::ClassDeclaration(_)
        ));
        let err = parse(&text).unwrap_err().to_string();
        assert!(
            err.starts_with("src/a.js: `A` looks like a custom element"),
            "{err}"
        );
        assert!(err.contains("missing field `name`"), "{err}");
    }

    #[test]
    fn malformed_non_element_is_dropped() {
        let pkg = parse(&manifest(
            r#"{ "kind": "class", "name": "Bundled", "superclass": { "module": "x.js" } },
               { "kind": "class", "name": "A", "customElement": true, "tagName": "x-a" }"#,
        ))
        .unwrap();
        assert_eq!(pkg.modules[0].declarations.len(), 1);
    }

    #[test]
    fn malformed_js_export_is_dropped_but_tag_export_is_an_error() {
        let with_exports = |exports: &str| {
            format!(
                r#"{{ "schemaVersion": "1.0.0", "modules": [{{ "kind": "javascript-module",
                    "path": "src/a.js", "exports": [{exports}] }}] }}"#
            )
        };
        let pkg = parse(&with_exports(
            r#"{ "kind": "js", "name": "default", "declaration": { "module": "x.js" } }"#,
        ))
        .unwrap();
        assert!(pkg.modules[0].exports.is_empty());
        let err = parse(&with_exports(
            r#"{ "kind": "custom-element-definition", "name": "x-a", "declaration": {} }"#,
        ))
        .unwrap_err();
        assert!(matches!(err, ParseError::Element { .. }), "{err}");
    }

    #[test]
    fn plain_classes_and_bad_json() {
        assert!(parse(&manifest(r#"{ "kind": "class", "name": "Util" }"#)).is_ok());
        assert!(matches!(parse("{"), Err(ParseError::Json(_))));
    }
}
