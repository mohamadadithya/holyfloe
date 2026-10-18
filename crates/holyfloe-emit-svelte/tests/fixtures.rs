//! Per fixture: an insta snapshot of every file, plus byte equality with the hand-written
//! `fixtures/expected/svelte/<name>/` (the spec).

use std::fs;

use holyfloe_emit_svelte::emit;

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
        let expected = fixtures_dir().join("expected/svelte").join(name);
        let mut on_disk: Vec<_> = fs::read_dir(&expected)
            .unwrap()
            .map(|e| e.unwrap().file_name().into_string().unwrap())
            .collect();
        on_disk.sort();
        let mut emitted: Vec<_> = files.iter().map(|f| f.path.clone()).collect();
        emitted.sort();
        assert_eq!(emitted, on_disk, "svelte/{name}: file set");
        for f in &files {
            assert_eq!(
                f.contents,
                read(&expected.join(&f.path)),
                "svelte/{name}/{} differs from expected",
                f.path
            );
            insta::assert_snapshot!(format!("svelte__{name}__{}", f.path), f.contents);
        }
    }
}

/// A prop with the name of something the wrapper declares is E013, not broken output.
#[test]
fn generated_name_clashes_are_e013() {
    for (fixture, prop, what) in [
        ("fx-badge", "element", "`element`"),
        ("fx-badge", "children", "`children`"),
        ("fx-card", "header", "slot `header`"),
        ("fx-button", "onFxPress", "event `fx-press`"),
    ] {
        let mut lib = build_fixture(fixture);
        let mut p = lib.components[0].props[0].clone();
        p.name = prop.into();
        lib.components[0].props.push(p);
        let err = emit(&lib).unwrap_err();
        assert_eq!(err.code(), Some("E013"), "{fixture}.{prop}");
        assert!(err.to_string().contains(what), "{err}");
    }
}

/// Svelte reads a trailing `capture` as the capture flag, so such a name gets it twice.
#[test]
fn names_ending_in_capture_use_the_capture_form() {
    let mut lib = build_fixture("fx-events");
    lib.components[0].events[3].dom_name = "screen-capture".into();
    let files = emit(&lib).unwrap();
    let component = &files
        .iter()
        .find(|f| f.path == "FxEvents.svelte")
        .unwrap()
        .contents;
    assert!(component.contains("  onscreen-capturecapture={(event: Event) => onMYEVENT?.("));
    assert!(component.contains("`on<name>capture` listeners"));
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
    for f in emit(&lib).unwrap() {
        let line = match f.path.as_str() {
            "FxEvents.svelte" => "  import type { Change } from '@holyfloe/fixtures/types.js';\n",
            "svelte-elements.d.ts" => {
                "import type { Change } from '@holyfloe/fixtures/types.js';\n"
            }
            _ => continue,
        };
        assert!(f.contents.contains(line), "{}", f.path);
    }
    let ty = lib.components[0].events[0].event_type.as_mut().unwrap();
    ty.imports[0].name = "Snippet".into();
    assert_eq!(emit(&lib).unwrap_err().code(), Some("E012"));
}
