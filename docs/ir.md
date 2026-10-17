# IR specification

The IR is the only thing emitters read. It is framework-neutral, serde-serializable, and built by `holyfloe-ir` from CEM (M1) or from Lit source (M4).

## Data model

```rust
// Source of truth: crates/holyfloe-ir/src/model.rs. Every Component, Prop, Event, Slot,
// Method, CssPart/CssProp/CssState also carries `description` and `deprecated`
// (`Some("")` = deprecated without a reason).

pub struct Library {
    pub package: PackageInfo,          // name, version, exports (deserialized from package.json)
    pub components: Vec<Component>,
}

pub struct Component {
    pub tag: String,                   // CEM: tagName, or a custom-element-definition export
    pub name: String,                  // derived: Pascal(tag), e.g. "FxButton"
    pub class_name: String,            // CEM: declaration.name
    pub module: ModuleRef,             // { path: CEM module.path, specifier: "<pkg>/<subpath>" }; declares the class (import type)
    pub definition: ModuleRef,         // registers the tag (side-effect import); the custom-element-definition export's module, else `module`
    pub summary: Option<String>,
    pub props: Vec<Prop>,
    pub events: Vec<Event>,
    pub slots: Vec<Slot>,
    pub methods: Vec<Method>,          // public, non-static
    pub css_parts: Vec<CssPart>,
    pub css_props: Vec<CssProp>,       // custom properties (+ syntax, default)
    pub css_states: Vec<CssState>,
    pub form_associated: bool,         // NOT in CEM (false until M2)
    pub bindings: Vec<TwoWayBinding>,  // NOT in CEM
}

pub struct Prop {
    pub name: String,                  // CEM: field.name
    pub attribute: Option<String>,     // CEM: field.attribute
    pub ty: Option<TypeRef>,           // CEM type is optional
    pub default: Option<String>,       // raw JS text
    pub reflects: bool,
    pub readonly: bool,
    pub kind: PropKind,                // derived
}

pub enum PropKind { Boolean, Primitive, Complex }

pub struct Event {
    pub dom_name: String,              // exact name as dispatched, e.g. "my-change"
    pub base_name: String,             // derived, framework-neutral, e.g. "MyChange"; each emitter adds its prefix (React: "on")
    pub detail: Option<TypeRef>,       // T from CustomEvent<T>, else None (emitted as `unknown`)
    pub event_type: Option<TypeRef>,   // e.g. CustomEvent<...>, or a named Event subclass
}

pub struct Slot { pub name: String /* "" = default */ }

pub struct TypeRef {
    pub text: String,                  // verbatim, never rewritten
    pub imports: Vec<TypeImport>,      // { name, specifier }; from CEM type.references, filled only when the event has a CustomEvent<T> detail
}

pub struct TwoWayBinding {
    pub prop: String,
    pub event: String,                 // dom_name of the event that signals a change
    pub value: ValuePath,
    pub inferred: bool,
}

pub enum ValuePath { Detail, DetailField(String), TargetProp(String) }
```

## Normalization passes (run in this order)

1. **flatten-inheritance.** Merge members, events, slots, parts from superclass and mixins using `inheritedFrom` and dependency manifests. Error on cycles.
2. **drop-non-public.** Remove private, protected, static, and `@internal`-tagged members, plus any `attributes` entry whose `fieldName` points at a removed member (the analyzer lists protected fields there too). The analyzer already strips `@internal`; the check still matters for M4 input.
3. **name-derive.** Compute event `base_name` (config `overrides.<tag>.events.<dom-name>` wins) and component names. Respect config `tagFormatter` and `componentNameFormatter`.
4. **classify-props.** Set `PropKind` by lexing `type.text`, after stripping top-level `| undefined` and `| null` (the analyzer writes optional props as `string | undefined`):
   - `boolean` -> Boolean
   - `string`, `number`, string/number literal unions, `bigint` -> Primitive
   - everything else (objects, arrays, functions, `unknown`, `any`, imported types) -> Complex

   `PropKind` comes from the type only. A Boolean or Primitive prop with `attribute: None` (Lit `attribute: false`) has no attribute to render: emitters set it as a property only.
5. **infer-bindings.** Apply the heuristic below, then overlay explicit `@model` JSDoc and config. Explicit always wins.
6. **validate.** Duplicate tags, duplicate prop names, events with empty names, event base names that clash, and component or event names that are not identifiers (`[A-Za-z0-9_$]`) are errors with source locations. An event override for a present tag that names no event of it is an error; overrides for absent tags are ignored, so one config can serve several manifests.

## Binding inference heuristic

A binding is inferred when a component has prop `P` in {`value`, `checked`, `open`, `selected`} AND an event `E` in {`change`, `input`, `<P>-change`, `<P>Change`}.
- `value` + `input`/`change` on form-associated components: `ValuePath::TargetProp("value")`.
- `checked` + `change`: `ValuePath::TargetProp("checked")`.
- Otherwise `ValuePath::Detail`.
Inferred bindings are marked `inferred: true` and can be disabled with config `bindings: "explicit-only"`.

## What CEM lacks and how the IR fills it

| Need | CEM gap | IR fill |
|---|---|---|
| Two-way binding | Not modelled | `@model` JSDoc tag via analyzer plugin, config override, or heuristic above |
| Event detail types | `type.text` often bare `CustomEvent` | `@fires name {DetailType}` JSDoc convention (M1-M3); `new CustomEvent<T>` extraction (M4) |
| Prop kind | Only type text | `classify-props` pass |
| Form association | Absent | Config flag or `static formAssociated` detection (M4) |
| Importable type specifiers | `type.references` optional, often missing | Emit `import type { El } from '<pkg>/<module>'` and use `El['prop']` |
| Cross-package inheritance | `inheritedFrom` partial | Flatten pass over `node_modules/*/custom-elements.json` |
| SSR capability | Absent | Config `ssr: "lit" \| "none"` |

## Config file (`holyfloe.config.json`)

```json
{
  "cem": "./custom-elements.json",
  "outDir": "./generated",
  "frameworks": ["react", "vue", "svelte"],
  "react": { "mode": "wrapper", "useClient": true },
  "ssr": "none",
  "bindings": "infer",
  "tagFormatter": null,
  "componentNameFormatter": null,
  "exclude": ["internal-*"],
  "overrides": {
    "my-input": { "bindings": [{ "prop": "value", "event": "my-change", "value": { "detailField": "value" } }] }
  }
}
```

## Errors

Use `thiserror`. Every error carries: component tag (if known), CEM module path, and a short fix hint. The CLI prints them as `error[E0xx]: message` with the hint on the next line.
