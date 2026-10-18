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
    pub export_name: String,           // name `module` exports the class under: class_name, or "default" (Shoelace, Web Awesome)
    pub definition: ModuleRef,         // registers the tag (side-effect import); the custom-element-definition export's module, else `module`
    pub summary: Option<String>,
    pub props: Vec<Prop>,
    pub events: Vec<Event>,
    pub slots: Vec<Slot>,
    pub methods: Vec<Method>,          // public, non-static
    pub css_parts: Vec<CssPart>,
    pub css_props: Vec<CssProp>,       // custom properties (+ syntax, default)
    pub css_states: Vec<CssState>,
    pub form_associated: bool,         // NOT in CEM: a static `formAssociated` field defaulting to `true` (D-065)
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

## Input

Parse manifests with `holyfloe_cem::parse`, never `serde_json::from_str`. The declarations enum is untagged, so an element missing one required field would otherwise fall back to a plain `ClassDeclaration` and silently lose its tag. `parse` makes that an error. Other declarations and `js` exports that match no schema variant are dropped: published manifests include bundled dependency code (Lion ships minified Lit classes and Popper exports), and those never produce output. `Event.type` is optional (schema patch 3, D-049).

`build(cem, package_json, deps, config)` first checks the config (`E010`), then discovers classes. `deps` are the manifests of other packages that superclasses and mixins name (D-069); the CLI finds them through `node_modules/<pkg>/package.json` `customElements`, and a package without one (`lit`) is skipped:
- A class is a component for each tag it has, from `tagName` or a `custom-element-definition` export. Literal quotes around a tag are stripped (the analyzer keeps `` `lion-input-range` `` verbatim). `customElement: true` without a tag is a base class.
- One class with two tags gives two components. The analyzer identifies classes by name, so same-named classes in one module look like one class.
- Module references inside the manifest (re-exports, superclasses, definition exports, type references) starting with `./` or `../` are relative to the module that wrote them. `/x` and `x` are package-root paths. `js` re-exports are followed to the declaring class.
- Components whose tag matches a config `exclude` glob are dropped after flatten, so they can still be base classes.

## Module resolution

