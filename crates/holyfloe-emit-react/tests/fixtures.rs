//! Per fixture x mode: an insta snapshot, plus byte equality with the hand-written
//! `fixtures/expected/react{,-types}/<name>/` (the spec).

use std::fs;

use holyfloe_emit_react::{EmitError, ReactConfig, ReactMode, RefStyle, emit};

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

fn check(mode: ReactMode, expected_dir: &str) {
    let cfg = ReactConfig {
        mode,
        ..ReactConfig::default()
    };
    for name in FIXTURES {
        let files = emit(&build_fixture(name), &cfg).unwrap();
        let expected = fixtures_dir()
            .join("expected")
            .join(expected_dir)
            .join(name);
        let mut on_disk: Vec<_> = fs::read_dir(&expected)
            .unwrap()
            .map(|e| e.unwrap().file_name().into_string().unwrap())
            .collect();
        on_disk.sort();
        let mut emitted: Vec<_> = files.iter().map(|f| f.path.clone()).collect();
        emitted.sort();
        assert_eq!(emitted, on_disk, "{expected_dir}/{name}: file set");
        for f in &files {
            assert_eq!(
                f.contents,
                read(&expected.join(&f.path)),
                "{expected_dir}/{name}/{} differs from expected",
                f.path
            );
            insta::assert_snapshot!(format!("{expected_dir}__{name}__{}", f.path), f.contents);
        }
    }
}

#[test]
fn wrapper_matches_expected() {
    check(ReactMode::Wrapper, "react");
}

#[test]
fn types_matches_expected() {
    check(ReactMode::Types, "react-types");
}

/// `react.ref: "prop"` has no expected/ file yet; the snapshot is the spec until M1 step 6 type-checks it.
/// `react.ref: "prop"` (React 19 only) on fx-button: `fixtures/expected/react-ref-prop/fx-button/`.
#[test]
fn ref_prop_matches_expected() {
    let cfg = ReactConfig {
        ref_style: RefStyle::Prop,
        ..ReactConfig::default()
    };
    let files = emit(&build_fixture("fx-button"), &cfg).unwrap();
    let expected = fixtures_dir().join("expected/react-ref-prop/fx-button");
    assert_eq!(fs::read_dir(&expected).unwrap().count(), files.len());
    for f in &files {
        assert_eq!(
            f.contents,
            read(&expected.join(&f.path)),
            "react-ref-prop/fx-button/{}",
            f.path
        );
    }
    let component = files.iter().find(|f| f.path == "FxButton.tsx").unwrap();
    insta::assert_snapshot!("ref_prop__fx-button__FxButton.tsx", component.contents);
}

