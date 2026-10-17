//! Every committed fixture manifest must survive deserialize -> serialize unchanged.

use std::path::Path;

#[test]
fn fixture_manifests_round_trip() {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/src");
    let mut checked = 0;
    for entry in std::fs::read_dir(&src).unwrap() {
        let path = entry.unwrap().path().join("custom-elements.json");
        if !path.exists() {
            continue;
        }
        let text = std::fs::read_to_string(&path).unwrap();
        let original: serde_json::Value = serde_json::from_str(&text).unwrap();
        let parsed: holyfloe_cem::Package =
            serde_json::from_str(&text).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        let again = serde_json::to_value(&parsed).unwrap();
        assert_eq!(original, again, "{} did not round-trip", path.display());
        checked += 1;
    }
    assert!(
        checked > 0,
        "no fixture manifests found under {}",
        src.display()
    );
}
