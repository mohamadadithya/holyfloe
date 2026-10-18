//! Pass 2: remove private, protected, static and `#private` members, after reading the static
//! `formAssociated` flag. Props come from fields
//! only, so `attributes` entries for removed fields (the analyzer lists protected ones) never
//! reach the IR. `@internal` has no CEM field; the analyzer strips those members (`docs/ir.md`).

use holyfloe_cem::Privacy;

use crate::class::Class;

pub(crate) fn run(classes: &mut [Class]) {
    for class in classes {
        // The only static the IR reads (D-065), inherited ones included: flatten ran first.
        class.form_associated = class.members.iter().any(|m| {
            m.is_static
                && m.name == "formAssociated"
                && m.field.as_ref().and_then(|f| f.default.as_deref()) == Some("true")
        });
        class.members.retain(|m| {
            !matches!(m.privacy, Some(Privacy::Private | Privacy::Protected))
                && !m.is_static
                && !m.name.starts_with('#')
        });
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::test_util::{build_one, element};

    #[test]
    fn only_public_instance_members_survive() {
        let mut el = element("x-a", "A");
        el["members"] = json!([
            { "kind": "field", "name": "open", "type": { "text": "boolean" } },
            { "kind": "field", "name": "busy", "privacy": "protected", "attribute": "busy" },
            { "kind": "field", "name": "secret", "privacy": "private" },
            { "kind": "field", "name": "defaultTone", "static": true },
            { "kind": "method", "name": "#onClick", "privacy": "private" },
            { "kind": "method", "name": "show" },
            { "kind": "method", "name": "create", "static": true }
        ]);
        el["attributes"] = json!([{ "name": "busy", "fieldName": "busy" }]);
        let c = build_one(el);
        let props: Vec<_> = c.props.iter().map(|p| p.name.as_str()).collect();
        let methods: Vec<_> = c.methods.iter().map(|m| m.name.as_str()).collect();
        assert_eq!(props, ["open"]);
        assert_eq!(methods, ["show"]);
        assert!(!c.form_associated);
    }

    #[test]
    fn static_form_associated_sets_the_flag() {
        let mut el = element("x-a", "A");
        el["members"] = json!([
            { "kind": "field", "name": "formAssociated", "static": true, "default": "true" }
        ]);
        let c = build_one(el);
        assert!(c.form_associated);
        assert!(c.props.is_empty());
    }
}
