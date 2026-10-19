//! Constructs the fixtures do not cover. Each test writes a small package and checks the
//! manifest `holyfloe_lit::extract` builds for it.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{Value, json};

/// A temp package `name` with `files` (path, source); returns its root.
fn package(name: &str, files: &[(&str, &str)]) -> PathBuf {
    let root = std::env::temp_dir().join(format!("holyfloe-lit-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    for (path, src) in files {
        let p = root.join(path);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(p, src).unwrap();
    }
    fs::write(root.join("package.json"), r#"{ "name": "pkg" }"#).unwrap();
    root
}

fn extract(root: &Path, files: &[&str]) -> Value {
    let files: Vec<PathBuf> = files.iter().map(|f| root.join(f)).collect();
    let manifest = holyfloe_lit::extract(root, &files).unwrap();
    // Whatever the extractor writes must pass the strict manifest parse (D-049).
    holyfloe_cem::parse(&manifest.to_string()).unwrap_or_else(|e| panic!("{e}\n{manifest:#}"));
    manifest
}

fn module<'v>(m: &'v Value, path: &str) -> &'v Value {
    m["modules"]
        .as_array()
        .unwrap()
        .iter()
        .find(|x| x["path"] == path)
        .unwrap_or_else(|| panic!("no module {path} in {m:#}"))
}

fn decl<'v>(module: &'v Value, name: &str) -> &'v Value {
    module["declarations"]
        .as_array()
        .unwrap()
        .iter()
        .find(|d| d["name"] == name)
        .unwrap_or_else(|| panic!("no declaration {name} in {module:#}"))
}

fn member<'v>(decl: &'v Value, name: &str) -> &'v Value {
    decl["members"]
        .as_array()
        .unwrap()
        .iter()
        .find(|m| m["name"] == name)
        .unwrap_or_else(|| panic!("no member {name} in {decl:#}"))
}

fn names(decl: &Value, key: &str) -> Vec<String> {
    decl[key]
        .as_array()
        .map(|a| {
            a.iter()
                .map(|m| m["name"].as_str().unwrap().to_owned())
                .collect()
        })
        .unwrap_or_default()
}

#[test]
fn static_properties_getters_and_skipped_members() {
    let root = package(
        "members",
        &[(
            "x-el.js",
            r#"
import { LitElement } from 'lit';
import { state, query } from 'lit/decorators.js';
export class XEl extends LitElement {
  static properties = {
    mode: { type: String, reflect: true },
    count: { type: Number, attribute: 'item-count' },
    open: { type: Boolean },
    busy: { state: true },
  };
  @state() internal = 1;
  @query('input') input;
  static styles = [];
  constructor() {
    super();
    this.mode = 'auto';
    this.count = 0;
    this.open = false;
  }
  /** The size. */
  get size() { return 1; }
  /** @type {string} */
  get label() { return this._label; }
  set label(v) { this._label = v; }
  /** @internal */
  hidden = 1;
  render() {}
  /** Focuses the input. */
  focus() {}
}
customElements.define('x-el', XEl);
"#,
        )],
    );
    let m = extract(&root, &["x-el.js"]);
    let el = decl(module(&m, "x-el.js"), "XEl");
    assert_eq!(
        names(el, "members"),
        ["size", "label", "focus", "mode", "count", "open"]
    );
    assert_eq!(member(el, "mode")["attribute"], "mode");
    assert_eq!(member(el, "mode")["reflects"], true);
    assert_eq!(member(el, "mode")["default"], "'auto'");
    assert_eq!(member(el, "mode")["type"]["text"], "string");
    assert_eq!(member(el, "count")["attribute"], "item-count");
    assert_eq!(member(el, "open")["type"]["text"], "boolean");
    assert_eq!(member(el, "size")["readonly"], true);
    assert!(
        member(el, "label").get("readonly").is_none(),
        "a getter with a setter is writable"
    );
    assert_eq!(member(el, "focus")["kind"], "method");
    let exports = module(&m, "x-el.js")["exports"].as_array().unwrap();
    assert!(exports.contains(&json!({
        "kind": "custom-element-definition", "name": "x-el",
        "declaration": { "name": "XEl", "module": "x-el.js" }
    })));
}

#[test]
fn decorators_lowercase_attributes_and_skip_private_members() {
    let root = package(
        "decorators",
        &[(
            "src/x-el.ts",
            r#"
import { LitElement } from 'lit';
import { customElement, property } from 'lit/decorators.js';
@customElement('x-el')
export class XEl extends LitElement {
  @property() ringColor = 'red';
  @property({ attribute: false }) items: string[] = [];
  @property({ type: Boolean }) declare compact: boolean;
  @property() private secret = '';
  protected guarded = 1;
  #hidden = 2;
}
"#,
        )],
    );
    let m = extract(&root, &["src/x-el.ts"]);
    let el = decl(module(&m, "src/x-el.ts"), "XEl");
    assert_eq!(names(el, "members"), ["ringColor", "items", "compact"]);
    assert_eq!(member(el, "ringColor")["attribute"], "ringcolor");
    assert!(member(el, "items").get("attribute").is_none());
    assert_eq!(member(el, "items")["type"]["text"], "string[]");
    assert_eq!(el["tagName"], "x-el");
    assert_eq!(
        el["superclass"],
        json!({ "name": "LitElement", "package": "lit" })
    );
}

