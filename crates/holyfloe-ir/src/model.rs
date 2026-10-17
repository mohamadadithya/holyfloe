//! The IR: the only thing emitters read (D-011). See `docs/ir.md`.

use serde::Serialize;

use crate::PackageInfo;

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct Library {
    pub package: PackageInfo,
    pub components: Vec<Component>,
}

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct Component {
    pub tag: String,
    /// Wrapper name, Pascal(tag), e.g. `FxButton`.
    pub name: String,
    /// Element class name from the CEM declaration.
    pub class_name: String,
    /// Declares the class: `import type { El } from ...`.
    pub module: ModuleRef,
    /// Registers the tag: the side-effect import. Same as `module` unless a separate
    /// file holds the `custom-element-definition` export.
    pub definition: ModuleRef,
    pub summary: Option<String>,
    pub description: Option<String>,
    /// `Some("")` when deprecated without a reason.
    pub deprecated: Option<String>,
    pub props: Vec<Prop>,
    pub events: Vec<Event>,
    pub slots: Vec<Slot>,
    pub methods: Vec<Method>,
    pub css_parts: Vec<CssPart>,
    pub css_props: Vec<CssProp>,
    pub css_states: Vec<CssState>,
    pub form_associated: bool,
    pub bindings: Vec<TwoWayBinding>,
}

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct ModuleRef {
    /// CEM `module.path`, verbatim.
    pub path: String,
    /// Import specifier, e.g. `@holyfloe/fixtures/fx-button/fx-button.js`.
    pub specifier: String,
}

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct Prop {
    pub name: String,
    pub attribute: Option<String>,
    pub ty: Option<TypeRef>,
    /// Raw JS text.
    pub default: Option<String>,
    pub reflects: bool,
    pub readonly: bool,
    pub kind: PropKind,
    pub description: Option<String>,
    pub deprecated: Option<String>,
}

#[derive(Serialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum PropKind {
    Boolean,
    Primitive,
    Complex,
}

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct Event {
    /// Exact name as dispatched, e.g. `my-change`.
    pub dom_name: String,
    /// Framework-neutral prop name without prefix, e.g. `MyChange`. Each emitter adds its own (React: `on`).
    pub base_name: String,
    pub detail: Option<TypeRef>,
    pub event_type: Option<TypeRef>,
    pub description: Option<String>,
    pub deprecated: Option<String>,
}

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct Slot {
    /// `""` is the default slot.
    pub name: String,
    pub description: Option<String>,
    pub deprecated: Option<String>,
}

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct Method {
    pub name: String,
    pub description: Option<String>,
    pub deprecated: Option<String>,
}

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct CssPart {
    pub name: String,
    pub description: Option<String>,
    pub deprecated: Option<String>,
}

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct CssProp {
    pub name: String,
    pub syntax: Option<String>,
    pub default: Option<String>,
    pub description: Option<String>,
    pub deprecated: Option<String>,
}

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct CssState {
    pub name: String,
    pub description: Option<String>,
    pub deprecated: Option<String>,
}

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct TypeRef {
    /// Verbatim CEM type text, never rewritten.
    pub text: String,
    /// Filled only for event types with a `CustomEvent<T>` detail: the one type text emitters re-emit.
    pub imports: Vec<TypeImport>,
}

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct TypeImport {
    pub name: String,
    pub specifier: String,
}

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct TwoWayBinding {
    pub prop: String,
    /// `dom_name` of the event that signals a change.
    pub event: String,
    pub value: ValuePath,
    pub inferred: bool,
}

#[derive(Serialize, Clone, Debug, PartialEq)]
pub enum ValuePath {
    Detail,
    DetailField(String),
    TargetProp(String),
}
