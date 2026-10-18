//! IR snapshot per fixture, built with the fixture package.json and holyfloe.config.json
//! (`src/<name>/holyfloe.config.json` when the fixture has its own, e.g. fx-events overrides).

#[path = "common/mod.rs"]
mod common;

use common::build_fixture;
use holyfloe_ir::{PropKind, TwoWayBinding, ValuePath};

const FIXTURES: [&str; 12] = [
    "fx-badge",
    "fx-button",
    "fx-card",
    "fx-deprecated",
    "fx-derived",
    "fx-dialog",
    "fx-events",
    "fx-input",
    "fx-lazy",
    "fx-list",
    "fx-mixin",
    "fx-select",
];

fn binding(prop: &str, event: &str, value: ValuePath, inferred: bool) -> TwoWayBinding {
    TwoWayBinding {
        prop: prop.into(),
        event: event.into(),
        value,
        inferred,
    }
}

#[test]
fn m2_bindings() {
    let input = build_fixture("fx-input").components.remove(0);
    assert!(
        input.form_associated,
        "static formAssociated = true (D-065)"
    );
    assert_eq!(
        input.bindings,
        [binding(
            "value",
            "change",
            ValuePath::TargetProp("value".into()),
            true
        )]
    );
    let dialog = build_fixture("fx-dialog").components.remove(0);
    assert_eq!(
        dialog.bindings,
        [binding(
            "open",
            "open-change",
            ValuePath::DetailField("open".into()),
            false
        )],
        "@model replaces the inferred Detail binding"
    );
    let select = build_fixture("fx-select").components.remove(0);
    assert_eq!(
        select.bindings,
        [binding(
            "value",
            "fx-change",
            ValuePath::DetailField("value".into()),
            false
        )],
        "config binding; the heuristic does not know fx-change"
    );
}

#[test]
fn fx_mixin_flattens_the_dependency_mixin() {
    let c = build_fixture("fx-mixin").components.remove(0);
    let props: Vec<_> = c
        .props
        .iter()
        .map(|p| (p.name.as_str(), p.attribute.as_deref(), p.reflects))
        .collect();
    assert_eq!(
        props,
        [
            ("prefix", Some("prefix"), false),
            ("focused", Some("focused"), true),
            ("ringColor", Some("ring-color"), false)
        ]
    );
    let methods: Vec<_> = c.methods.iter().map(|m| m.name.as_str()).collect();
    assert_eq!(methods, ["toggleFocus"]);
    assert_eq!(c.events[0].dom_name, "focus-change");
}

#[test]
fn ir_snapshots() {
    for name in FIXTURES {
        insta::assert_json_snapshot!(name, build_fixture(name));
    }
}

#[test]
fn fx_derived_inherits_without_duplicates() {
    let lib = build_fixture("fx-derived");
    let names: Vec<_> = lib.components.iter().map(|c| c.tag.as_str()).collect();
    assert_eq!(
        names,
        ["fx-derived"],
        "FxBase has no tag and is not a component"
    );
    let props: Vec<_> = lib.components[0]
        .props
        .iter()
        .map(|p| p.name.as_str())
        .collect();
    assert_eq!(props, ["label", "size", "loading"]);
    assert_eq!(lib.components[0].events.len(), 1);
}

#[test]
fn fx_deprecated_drops_non_public() {
    let c = build_fixture("fx-deprecated").components.remove(0);
    let props: Vec<_> = c.props.iter().map(|p| p.name.as_str()).collect();
    assert_eq!(
        props,
        ["tone", "kind"],
        "busy is protected, defaultTone static, @internal stripped"
    );
    assert_eq!(c.deprecated.as_deref(), Some("Use fx-card instead."));
}

#[test]
fn fx_list_type_text_is_verbatim() {
    let c = build_fixture("fx-list").components.remove(0);
    let render = c.props.iter().find(|p| p.name == "renderLabel").unwrap();
    // Lossy analyzer text (D-055) is kept as-is; emitters index `El['renderLabel']` instead.
    assert_eq!(
        render.ty.as_ref().unwrap().text,
        "(item: ListItem) => string | undefined"
    );
    assert!(c.props.iter().all(|p| p.kind == PropKind::Complex));
}
