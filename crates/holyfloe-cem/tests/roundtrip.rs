//! Every committed fixture manifest must survive deserialize -> serialize unchanged.

use holyfloe_cem::JavaScriptModuleDeclarationsItem;
use std::path::Path;

#[test]
fn fixture_manifests_round_trip() {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/src");
    let mut checked = 0;
    for entry in std::fs::read_dir(&src).unwrap() {
        let path = entry.unwrap().path().join("custom-elements.json");
        assert!(
            path.exists(),
            "{} is missing; run `just gen-fixtures-cem`",
            path.display()
        );
        let text = std::fs::read_to_string(&path).unwrap();
        let mut original: serde_json::Value = serde_json::from_str(&text).unwrap();
        drop_empty_arrays(&mut original);
        let parsed: holyfloe_cem::Package =
            serde_json::from_str(&text).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        let again = serde_json::to_value(&parsed).unwrap();
        assert_eq!(original, again, "{} did not round-trip", path.display());

        // Equal JSON alone does not prove the custom-element variant was chosen (D-046).
        let raw_tags: Vec<&str> = original["modules"]
            .as_array()
            .unwrap()
            .iter()
            .flat_map(|m| m["declarations"].as_array().into_iter().flatten())
            .filter_map(|d| d["tagName"].as_str())
            .collect();
        let parsed_tags: Vec<&str> = parsed
            .modules
            .iter()
            .flat_map(|m| &m.declarations)
            .filter_map(|d| match d {
                JavaScriptModuleDeclarationsItem::CustomElementDeclaration(c) => {
                    c.tag_name.as_deref()
                }
                _ => None,
            })
            .collect();
        assert_eq!(
            raw_tags,
            parsed_tags,
            "{}: tags not parsed as custom elements",
            path.display()
        );
        checked += 1;
    }
    assert!(
        checked > 0,
        "no fixture manifests found under {}",
        src.display()
    );
}

/// Optional arrays: `[]` and absent mean the same, and the generated types omit empty vecs.
fn drop_empty_arrays(v: &mut serde_json::Value) {
    match v {
        serde_json::Value::Object(map) => {
            map.retain(|_, x| !matches!(x, serde_json::Value::Array(a) if a.is_empty()));
            map.values_mut().for_each(drop_empty_arrays);
        }
        serde_json::Value::Array(items) => items.iter_mut().for_each(drop_empty_arrays),
        _ => {}
    }
}
