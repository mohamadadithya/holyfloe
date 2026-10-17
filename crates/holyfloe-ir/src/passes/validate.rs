//! Pass 6: reject what no emitter can render.

use std::collections::{BTreeMap, HashSet};

use crate::{Component, IrError, Library};

pub(crate) fn run(lib: &Library) -> Result<(), IrError> {
    let mut tags = HashSet::new();
    for c in &lib.components {
        let err = |code, msg: String, hint: String| {
            IrError::new(code, msg, Some(&c.tag), Some(&c.module.path), hint)
        };
        if !tags.insert(c.tag.as_str()) {
            return Err(err(
                "E004",
                format!("tag `{}` is defined more than once", c.tag),
                "register each tag from exactly one class".to_owned(),
            ));
        }
        let mut props = HashSet::new();
        if let Some(p) = c.props.iter().find(|p| !props.insert(p.name.as_str())) {
            return Err(err(
                "E005",
                format!("prop `{}` is declared more than once", p.name),
                "remove the duplicate field from the element class".to_owned(),
            ));
        }
        if c.events.iter().any(|e| e.dom_name.is_empty()) {
            return Err(err(
                "E007",
                "event with an empty name".to_owned(),
                "give the event a name in its `@fires` tag".to_owned(),
            ));
        }
        if !is_identifier(&c.name) {
            return Err(err(
                "E008",
                format!("tag `{}` gives `{}`, which is not a valid identifier", c.tag, c.name),
                "use a tag of letters, digits and `-`; component name overrides are not supported yet".to_owned(),
            ));
        }
        if let Some(e) = c.events.iter().find(|e| !is_identifier(&e.base_name)) {
            return Err(err(
                "E008",
                format!(
                    "event `{}` gives the prop name `{}`, which is not a valid identifier",
                    e.dom_name, e.base_name
                ),
                format!(
                    "set `overrides.{}.events.{}: \"<PropName>\"` in holyfloe.config.json",
                    c.tag, e.dom_name
                ),
            ));
        }
        if let Some(clash) = event_clash(c) {
            return Err(err(
                "E006",
                format!(
                    "events {} all map to the prop name `{}`",
                    clash
                        .iter()
                        .map(|n| format!("`{n}`"))
                        .collect::<Vec<_>>()
                        .join(", "),
                    c.events
                        .iter()
                        .find(|e| e.dom_name == clash[0])
                        .map_or("", |e| &e.base_name),
                ),
                format!(
                    "rename all but one in holyfloe.config.json: `overrides.{}.events.<dom-name>: \"<PropName>\"`",
                    c.tag
                ),
            ));
        }
    }
    Ok(())
}

/// Base names may start with a digit: every emitter adds a prefix (React: `on`).
fn is_identifier(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '$')
}

/// DOM names of the first group of events sharing a base name.
fn event_clash(c: &Component) -> Option<Vec<&str>> {
    let mut by_base: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for e in &c.events {
        by_base.entry(&e.base_name).or_default().push(&e.dom_name);
    }
    by_base.into_values().find(|names| names.len() > 1)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::test_util::{build_json, cem_one, element};

    fn events(names: &[&str]) -> serde_json::Value {
        let mut el = element("x-a", "XA");
        el["events"] = names
            .iter()
            .map(|n| json!({ "name": n, "type": { "text": "CustomEvent" } }))
            .collect();
        cem_one(el)
    }

    #[test]
    fn event_collision_is_e006() {
        let err = build_json(events(&["my-change", "myChange", "MyChange", "other"])).unwrap_err();
        assert_eq!(err.code, "E006");
        assert_eq!(
            err.message,
            "events `my-change`, `myChange`, `MyChange` all map to the prop name `MyChange`"
        );
        assert!(err.hint.contains("overrides.x-a.events"));
    }

    #[test]
    fn non_identifier_names_are_e008() {
        let err = build_json(events(&["item:select"])).unwrap_err();
        assert_eq!(err.code, "E008");
        assert!(err.hint.contains("overrides.x-a.events.item:select"));
        assert_eq!(
            build_json(cem_one(element("x-a.v2", "A")))
                .unwrap_err()
                .code,
            "E008"
        );
        assert!(build_json(events(&["2fa-done", "$x_y"])).is_ok());
    }

    #[test]
    fn empty_event_name_is_e007() {
        assert_eq!(build_json(events(&[""])).unwrap_err().code, "E007");
    }

    #[test]
    fn duplicate_tag_is_e004() {
        let cem = json!({
            "schemaVersion": "1.0.0",
            "modules": [{ "kind": "javascript-module", "path": "src/a.ts",
                "declarations": [element("x-a", "A"), element("x-a", "B")] }]
        });
        assert_eq!(build_json(cem).unwrap_err().code, "E004");
    }

    #[test]
    fn duplicate_prop_is_e005() {
        let mut el = element("x-a", "XA");
        el["members"] = json!([
            { "kind": "field", "name": "value" },
            { "kind": "field", "name": "value" }
        ]);
        assert_eq!(build_json(cem_one(el)).unwrap_err().code, "E005");
    }
}
