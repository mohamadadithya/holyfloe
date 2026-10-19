//! Loads a fixture the way the CLI does. Shared by the IR and emitter tests through
//! `#[path = ".../holyfloe-ir/tests/common/mod.rs"] mod common;`.

use std::fs;
use std::path::{Path, PathBuf};

use holyfloe_ir::{Config, Dependency, Library, PackageInfo, build};

pub fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures")
}

pub fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

/// The fixture's own `holyfloe.config.json` if it has one (fx-events, fx-select), else the
/// shared one. Dependency manifests come from `fixtures/node_modules` (fx-mixin).
pub fn build_fixture(name: &str) -> Library {
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
    let deps: Vec<Dependency> = holyfloe_ir::referenced_packages(&cem)
        .into_iter()
        .filter_map(|name| {
            let root = dir.join("node_modules").join(&name);
            let pkg: serde_json::Value =
                serde_json::from_str(&fs::read_to_string(root.join("package.json")).ok()?).ok()?;
            let manifest = root.join(pkg["customElements"].as_str()?);
            let cem = holyfloe_cem::parse(&read(&manifest)).unwrap();
            Some(Dependency { name, cem })
        })
        .collect();
    build(&cem, &pkg, &deps, &cfg).unwrap()
}
