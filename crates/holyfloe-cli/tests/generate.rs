//! Runs the built `holyfloe` binary on fixtures.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures")
}

fn generate(name: &str, out: &PathBuf, config: &str) -> Output {
    let dir = fixtures();
    Command::new(env!("CARGO_BIN_EXE_holyfloe"))
        .args(["generate", "--frameworks", "react", "--cem"])
        .arg(dir.join("src").join(name).join("custom-elements.json"))
        .arg("--out")
        .arg(out)
        .arg("--config")
        .arg(dir.join(config))
        .output()
        .unwrap()
}

fn temp_out(name: &str) -> PathBuf {
    let out = std::env::temp_dir().join(format!("holyfloe-cli-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&out);
    out
}

#[test]
fn fx_button_matches_expected() {
    let out = temp_out("button");
    let run = generate("fx-button", &out, "holyfloe.config.json");
    assert!(
        run.status.success(),
        "{}",
        String::from_utf8_lossy(&run.stderr)
    );
    let expected = fixtures().join("expected/react/fx-button");
    let mut names: Vec<_> = fs::read_dir(&expected)
        .unwrap()
        .map(|e| e.unwrap().file_name())
        .collect();
    names.sort();
    let mut written: Vec<_> = fs::read_dir(out.join("react"))
        .unwrap()
        .map(|e| e.unwrap().file_name())
        .collect();
    written.sort();
    assert_eq!(written, names);
    for n in names {
        assert_eq!(
            fs::read_to_string(out.join("react").join(&n)).unwrap(),
            fs::read_to_string(expected.join(&n)).unwrap(),
            "{n:?}"
        );
    }
    let again = generate("fx-button", &out, "holyfloe.config.json");
    assert_eq!(
        String::from_utf8_lossy(&again.stdout).split(" in ").next(),
        Some("react: 0 written, 3 unchanged")
    );
    fs::remove_dir_all(&out).unwrap();
}

/// Without its own config, fx-events' three `my-change` spellings clash.
#[test]
fn ir_error_prints_code_and_hint_and_exits_1() {
    let out = temp_out("events");
    let run = generate("fx-events", &out, "holyfloe.config.json");
    assert_eq!(run.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&run.stderr);
    let mut lines = stderr.lines();
    assert!(
        lines
            .next()
            .unwrap()
            .starts_with("error[E006]: events `my-change`"),
        "{stderr}"
    );
    assert!(lines.next().unwrap().starts_with("  hint: "), "{stderr}");
    assert!(!out.exists(), "nothing is written on error");
}

/// Emitter errors print like IR errors: a callback prop `onXReady` next to event `x-ready`.
#[test]
fn emit_error_prints_code_and_hint() {
    let dir = temp_out("clash");
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("package.json"), r#"{ "name": "pkg" }"#).unwrap();
    let cem = r#"{ "schemaVersion": "1.0.0", "modules": [{ "kind": "javascript-module", "path": "x.js",
        "declarations": [{ "kind": "class", "name": "XEl", "tagName": "x-el", "customElement": true,
          "members": [{ "kind": "field", "name": "onXReady" }], "events": [{ "name": "x-ready" }] }] }] }"#;
    fs::write(dir.join("custom-elements.json"), cem).unwrap();
    let run = Command::new(env!("CARGO_BIN_EXE_holyfloe"))
        .args(["generate", "--frameworks", "react", "--cem"])
        .arg(dir.join("custom-elements.json"))
        .arg("--out")
        .arg(dir.join("out"))
        .output()
        .unwrap();
    assert_eq!(run.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&run.stderr);
    assert!(
        stderr.starts_with("error[E013]: prop `onXReady` of `x-el`"),
        "{stderr}"
    );
    assert!(
        stderr.contains("\n  hint: drop the prop with `overrides.x-el.excludeProps`, or, when the other name is an event prop, rename the event with `overrides.x-el.events`"),
        "{stderr}"
    );
    fs::remove_dir_all(&dir).unwrap();
}