#[test]
fn config_parses_from_json() {
    let cfg: ReactConfig =
        serde_json::from_str(r#"{ "mode": "types", "useClient": false, "ref": "prop" }"#).unwrap();
    assert_eq!(
        cfg,
        ReactConfig {
            mode: ReactMode::Types,
            use_client: false,
            ref_style: RefStyle::Prop
        }
    );
    assert_eq!(
        serde_json::from_str::<ReactConfig>("{}").unwrap(),
        ReactConfig::default()
    );
    for unknown in [r#"{ "useclient": false }"#, r#"{ "slotHelpers": true }"#] {
        let err = serde_json::from_str::<ReactConfig>(unknown).unwrap_err();
        assert!(err.to_string().contains("unknown field"), "{err}");
    }
}

/// No M1 fixture names a type in an event detail, so add references by hand.
#[test]
fn detail_type_references_become_imports() {
    let mut lib = build_fixture("fx-events");
    let events = &mut lib.components[0].events;
    for (event, name) in [(0, "Change"), (1, "Change"), (2, "Toggle")] {
        let ty = events[event].event_type.as_mut().unwrap();
        ty.imports.push(holyfloe_ir::TypeImport {
            name: name.into(),
            specifier: "@holyfloe/fixtures/types.js".into(),
        });
    }
    let line = "import type { Change, Toggle } from '@holyfloe/fixtures/types.js';\n";
    let clashing = |name: &str, specifier: &str| {
        let mut lib = lib.clone();
        let ty = lib.components[0].events[3].event_type.as_mut().unwrap();
        ty.imports.push(holyfloe_ir::TypeImport {
            name: name.into(),
            specifier: specifier.into(),
        });
        lib
    };
    for (name, specifier, clash) in [
        (
            "Change",
            "@holyfloe/fixtures/other.js",
            "imported from both",
        ),
        ("Ref", "@holyfloe/fixtures/types.js", "reserved"),
        ("FxEventsElement", "@holyfloe/fixtures/types.js", "reserved"),
    ] {
        let err = emit(&clashing(name, specifier), &ReactConfig::default()).unwrap_err();
        assert!(err.to_string().contains(clash), "{name}: {err}");
    }
    for mode in [ReactMode::Wrapper, ReactMode::Types] {
        let cfg = ReactConfig {
            mode,
            ..ReactConfig::default()
        };
        let files = emit(&lib, &cfg).unwrap();
        let out = &files[0].contents;
        assert_eq!(out.matches(line).count(), 1, "{mode:?}:\n{out}");
    }
}

/// fx-badge covers reserved words (`for`); a prop named like a wrapper local must not shadow it.
#[test]
fn props_named_like_wrapper_locals_bind_other_names() {
    let mut lib = build_fixture("fx-badge");
    let tone = &mut lib.components[0].props[1];
    assert_eq!(tone.name, "tone");
    tone.name = "rest".into();
    tone.attribute = Some("rest".into());
    let files = emit(&lib, &ReactConfig::default()).unwrap();
    let out = &files[0].contents;
    for line in [
        "  { count, rest: rest_, label, for: for_, ...rest },\n",
        "  useProperties(ref, 'fx-badge', { count, rest: rest_, label, for: for_ });\n",
        "    rest: rest_,\n",
    ] {
        assert!(out.contains(line), "missing {line:?} in:\n{out}");
    }
}

/// A callback prop named like an event's React prop would be destructured twice.
#[test]
fn prop_named_like_an_event_prop_is_an_error() {
    let mut lib = build_fixture("fx-derived");
    lib.components[0].props[0].name = "onFxReady".into();
    let err = emit(&lib, &ReactConfig::default()).unwrap_err();
    assert!(
        matches!(&err, holyfloe_emit_react::EmitError::PropEventClash { prop, event, .. } if prop == "onFxReady" && event == "fx-ready"),
        "{err}"
    );
    assert_eq!(err.code(), Some("E013"));
    let types = ReactConfig {
        mode: ReactMode::Types,
        ..ReactConfig::default()
    };
    assert!(
        emit(&lib, &types).is_ok(),
        "types mode does not destructure"
    );
}

/// A probe event with a `CustomEvent<T>` detail, so detail imports are considered.
fn probe_event(imports: Vec<holyfloe_ir::TypeImport>) -> holyfloe_ir::Event {
    holyfloe_ir::Event {
        dom_name: "zz-probe".into(),
        base_name: "ZzProbe".into(),
        detail: Some(holyfloe_ir::TypeRef {
            text: "Probe".into(),
            imports: Vec::new(),
        }),
        event_type: Some(holyfloe_ir::TypeRef {
            text: "CustomEvent<Probe>".into(),
            imports,
        }),
        description: None,
        deprecated: None,
    }
}

/// Every identifier a generated file imports or declares as a type: `import { a, type B }`,
/// `import type { X as Y }` (binds `Y`), `export type T =`, `type T<...> =`.
fn bound_names(src: &str) -> Vec<String> {
    let ident = |s: &str| {
        s.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || c == '$'))
            .next()
            .unwrap_or_default()
            .to_owned()
    };
    let mut names = Vec::new();
    for line in src.lines().map(str::trim_start) {
        if let Some(rest) = line.strip_prefix("import ")
            && let (Some(open), Some(close)) = (rest.find('{'), rest.find('}'))
        {
            for part in rest[open + 1..close].split(',') {
                let part = part.trim().trim_start_matches("type ");
                names.push(ident(part.rsplit(" as ").next().unwrap_or(part).trim()));
            }
        } else if let Some(rest) = line
            .strip_prefix("export type ")
            .or_else(|| line.strip_prefix("type "))
        {
            names.push(ident(rest));
        }
    }
    names.retain(|n| !n.is_empty());
    names
}

