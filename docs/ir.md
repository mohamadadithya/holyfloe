# IR specification

The IR is the only thing emitters read. It is framework-neutral, serde-serializable, and built by `holyfloe-ir` from CEM (M1) or from Lit source (M4).

## Data model

```rust
pub struct Library {
    pub package: PackageInfo,          // name, version, entry points from package.json "exports"
    pub components: Vec<Component>,
}

pub struct Component {
    pub tag: String,                   // CEM: custom-element-definition export / tagName
    pub class_name: String,            // CEM: declaration.name
    pub module: ModuleRef,             // CEM: module.path -> import specifier that defines the element
    pub summary: Option<String>,
    pub description: Option<String>,
    pub deprecated: Option<String>,
    pub props: Vec<Prop>,
    pub events: Vec<Event>,
    pub slots: Vec<Slot>,
    pub methods: Vec<Method>,          // public, non-static
    pub css_parts: Vec<CssPart>,
    pub css_props: Vec<CssProp>,       // custom properties
    pub css_states: Vec<CssState>,
    pub form_associated: bool,         // NOT in CEM
    pub bindings: Vec<TwoWayBinding>,  // NOT in CEM
}

pub struct Prop {
    pub name: String,                  // CEM: field.name
    pub attribute: Option<String>,     // CEM: field.attribute
    pub ty: TypeRef,
    pub default: Option<String>,       // raw JS text
    pub reflects: bool,
    pub readonly: bool,
    pub kind: PropKind,                // derived
}

pub enum PropKind { Boolean, Primitive, Complex }

pub struct Event {
    pub dom_name: String,              // exact name as dispatched, e.g. "my-change"
    pub prop_name: String,             // derived, e.g. "onMyChange"
    pub detail: Option<TypeRef>,
    pub event_type: Option<TypeRef>,   // e.g. CustomEvent<...>, or a named Event subclass
}

pub struct Slot { pub name: String /* "" = default */, pub description: Option<String> }

pub struct TypeRef {
    pub text: String,                  // verbatim, never rewritten
    pub imports: Vec<TypeImport>,      // from CEM type.references when present
}

pub struct TwoWayBinding {
    pub prop: String,
    pub event: String,                 // dom_name of the event that signals a change
    pub value: ValuePath,
}

pub enum ValuePath { Detail, DetailField(String), TargetProp(String) }
```

## Normalization passes (run in this order)

1. **flatten-inheritance.** Merge members, events, slots, parts from superclass and mixins using `inheritedFrom` and dependency manifests. Error on cycles.
2. **drop-non-public.** Remove private, protected, static, and `@internal`-tagged members.
3. **name-derive.** Compute `prop_name` for events, component class names, and file names. Respect config `tagFormatter` and `componentNameFormatter`.
4. **classify-props.** Set `PropKind` by lexing `type.text`:
   - `boolean` -> Boolean
   - `string`, `number`, string/number literal unions, `bigint` -> Primitive
   - everything else (objects, arrays, functions, `unknown`, `any`, imported types) -> Complex
5. **infer-bindings.** Apply the heuristic below, then overlay explicit `@model` JSDoc and config. Explicit always wins.
6. **validate.** Duplicate tags, duplicate prop names, events with empty names are errors with source locations.

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
