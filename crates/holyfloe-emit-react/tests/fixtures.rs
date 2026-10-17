//! Per M1 fixture x mode: an insta snapshot, plus byte equality with the hand-written
//! `fixtures/expected/react{,-types}/<name>/` (the spec).

use std::fs;
use std::path::{Path, PathBuf};

use holyfloe_emit_react::{ReactConfig, ReactMode, RefStyle, emit};
use holyfloe_ir::{Config, Library, PackageInfo, build};

const FIXTURES: [&str; 7] = [
    "fx-badge",
    "fx-button",
    "fx-card",
    "fx-deprecated",
    "fx-derived",
    "fx-events",
    "fx-list",
];

fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures")
}

fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

fn build_fixture(name: &str) -> Library {
    let dir = fixtures_dir();
    let src = dir.join("src").join(name);
    let cem = holyfloe_cem::parse(&read(&src.join("custom-elements.json"))).unwrap();
    let pkg: PackageInfo = serde_json::from_str(&read(&dir.join("package.json"))).unwrap();
    let own = src.join("holyfloe.config.json");
    let cfg_path = if own.exists() {
        own
    } else {
        dir.join("holyfloe.config.json")
    };
    let cfg: Config = serde_json::from_str(&read(&cfg_path)).unwrap();
    build(&cem, &pkg, &cfg).unwrap()
}

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
#[test]
fn ref_prop_snapshot() {
    let cfg = ReactConfig {
        ref_style: RefStyle::Prop,
        ..ReactConfig::default()
    };
    let files = emit(&build_fixture("fx-button"), &cfg).unwrap();
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