Each component gets two `ModuleRef`s:
- `module`: the declaring module if `exports` matches it, else the first module that re-exports the class under its own name (barrels such as Lion's `exports/*.js`).
  - `export_name` is the name `module` exports the class under. For the declaring module it comes from that module's `js` exports: the class name if it is listed, else the first other name (`default` for `export default class`), else the class name. For a barrel it is the class name.
- `definition`: the module holding the `custom-element-definition` export, else `module`.

A module path becomes a specifier in four steps:
1. Strip a leading `/` or `./`.
2. Change `.ts`/`.tsx` to `.js`, `.mts` to `.mjs` and `.cts` to `.cjs`.
3. Apply the longest matching `moduleRewrite` prefix. Keys and values are normalized the same way.
4. Match the result against `package.json` `exports`. String targets, `*` patterns and condition objects (`import` > `default`) are supported. Exact keys beat patterns, and longer pattern prefixes beat shorter ones. A candidate counts only if Node's forward lookup of that subpath lands on the same file, so `null` exclusions and shadowing keys are honoured.

No `exports` field means every file is importable as `<pkg>/<path>`. No match at all is `E002`.

## Normalization passes (run in this order)

1. **flatten-inheritance.** Merge members, events, slots, parts and `@model` bindings from the superclass chain and its mixins, in this package or a dependency manifest. Own entries win, then mixins, then the superclass. Error on cycles.
2. **drop-non-public.** Remove private, protected, static, and `@internal`-tagged members, plus any `attributes` entry whose `fieldName` points at a removed member (the analyzer lists protected fields there too). The analyzer already strips `@internal`; the check still matters for M4 input.
3. **name-derive.** Compute event `base_name` (config `overrides.<tag>.events.<dom-name>` wins) and component names, and drop the props listed in `overrides.<tag>.excludeProps`. Respect config `tagFormatter` and `componentNameFormatter`.
4. **classify-props.** Set `PropKind` by lexing `type.text`, after stripping top-level `| undefined` and `| null` (the analyzer writes optional props as `string | undefined`):
   - `boolean` -> Boolean
   - `string`, `number`, string/number literal unions, `bigint` -> Primitive
   - everything else (objects, arrays, functions, `unknown`, `any`, imported types) -> Complex

   `PropKind` comes from the type only. A Boolean or Primitive prop with `attribute: None` (Lit `attribute: false`) has no attribute to render: emitters set it as a property only.
5. **infer-bindings.** Apply the heuristic below, then overlay explicit `@model` JSDoc, then config `overrides.<tag>.bindings` (D-062). For each prop the later source replaces the earlier binding.
6. **validate.** Duplicate tags, component names that differ only in case or not at all (emitters name files after them), events with empty names, event base names that clash, and component or event names that are not identifiers (`[A-Za-z0-9_$]`) are errors with source locations. Override errors (unknown tag, event or excluded prop, `E009`) are raised earlier, in name-derive; an override for a tag removed by `exclude` is allowed.

## Binding inference heuristic

A binding is inferred when a component has prop `P` in {`value`, `checked`, `open`, `selected`} AND an event `E` in {`change`, `input`, `<P>-change`, `<P>Change`}.
- `value` + `input`/`change` on form-associated components: `ValuePath::TargetProp("value")`.
- `checked` + `change`: `ValuePath::TargetProp("checked")`.
- Otherwise `ValuePath::Detail`.
Inferred bindings are marked `inferred: true` and can be disabled with config `bindings: "explicit-only"`.

## Explicit bindings (D-062, D-063)

- JSDoc on the class: `@model <prop> <event> [<path>]`, e.g. `@model open open-change detail.open`. The CEM analyzer drops unknown tags, so `fixtures/holyfloe-cem-plugin.mjs` writes them to the declaration as `"x-holyfloe-models": [{ "prop", "event", "value"? }]` (schema patch 4).
- Config: `overrides.<tag>.bindings: [{ "prop": "value", "event": "fx-change", "value": "detail.value" }]`.
- `<path>` / `value`: `detail` (default), `detail.<field>`, `target.<prop>` (read from the host element).
- An unknown or read-only prop, an unknown event, a malformed path, or the same prop twice in one source is `E015`.

## What CEM lacks and how the IR fills it

| Need | CEM gap | IR fill |
|---|---|---|
| Two-way binding | Not modelled | `@model` JSDoc tag via the holyfloe analyzer plugin, config override, or heuristic above |
| Event detail types | `type.text` often bare `CustomEvent` | `@fires name {DetailType}` JSDoc convention (M1-M3); `new CustomEvent<T>` extraction (M4) |
| Prop kind | Only type text | `classify-props` pass |
| Form association | Absent | The analyzer records `static formAssociated = true` as a static field; the IR reads it (D-065) |
| Importable type specifiers | `type.references` optional, often missing | Emit `import type { El } from '<pkg>/<module>'` and use `El['prop']` |
| Cross-package inheritance | `inheritedFrom` partial | Flatten pass over dependency manifests (`customElements` in their `package.json`) |
| SSR capability | Absent | Config `ssr: "lit" \| "none"` |

## Config file (`holyfloe.config.json`)

The IR reads these keys and ignores the rest:
- `moduleRewrite`: prefix pairs, see Module resolution (D-053). Fixtures use `{"src/": "dist/"}`. Shoelace 2 and Web Awesome 3 write manifest paths relative to `dist/` and need `{"": "dist/"}`.
- `bindings`: `"infer"` (default) or `"explicit-only"`.
- `exclude`: tag globs, where `*` matches any run of characters.
- `overrides.<tag>.excludeProps`: prop names to leave out of every wrapper. The manifest cannot tell a stale JSDoc `@property` (Web Awesome `wa-drawer` `modal`, absent from the class, so `El['modal']` fails) from a real untyped field, so this is explicit; it also hides public internals such as Web Awesome's `didSSR` (D-059). Web Awesome needs `"wa-drawer": { "excludeProps": ["modal"] }`. A name that matches no prop is `E009`.
- `overrides.<tag>.bindings`: explicit two-way bindings, see Explicit bindings. A prop or event the component lacks is `E015`.
- `overrides.<tag>.events.<dom-name>`: the base prop name, without a framework prefix (`"myChange": "MyChangeCamel"` gives React `onMyChangeCamel`). This is how `E006` clashes are fixed (D-057). An unknown tag or event is `E009`, so a config serves one manifest. Fixtures with overrides have their own `fixtures/src/<name>/holyfloe.config.json`.
- `tagFormatter`, `componentNameFormatter`: any non-null value is `E010` (not supported yet).

The `react` key belongs to `holyfloe-emit-react` (`mode`, `useClient`, `ref`; see `emit-react.md`), which rejects unknown keys (D-060). The example also shows keys for later milestones (`cem`, `outDir`, `frameworks`, `ssr`); the IR ignores them, and the CLI takes `--cem` and `--out` as flags.

```json
{
  "cem": "./custom-elements.json",
  "outDir": "./generated",
  "frameworks": ["react", "vue", "svelte"],
  "moduleRewrite": { "src/": "dist/" },
  "react": { "mode": "wrapper", "useClient": true, "ref": "forwardRef" },
  "ssr": "none",
  "bindings": "infer",
  "tagFormatter": null,
  "componentNameFormatter": null,
  "exclude": ["internal-*"],
  "overrides": {
    "my-input": { "bindings": [{ "prop": "value", "event": "my-change", "value": "detail.value" }] }
  }
}
```

## Errors

Use `thiserror`. Every error carries: component tag (if known), CEM module path, and a short fix hint. The CLI prints them as `error[E0xx]: message (module)` with `  hint: ...` on the next line, and exits 1. The module is left out when the message already names it. Emitter errors a user can fix share the table (`EmitError::code()`/`hint()`); template errors are holyfloe bugs and print as `error: ...` with no code. E001 was never used and E011 is retired (used, then removed, in M1 step 2); never reuse a retired code.

| Code | Where | Meaning |
|---|---|---|
| E002 | lower | A module (component, definition, or detail type reference) matches no `exports` entry, and no exported module re-exports the class |
| E003 | flatten | Inheritance cycle |
| E004 | validate | Tag defined more than once |
| E005 | validate | Prop declared more than once |
| E006 | validate | Several events map to the same base prop name |
| E007 | validate | Event with an empty name |
| E008 | lower, validate | Tag is not a valid custom element name (e.g. a variable the analyzer recorded), or a component/event name is not `[A-Za-z0-9_$]` |
| E009 | name-derive | Override for a tag, event or excluded prop that does not exist |
| E010 | config check | Unsupported config key (`tagFormatter`, `componentNameFormatter`) |
| E012 | emitters | An event detail type name is imported from two modules, or is a name the generated file already uses (`EmitError::DetailImportClash`) |
| E013 | emitters (React wrapper mode, Vue, Svelte) | A prop has the same name as a name the wrapper generates: an event's `on<base_name>`, Svelte `element`, Vue `modelValue` (`EmitError::PropEventClash`) |
| E014 | validate | Two tags give the same component name, ignoring case (`fx-button` and `fx--button`, or `x-ab` and `xa-b`) |
| E015 | infer-bindings | An explicit binding (`@model` or config) names an unknown or read-only prop or an unknown event, has a malformed path, or repeats a prop |

Manifest parse errors come from `holyfloe_cem::ParseError`, not `IrError`.