/// The reserved-name lists in `lib.rs` are kept by hand; this fails when a template starts
/// binding a name they miss. A type-only import may share a name with the component `const`
/// (TS allows it), so component names are not checked.
#[test]
fn reserved_names_cover_every_name_the_templates_bind() {
    let modes = [
        (ReactMode::Wrapper, RefStyle::ForwardRef),
        (ReactMode::Wrapper, RefStyle::Prop),
        (ReactMode::Types, RefStyle::ForwardRef),
    ];
    for name in FIXTURES {
        let mut lib = build_fixture(name);
        lib.components[0].events.push(probe_event(Vec::new()));
        for (mode, ref_style) in modes {
            let cfg = ReactConfig {
                mode,
                ref_style,
                ..ReactConfig::default()
            };
            let files = emit(&lib, &cfg).unwrap();
            let bound = bound_names(&files[0].contents);
            assert!(bound.len() >= 4, "{name}: parsed too little: {bound:?}");
            for taken in &bound {
                let mut lib = lib.clone();
                let events = &mut lib.components[0].events;
                *events.last_mut().unwrap() = probe_event(vec![holyfloe_ir::TypeImport {
                    name: taken.clone(),
                    specifier: "@holyfloe/fixtures/types.js".into(),
                }]);
                let err = emit(&lib, &cfg).err();
                assert!(
                    matches!(err, Some(EmitError::DetailImportClash { .. })),
                    "{name} {mode:?} {ref_style:?}: `{taken}` is bound by the template but not reserved"
                );
                assert_eq!(err.and_then(|e| e.code()), Some("E012"));
            }
        }
    }
}

/// Several components: one `index.ts` and one `react-jsx.d.ts` list them all, `TypedEvent`
/// once. Every fixture has one component, so expected/ never covers this.
#[test]
fn several_components_share_index_and_types_file() {
    let mut lib = build_fixture("fx-button");
    for name in ["fx-card", "fx-events"] {
        lib.components.extend(build_fixture(name).components);
    }
    let files = emit(&lib, &ReactConfig::default()).unwrap();
    let paths: Vec<_> = files.iter().map(|f| f.path.as_str()).collect();
    assert_eq!(
        paths,
        [
            "FxButton.tsx",
            "FxCard.tsx",
            "FxEvents.tsx",
            "index.ts",
            "react-utils.ts"
        ]
    );
    insta::assert_snapshot!("multi__index.ts", files[3].contents);
    let types = ReactConfig {
        mode: ReactMode::Types,
        ..ReactConfig::default()
    };
    let files = emit(&lib, &types).unwrap();
    let d_ts = &files[0].contents;
    assert_eq!(d_ts.matches("type TypedEvent<").count(), 1);
    insta::assert_snapshot!("multi__react-jsx.d.ts", d_ts);
}

/// No props, events, slots, parts, CSS properties or docs: the template's empty branches.
#[test]
fn bare_component() {
    let mut lib = build_fixture("fx-button");
    let c = &mut lib.components[0];
    c.props.clear();
    c.events.clear();
    c.slots.clear();
    (c.summary, c.description) = (None, None);
    for mode in [ReactMode::Wrapper, ReactMode::Types] {
        let cfg = ReactConfig {
            mode,
            ..ReactConfig::default()
        };
        let files = emit(&lib, &cfg).unwrap();
        insta::assert_snapshot!(format!("bare__{mode:?}"), files[0].contents);
    }
}

/// Doc comment edge cases no fixture has.
#[test]
fn doc_edge_cases() {
    let mut lib = build_fixture("fx-derived");
    let c = &mut lib.components[0];
    c.deprecated = Some(String::new());
    c.slots[0].description = None;
    c.description = Some("Ends */ here.".into());
    let e = &mut c.events[0];
    e.description = Some("Line one.\nLine two.".into());
    e.deprecated = Some("Use `fx-done`.".into());
    c.events.push(holyfloe_ir::Event {
        description: Some("Has */ inside.".into()),
        deprecated: Some(String::new()),
        dom_name: "fx-other".into(),
        base_name: "FxOther".into(),
        ..c.events[0].clone()
    });
    let files = emit(&lib, &ReactConfig::default()).unwrap();
    let out = &files[0].contents;
    for expected in [
        " * Ends *\\/ here.\n",
        " * - default\n",
        " * @deprecated\n */\n",
        "  /**\n   * Line one.\n   * Line two.\n   *\n   * @deprecated Use `fx-done`.\n   */\n  onFxReady?:",
        "  /**\n   * Has *\\/ inside.\n   *\n   * @deprecated\n   */\n  onFxOther?:",
    ] {
        assert!(out.contains(expected), "missing {expected:?} in:\n{out}");
    }
}

#[test]
fn use_client_false_drops_the_directive() {
    let cfg = ReactConfig {
        use_client: false,
        ..ReactConfig::default()
    };
    let files = emit(&build_fixture("fx-button"), &cfg).unwrap();
    assert!(
        files[0]
            .contents
            .starts_with("// Generated by holyfloe. Do not edit.\n")
    );
    assert!(!files[0].contents.contains("use client"));
}