/// Frameworks are all emitted before any is written: Svelte's `element` clash (D-064) must not
/// leave React output behind.
#[test]
fn emit_error_in_one_framework_writes_nothing() {
    let dir = temp_out("partial");
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("package.json"), r#"{ "name": "pkg" }"#).unwrap();
    let cem = r#"{ "schemaVersion": "1.0.0", "modules": [{ "kind": "javascript-module", "path": "x.js",
        "declarations": [{ "kind": "class", "name": "XEl", "tagName": "x-el", "customElement": true,
          "members": [{ "kind": "field", "name": "element" }] }] }] }"#;
    fs::write(dir.join("custom-elements.json"), cem).unwrap();
    let run = Command::new(env!("CARGO_BIN_EXE_holyfloe"))
        .args(["generate", "--frameworks", "react,svelte", "--cem"])
        .arg(dir.join("custom-elements.json"))
        .arg("--out")
        .arg(dir.join("out"))
        .output()
        .unwrap();
    assert_eq!(run.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&run.stderr);
    assert!(stderr.starts_with("error[E013]: "), "{stderr}");
    assert!(!dir.join("out").exists(), "react output was written");
    fs::remove_dir_all(&dir).unwrap();
}

/// A nameless `dist/package.json` (`{"type": "module"}`) is a build marker, not the package.
#[test]
fn skips_nameless_package_json() {
    let dir = temp_out("nameless");
    fs::create_dir_all(dir.join("dist")).unwrap();
    fs::write(
        dir.join("package.json"),
        r#"{ "name": "pkg", "exports": { "./*": "./dist/*" } }"#,
    )
    .unwrap();
    fs::write(dir.join("dist/package.json"), r#"{ "type": "module" }"#).unwrap();
    let cem = r#"{ "schemaVersion": "1.0.0", "modules": [{ "kind": "javascript-module", "path": "dist/x.js",
        "declarations": [{ "kind": "class", "name": "XEl", "tagName": "x-el", "customElement": true }] }] }"#;
    fs::write(dir.join("dist/custom-elements.json"), cem).unwrap();
    let run = Command::new(env!("CARGO_BIN_EXE_holyfloe"))
        .args(["generate", "--frameworks", "react", "--cem"])
        .arg(dir.join("dist/custom-elements.json"))
        .arg("--out")
        .arg(dir.join("out"))
        .output()
        .unwrap();
    assert!(
        run.status.success(),
        "{}",
        String::from_utf8_lossy(&run.stderr)
    );
    let el = fs::read_to_string(dir.join("out/react/XEl.tsx")).unwrap();
    assert!(el.contains("import 'pkg/x.js';"), "{el}");
    fs::remove_dir_all(&dir).unwrap();
}

/// `--lit` reads the Lit source instead of the manifest and writes the same wrappers.
#[test]
fn lit_input_matches_expected() {
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures");
    let out = temp_out("lit");
    let run = Command::new(env!("CARGO_BIN_EXE_holyfloe"))
        .args(["generate", "--frameworks", "react,vue,svelte", "--lit"])
        .arg(fixtures.join("src/fx-dialog"))
        .arg("--config")
        .arg(fixtures.join("holyfloe.config.json"))
        .arg("--out")
        .arg(&out)
        .output()
        .unwrap();
    assert!(
        run.status.success(),
        "{}",
        String::from_utf8_lossy(&run.stderr)
    );
    for fw in ["react", "vue", "svelte"] {
        let expected = fixtures.join("expected").join(fw).join("fx-dialog");
        for entry in fs::read_dir(&expected).unwrap() {
            let name = entry.unwrap().file_name();
            assert_eq!(
                fs::read_to_string(out.join(fw).join(&name)).unwrap(),
                fs::read_to_string(expected.join(&name)).unwrap(),
                "{fw}/{}",
                name.to_string_lossy()
            );
        }
    }
    fs::remove_dir_all(&out).unwrap();
}