#[test]
fn events_from_generics_with_type_references() {
    let root = package(
        "events",
        &[
            ("src/types.ts", "export interface Detail { id: string }\n"),
            (
                "src/x-el.ts",
                r#"
import { LitElement } from 'lit';
import type { Detail } from './types.js';
interface Local { n: number }
/**
 * @fires {CustomEvent<Local>} x-local - Documented.
 * @event x-docs-only
 */
export class XEl extends LitElement {
  go() {
    this.dispatchEvent(new CustomEvent<Detail>('x-pick', { detail: { id: 'a' } }));
    this.dispatchEvent(new CustomEvent('x-local'));
    this.dispatchEvent(new Event('x-plain'));
  }
}
"#,
            ),
        ],
    );
    let m = extract(&root, &["src/x-el.ts"]);
    let events = decl(module(&m, "src/x-el.ts"), "XEl")["events"].clone();
    assert_eq!(
        events,
        json!([
            { "name": "x-local", "description": "Documented.",
              "type": { "text": "CustomEvent<Local>", "references": [{ "name": "Local", "module": "src/x-el.ts" }] } },
            { "name": "x-docs-only" },
            { "name": "x-pick",
              "type": { "text": "CustomEvent<Detail>", "references": [{ "name": "Detail", "module": "src/types.ts" }] } },
            { "name": "x-plain", "type": { "text": "Event", "references": [] } },
        ])
    );
}

#[test]
fn mixins_exports_and_bases_outside_the_scanned_files() {
    let root = package(
        "mixins",
        &[
            (
                "src/mixins.ts",
                r#"
import { property } from 'lit/decorators.js';
export const Sized = (base) => class extends base {
  @property() size = 'md';
};
export function Labelled(base) {
  class L extends base {
    /** The label. */
    @property() label = '';
  }
  return L;
}
"#,
            ),
            (
                "src/base.ts",
                "import { LitElement } from 'lit';\nexport class Base extends LitElement {}\n",
            ),
            (
                "src/x-el.ts",
                r#"
import { Sized, Labelled } from './mixins.js';
import { Base } from './base.js';
import { Remote } from '@other/pkg/remote.js';
export class XEl extends Sized(Labelled(Base)) {}
export class YEl extends Remote {}
export { XEl as Renamed };
export { Base as ReBase } from './base.js';
"#,
            ),
        ],
    );
    let m = extract(&root, &["src/x-el.ts"]);
    let x = module(&m, "src/x-el.ts");
    let el = decl(x, "XEl");
    assert_eq!(
        el["mixins"],
        json!([
            { "name": "Sized", "module": "src/mixins.ts" },
            { "name": "Labelled", "module": "src/mixins.ts" },
        ])
    );
    assert_eq!(
        el["superclass"],
        json!({ "name": "Base", "module": "src/base.ts" })
    );
    assert_eq!(
        decl(x, "YEl")["superclass"],
        json!({ "name": "Remote", "package": "@other/pkg/remote.js" })
    );
    // Followed even though only x-el.ts was passed.
    let mixins = module(&m, "src/mixins.ts");
    assert_eq!(decl(mixins, "Sized")["kind"], "mixin");
    assert_eq!(names(decl(mixins, "Sized"), "members"), ["size"]);
    assert_eq!(names(decl(mixins, "Labelled"), "members"), ["label"]);
    module(&m, "src/base.ts");
    let exports = x["exports"].as_array().unwrap();
    assert!(exports.contains(&json!({ "kind": "js", "name": "Renamed", "declaration": { "name": "XEl", "module": "src/x-el.ts" } })));
    assert!(exports.contains(&json!({ "kind": "js", "name": "ReBase", "declaration": { "name": "Base", "module": "src/base.ts" } })));
}

#[test]
fn css_tags_and_window_define() {
    let root = package(
        "css",
        &[(
            "x-el.js",
            r#"
/**
 * @cssprop {<length>} [--x-gap=4px] - Gap.
 * @cssstate active - When active.
 * @tag ignored-when-defined
 */
class XEl extends HTMLElement {}
window.customElements.define('x-el', XEl);
"#,
        )],
    );
    let m = extract(&root, &["x-el.js"]);
    let el = decl(module(&m, "x-el.js"), "XEl");
    assert_eq!(
        el["cssProperties"],
        json!([{ "name": "--x-gap", "default": "4px", "syntax": "<length>", "description": "Gap." }])
    );
    assert_eq!(
        el["cssStates"],
        json!([{ "name": "active", "description": "When active." }])
    );
    assert_eq!(module(&m, "x-el.js")["exports"][0]["name"], "x-el");
    assert_eq!(el["tagName"], "x-el", "the defined tag beats `@tag`");
}

#[test]
fn parse_errors_name_the_file_and_line() {
    let root = package("broken", &[("x.ts", "export class X {\n  a = ;\n}\n")]);
    let err = holyfloe_lit::extract(&root, &[root.join("x.ts")]).unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("x.ts:2: "), "{msg}");
}
