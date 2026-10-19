//! Per fixture: an insta snapshot of every file, plus byte equality with the hand-written
//! `fixtures/expected/vue/<name>/` (the spec).

use std::fs;

use holyfloe_emit_vue::emit;

#[path = "../../holyfloe-ir/tests/common/mod.rs"]
mod common;
use common::{build_fixture, fixtures_dir, read};

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

#[test]
fn matches_expected() {
    for name in FIXTURES {
        let files = emit(&build_fixture(name)).unwrap();
        let expected = fixtures_dir().join("expected/vue").join(name);
        let mut on_disk: Vec<_> = fs::read_dir(&expected)
            .unwrap()
            .map(|e| e.unwrap().file_name().into_string().unwrap())
            .collect();
        on_disk.sort();
        let mut emitted: Vec<_> = files.iter().map(|f| f.path.clone()).collect();
        emitted.sort();
        assert_eq!(emitted, on_disk, "vue/{name}: file set");
        for f in &files {
            assert_eq!(
                f.contents,
                read(&expected.join(&f.path)),
                "vue/{name}/{} differs from expected",
                f.path
            );
            insta::assert_snapshot!(format!("vue__{name}__{}", f.path), f.contents);
        }
    }
}

/// A prop with the name of something the wrapper generates is E013, not broken output.
#[test]
fn generated_name_clashes_are_e013() {
    let clash = |name: &str, prop: &str| {
        let mut lib = build_fixture(name);
        let mut p = lib.components[0].props[0].clone();
        p.name = prop.into();
        lib.components[0].props.push(p);
        emit(&lib).unwrap_err()
    };
    for (fixture, prop, what) in [
        ("fx-dialog", "show", "forwarded method `show`"),
        ("fx-input", "modelValue", "`modelValue`"),
        ("fx-button", "onFxPress", "event `fx-press`"),
        ("fx-badge", "element", "`element`"),
    ] {
        let err = clash(fixture, prop);
        assert_eq!(err.code(), Some("E013"), "{fixture}.{prop}");
        assert!(err.to_string().contains(what), "{err}");
    }
}

/// No fixture fires an event Vue would misread as a listener option, so rename one.
#[test]
fn once_passive_capture_events_attach_after_mount() {
    let mut lib = build_fixture("fx-events");
    lib.components[0].events[3].dom_name = "zoomCapture".into();
    let files = emit(&lib).unwrap();
    let component = &files
        .iter()
        .find(|f| f.path == "FxEvents.ts")
        .unwrap()
        .contents;
    assert!(!component.contains("'on:zoomCapture'"));
    assert!(component.contains(
        "this.element?.addEventListener('zoomCapture', (event: Event) => this.$emit('MYEVENT',"
    ));
    insta::assert_snapshot!("late_event__FxEvents.ts", component);
}

/// No fixture names a type in an event detail, so add one by hand.
#[test]
fn detail_type_references_become_imports() {
    let mut lib = build_fixture("fx-events");
    let ty = lib.components[0].events[0].event_type.as_mut().unwrap();
    ty.imports.push(holyfloe_ir::TypeImport {
        name: "Change".into(),
        specifier: "@holyfloe/fixtures/types.js".into(),
    });
    let line = "import type { Change } from '@holyfloe/fixtures/types.js';\n";
    for f in emit(&lib).unwrap() {
        if f.path == "FxEvents.ts" || f.path == "global-components.d.ts" {
            assert!(f.contents.contains(line), "{}", f.path);
        }
    }
    let ty = lib.components[0].events[0].event_type.as_mut().unwrap();
    ty.imports[0].name = "TypedEvent".into();
    assert_eq!(emit(&lib).unwrap_err().code(), Some("E012"));
}

/// Shoelace shape: slot names that are not identifiers get camelCase locals (D-081).
#[test]
fn hyphenated_slot_names_get_identifier_locals() {
    let mut lib = build_fixture("fx-card");
    lib.components[0].slots[1].name = "help-text".into();
    let files = emit(&lib).unwrap();
    let card = &files
        .iter()
        .find(|f| f.path == "FxCard.ts")
        .unwrap()
        .contents;
    assert!(
        card.contains("    const { default: body, 'help-text': helpText, footer } = this.$slots;"),
        "{card}"
    );
    assert!(card.contains("        helpText && h('span', { slot: 'help-text', style: 'display: contents' }, helpText()),"));
    assert!(card.contains("    'help-text'?: () => VNode[];"));
    insta::assert_snapshot!("renamed_slots__FxCard.ts", card);
}
