//! IR snapshot per M1 fixture, built with the fixture package.json and holyfloe.config.json
//! (`src/<name>/holyfloe.config.json` when the fixture has its own, e.g. fx-events overrides).

use std::fs;
use std::path::PathBuf;

use holyfloe_ir::{Config, PackageInfo, PropKind, build};

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

fn read(path: PathBuf) -> String {
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

fn build_fixture(name: &str) -> holyfloe_ir::Library {
    let dir = fixtures_dir();
    let cem = holyfloe_cem::parse(&read(
        dir.join("src").join(name).join("custom-elements.json"),
    ))
    .unwrap();
    let pkg: PackageInfo = serde_json::from_str(&read(dir.join("package.json"))).unwrap();
    let own = dir.join("src").join(name).join("holyfloe.config.json");
    let cfg_path = if own.exists() {
        own
    } else {
        dir.join("holyfloe.config.json")
    };
    let cfg: Config = serde_json::from_str(&read(cfg_path)).unwrap();
    build(&cem, &pkg, &cfg).unwrap()
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
    // Lossy analyzer text (OQ-015) is kept as-is; emitters index `El['renderLabel']` instead.
    assert_eq!(
        render.ty.as_ref().unwrap().text,
        "(item: ListItem) => string | undefined"
    );
    assert!(c.props.iter().all(|p| p.kind == PropKind::Complex));
}
