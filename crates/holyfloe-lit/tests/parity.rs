//! `just parity`: on every fixture, the IR built from the Lit source equals the IR built from
//! the committed analyzer manifest (same package.json, config and dependency manifests).

use std::fs;
use std::path::{Path, PathBuf};

use holyfloe_ir::{Config, Dependency, Library, PackageInfo, build};

fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures")
}

fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

fn library(dir: &Path, name: &str, cem: &holyfloe_cem::Package) -> Library {
    let pkg: PackageInfo = serde_json::from_str(&read(&dir.join("package.json"))).unwrap();
    let own = dir.join("src").join(name).join("holyfloe.config.json");
    let cfg_path = if own.exists() {
        own
    } else {
        dir.join("holyfloe.config.json")
    };
    let cfg: Config = serde_json::from_str(&read(&cfg_path)).unwrap();
    let deps: Vec<Dependency> = holyfloe_ir::referenced_packages(cem)
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
    build(cem, &pkg, &deps, &cfg).unwrap_or_else(|e| panic!("{name}: {e:?}"))
}

#[test]
fn lit_source_gives_the_analyzer_ir_on_every_fixture() {
    let dir = fixtures_dir();
    let mut names: Vec<String> = fs::read_dir(dir.join("src"))
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .filter(|n| !n.starts_with('_'))
        .collect();
    names.sort();
    assert_eq!(names.len(), 12);
    let mut failed = Vec::new();
    for name in &names {
        let src = dir.join("src").join(name);
        let analyzer = holyfloe_cem::parse(&read(&src.join("custom-elements.json"))).unwrap();
        let files: Vec<PathBuf> = fs::read_dir(&src)
            .unwrap()
            .map(|e| e.unwrap().path())
            .filter(|p| p.extension().is_some_and(|x| x == "ts"))
            .collect();
        let manifest = holyfloe_lit::extract(&dir, &files).unwrap();
        let lit = holyfloe_cem::parse(&manifest.to_string())
            .unwrap_or_else(|e| panic!("{name}: {e}\n{manifest:#}"));
        let (want, got) = (library(&dir, name, &analyzer), library(&dir, name, &lit));
        if want != got {
            let (w, g) = (
                serde_json::to_string_pretty(&want).unwrap(),
                serde_json::to_string_pretty(&got).unwrap(),
            );
            let diff: Vec<String> = w
                .lines()
                .zip(g.lines())
                .enumerate()
                .filter(|(_, (a, b))| a != b)
                .take(8)
                .map(|(i, (a, b))| format!("  line {i}:\n    analyzer: {a}\n    lit:      {b}"))
                .collect();
            failed.push(format!("{name}:\n{}", diff.join("\n")));
        }
    }
    assert!(failed.is_empty(), "IR differs:\n{}", failed.join("\n"));
}
