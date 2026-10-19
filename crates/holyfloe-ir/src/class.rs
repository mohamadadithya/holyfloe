//! Working representation between CEM and the IR. Flatten and drop-non-public need
//! superclass links, privacy and `static`, which the IR does not keep.

use std::collections::{HashMap, HashSet};

use holyfloe_cem as cem;

use crate::model::*;
use crate::passes::{classify_props, infer_bindings};
use crate::{Config, IrError, PackageInfo, module_path};

/// `true` -> `Some("")`, a reason -> `Some(reason)`. Every CEM `*Deprecated` enum has this shape.
macro_rules! deprecated {
    ($value:expr, $ty:ident) => {
        match &$value {
            Some(cem::$ty::String(reason)) => Some(reason.clone()),
            Some(cem::$ty::Boolean(true)) => Some(String::new()),
            _ => None,
        }
    };
}

#[derive(Clone, Debug)]
pub(crate) struct Class {
    pub name: String,
    /// The dependency package declaring the class; `None` for the manifest being built.
    pub package: Option<String>,
    pub module: String,
    pub tag: Option<String>,
    /// Module holding the `custom-element-definition` export, when the manifest has one.
    pub definition_module: Option<String>,
    /// Modules that re-export this class under its own name, in manifest order.
    pub reexports: Vec<String>,
    /// Names the declaring module exports this class under, e.g. `default`.
    pub own_exports: Vec<String>,
    /// The class that declares the superclass, re-exports followed.
    pub superclass: Option<Base>,
    /// Mixins applied in the extends clause, innermost first (CEM order).
    pub mixins: Vec<Base>,
    /// From a static `formAssociated` field defaulting to `true` (D-065); set by drop-non-public.
    pub form_associated: bool,
    /// `@model` tags from the holyfloe analyzer plugin (D-063).
    pub models: Vec<cem::HolyfloeModel>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub deprecated: Option<String>,
    pub members: Vec<Member>,
    pub events: Vec<cem::Event>,
    pub slots: Vec<cem::Slot>,
    pub css_parts: Vec<cem::CssPart>,
    pub css_props: Vec<cem::CssCustomProperty>,
    pub css_states: Vec<cem::CssCustomState>,
}

/// A declared class: (normalized module, class name).
pub(crate) type Key = (String, String);

/// A superclass or mixin reference.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Base {
    /// A class in the same manifest.
    Local(Key),
    /// A class in another package's manifest. Missing when that package has none (`lit`).
    // shortcut: matched by class name within the package, not by module; upgrade if a dependency
    // declares two classes with one name.
    Package { package: String, name: String },
}

pub(crate) fn key(module: &str, name: &str) -> Key {
    (module_path::normalize(module).to_owned(), name.to_owned())
}

#[derive(Clone, Debug)]
pub(crate) struct Member {
    pub name: String,
    pub privacy: Option<cem::Privacy>,
    pub is_static: bool,
    pub description: Option<String>,
    pub deprecated: Option<String>,
    pub field: Option<Field>,
}

/// `None` on `Member::field` means a method.
#[derive(Clone, Debug)]
pub(crate) struct Field {
    pub attribute: Option<String>,
    pub reflects: bool,
    pub readonly: bool,
    pub ty: Option<cem::Type>,
    pub default: Option<String>,
}

/// Every class declaration in the manifest. A class is a component when it has a tag, from
/// `tagName` or a `custom-element-definition` export. `customElement: true` without a tag
/// (e.g. `FxBase`) is a base class (D-055).
pub(crate) fn discover(pkg: &cem::Package) -> Vec<Class> {
    let mut declared = HashSet::new();
    // (module, exported name) -> the (module, name) it re-exports
    let mut reexported: HashMap<Key, Key> = HashMap::new();
    for module in &pkg.modules {
        for decl in &module.declarations {
            if let Some(name) = class_name(decl) {
                declared.insert(key(&module.path, name));
            }
        }
        for export in &module.exports {
            if let cem::JavaScriptModuleExportsItem::JavaScriptExport(e) = export
                && e.name != "*"
                && let Some(target) = &e.declaration.module
            {
                let from = key(&module.path, &e.name);
                let to = key(
                    &module_path::resolve_ref(&module.path, target),
                    &e.declaration.name,
                );
                if from != to {
                    reexported.insert(from, to);
                }
            }
        }
    }
    // Follow re-exports to the declaring class. Bounded, so a re-export cycle cannot hang.
    let canonical = |mut k: Key| {
        for _ in 0..32 {
            match reexported.get(&k) {
                Some(next) if !declared.contains(&k) => k = next.clone(),
                _ => break,
            }
        }
        k
    };
    let target = |module: &str, decl: &cem::Reference| {
        let path = match &decl.module {
            Some(m) => module_path::resolve_ref(module, m),
            None => module.to_owned(),
        };
        canonical(key(&path, &decl.name))
    };

    // class -> [(tag, module that defines it)], and class -> modules re-exporting it
    let mut tags: HashMap<Key, Vec<(String, String)>> = HashMap::new();
    let mut reexports: HashMap<Key, Vec<String>> = HashMap::new();
    let mut own_exports: HashMap<Key, Vec<String>> = HashMap::new();
    for module in &pkg.modules {
        for export in &module.exports {
            if let cem::JavaScriptModuleExportsItem::JavaScriptExport(e) = export {
                let class = target(&module.path, &e.declaration);
                if class.0 == module_path::normalize(&module.path) {
                    own_exports.entry(class).or_default().push(e.name.clone());
                }
            }
            match export {
                cem::JavaScriptModuleExportsItem::CustomElementExport(e) => {
                    let class = target(&module.path, &e.declaration);
                    tags.entry(class)
                        .or_default()
                        .push((unquote(&e.name), module.path.clone()));
                }
                cem::JavaScriptModuleExportsItem::JavaScriptExport(e)
                    if e.name == e.declaration.name =>
                {
                    let class = target(&module.path, &e.declaration);
                    if class != key(&module.path, &e.name) {
                        reexports
                            .entry(class)
                            .or_default()
                            .push(module.path.clone());
                    }
                }
                _ => {}
            }
        }
    }

    let mut classes = Vec::new();
    for module in &pkg.modules {
        for decl in &module.declarations {
            let (mut class, superclass, mixins) = match decl {
                cem::JavaScriptModuleDeclarationsItem::CustomElementDeclaration(d) => (
                    from_element(d, &module.path),
                    d.superclass.as_ref(),
                    &d.mixins,
                ),
                cem::JavaScriptModuleDeclarationsItem::ClassDeclaration(d) => (
                    from_class(d, &module.path),
                    d.superclass.as_ref(),
                    &d.mixins,
                ),
                cem::JavaScriptModuleDeclarationsItem::CustomElementMixinDeclaration(d) => (
                    from_mixin(d, &module.path),
                    d.superclass.as_ref(),
                    &d.mixins,
                ),
                _ => continue,
            };
            let base = |r: &cem::Reference| match &r.package {
                Some(p) => Base::Package {
                    package: module_path::package_name(p).to_owned(),
                    name: r.name.clone(),
                },
                None => Base::Local(target(&module.path, r)),
            };
            class.superclass = superclass.map(base);
            class.mixins = mixins.iter().map(base).collect();
            class.tag = class.tag.as_deref().map(unquote);
            let own = key(&module.path, &class.name);
            class.reexports = reexports.get(&own).cloned().unwrap_or_default();
            class.own_exports = own_exports.get(&own).cloned().unwrap_or_default();
            let defs = tags.get(&own).map(Vec::as_slice).unwrap_or_default();
            let mut found: Vec<(String, Option<String>)> = defs
                .iter()
                .map(|(tag, def)| (tag.clone(), Some(def.clone())))
                .collect();
            // A `tagName` with no matching export is defined where the class is imported.
            if let Some(own) = &class.tag
                && !defs.iter().any(|(tag, _)| tag == own)
            {
                found.insert(0, (own.clone(), None));
            }
            if found.is_empty() {
                classes.push(class);
                continue;
            }
            // One component per tag. The analyzer identifies classes by name, so two
            // same-named classes in one module (Lion's test suites) read as one class with two
            // tags. A real double registration is the library's own runtime error.
            for (tag, definition) in found {
                let mut c = class.clone();
                c.tag = Some(tag);
                c.definition_module = definition;
                classes.push(c);
            }
        }
    }
    classes
}

/// The analyzer keeps a template-literal tag verbatim (Lion: `` `lion-input-range` ``).
fn unquote(tag: &str) -> String {
    let quoted = |q: char| tag.len() >= 2 && tag.starts_with(q) && tag.ends_with(q);
    if quoted('\'') || quoted('"') || (quoted('`') && !tag.contains("${")) {
        tag[1..tag.len() - 1].to_owned()
    } else {
        tag.to_owned()
    }
}

fn class_name(decl: &cem::JavaScriptModuleDeclarationsItem) -> Option<&str> {
    match decl {
        cem::JavaScriptModuleDeclarationsItem::CustomElementDeclaration(d) => Some(&d.name),
        cem::JavaScriptModuleDeclarationsItem::ClassDeclaration(d) => Some(&d.name),
        cem::JavaScriptModuleDeclarationsItem::CustomElementMixinDeclaration(d) => Some(&d.name),
        _ => None,
    }
}

fn from_element(d: &cem::CustomElementDeclaration, module: &str) -> Class {
    let members = d
        .members
        .iter()
        .map(|m| match m {
            cem::CustomElementDeclarationMembersItem::CustomElementField(f) => element_field(f),
            cem::CustomElementDeclarationMembersItem::ClassMethod(m) => method(m),
        })
        .collect();
    Class {
        name: d.name.clone(),
        package: None,
        module: module.to_owned(),
        tag: d.tag_name.clone(),
        definition_module: None,
        reexports: Vec::new(),
        own_exports: Vec::new(),
        superclass: None,
        mixins: Vec::new(),
        form_associated: false,
        models: d.x_holyfloe_models.clone(),
        summary: d.summary.clone(),
        description: d.description.clone(),
        deprecated: deprecated!(d.deprecated, CustomElementDeclarationDeprecated),
        members,
        events: d.events.clone(),
        slots: d.slots.clone(),
        css_parts: d.css_parts.clone(),
        css_props: d.css_properties.clone(),
        css_states: d.css_states.clone(),
    }
}

/// A mixin is never a component itself; flatten merges it into the classes that apply it.
fn from_mixin(d: &cem::CustomElementMixinDeclaration, module: &str) -> Class {
    let members = d
        .members
        .iter()
        .map(|m| match m {
            cem::CustomElementMixinDeclarationMembersItem::CustomElementField(f) => {
                element_field(f)
            }
            cem::CustomElementMixinDeclarationMembersItem::ClassMethod(m) => method(m),
        })
        .collect();
    Class {
        name: d.name.clone(),
        package: None,
        module: module.to_owned(),
        tag: None,
        definition_module: None,
        reexports: Vec::new(),
        own_exports: Vec::new(),
        superclass: None,
        mixins: Vec::new(),
        form_associated: false,
        models: Vec::new(),
        summary: d.summary.clone(),
        description: d.description.clone(),
        deprecated: deprecated!(d.deprecated, CustomElementMixinDeclarationDeprecated),
        members,
        events: d.events.clone(),
        slots: d.slots.clone(),
        css_parts: d.css_parts.clone(),
        css_props: d.css_properties.clone(),
        css_states: d.css_states.clone(),
    }
}

fn element_field(f: &cem::CustomElementField) -> Member {
    Member {
        name: f.name.clone(),
        privacy: f.privacy,
        is_static: f.static_.unwrap_or(false),
        description: f.description.clone(),
        deprecated: deprecated!(f.deprecated, CustomElementFieldDeprecated),
        field: Some(Field {
            attribute: f.attribute.clone(),
            reflects: f.reflects.unwrap_or(false),
            readonly: f.readonly.unwrap_or(false),
            ty: f.type_.clone(),
            default: f.default.clone(),
        }),
    }
}

fn from_class(d: &cem::ClassDeclaration, module: &str) -> Class {
    let members = d
        .members
        .iter()
        .map(|m| match m {
            cem::ClassDeclarationMembersItem::Field(f) => Member {
                name: f.name.clone(),
                privacy: f.privacy,
                is_static: f.static_.unwrap_or(false),
                description: f.description.clone(),
                deprecated: deprecated!(f.deprecated, ClassFieldDeprecated),
                field: Some(Field {
                    attribute: None,
                    reflects: false,
                    readonly: f.readonly.unwrap_or(false),
                    ty: f.type_.clone(),
                    default: f.default.clone(),
                }),
            },
            cem::ClassDeclarationMembersItem::Method(m) => method(m),
        })
        .collect();
    Class {
        name: d.name.clone(),
        package: None,
        module: module.to_owned(),
        tag: None,
        definition_module: None,
        reexports: Vec::new(),
        own_exports: Vec::new(),
        superclass: None,
        mixins: Vec::new(),
        form_associated: false,
        models: Vec::new(),
        summary: d.summary.clone(),
        description: d.description.clone(),
        deprecated: deprecated!(d.deprecated, ClassDeclarationDeprecated),
        members,
        events: Vec::new(),
        slots: Vec::new(),
        css_parts: Vec::new(),
        css_props: Vec::new(),
        css_states: Vec::new(),
    }
}

fn method(m: &cem::ClassMethod) -> Member {
    Member {
        name: m.name.clone(),
        privacy: m.privacy,
        is_static: m.static_.unwrap_or(false),
        description: m.description.clone(),
        deprecated: deprecated!(m.deprecated, ClassMethodDeprecated),
        field: None,
    }
}

impl Class {
    /// Into the IR. Derived fields (`name`, `base_name`, `kind`, `detail`, `bindings`) are
    /// left empty for the passes that compute them.
    pub(crate) fn lower(self, pkg: &PackageInfo, cfg: &Config) -> Result<Component, IrError> {
        let tag = self.tag.unwrap_or_default();
        if !is_custom_element_name(&tag) {
            return Err(IrError::new(
                "E008",
                format!(
                    "`{tag}` (class `{}`) is not a valid custom element name; the analyzer may have recorded a variable instead of a tag",
                    self.name
                ),
                Some(&tag),
                Some(&self.module),
                format!(
                    "define the element with a string literal tag, or add `{tag}` to `exclude` in holyfloe.config.json"
                ),
            ));
        }
        let module_ref = |path: String| {
            module_path::specifier(&path, pkg, cfg)
                .map(|specifier| ModuleRef {
                    path: path.clone(),
                    specifier,
                })
                .map_err(|file| unexported(&file, Some(&tag), &path))
        };
        // The declaring module if it is exported, else the first module re-exporting the class
        // (barrel layouts such as Lion's `exports/*.js`).
        // Shoelace and Web Awesome export their classes only as `default`. With no export
        // listed, assume the class name.
        let own_export = if self.own_exports.contains(&self.name) {
            self.name.clone()
        } else {
            self.own_exports
                .first()
                .cloned()
                .unwrap_or_else(|| self.name.clone())
        };
        let (module, export_name) = match module_ref(self.module.clone()) {
            Ok(m) => (m, own_export),
            // Barrels are only recorded when they re-export the class under its own name.
            Err(err) => (
                self.reexports
                    .into_iter()
                    .find_map(|path| module_ref(path).ok())
                    .ok_or(err)?,
                self.name.clone(),
            ),
        };
        // A separate `define.ts` registers the element; importing the class module would not.
        let definition = match self.definition_module {
            Some(path) => module_ref(path)?,
            None => module.clone(),
        };
        // Prop and event names are checked in infer-bindings, after name-derive drops excluded props.
        let bindings = self
            .models
            .into_iter()
            .map(|m| {
                let value = infer_bindings::value_path(m.value.as_deref()).ok_or_else(|| {
                    infer_bindings::bad_path(m.value.as_deref(), &tag, &self.module, "`@model`")
                })?;
                Ok(TwoWayBinding {
                    prop: m.prop,
                    event: m.event,
                    value,
                    inferred: false,
                })
            })
            .collect::<Result<_, IrError>>()?;
        let mut props = Vec::new();
        let mut methods = Vec::new();
        for m in self.members {
            match m.field {
                Some(f) => props.push(Prop {
                    name: m.name,
                    attribute: f.attribute,
                    ty: f.ty.map(|t| TypeRef {
                        text: t.text,
                        imports: Vec::new(),
                    }),
                    default: f.default,
                    reflects: f.reflects,
                    readonly: f.readonly,
                    kind: PropKind::Complex,
                    description: m.description,
                    deprecated: m.deprecated,
                }),
                None => methods.push(Method {
                    name: m.name,
                    description: m.description,
                    deprecated: m.deprecated,
                }),
            }
        }
        let events = self
            .events
            .into_iter()
            .map(|e| {
                // Only a `CustomEvent<T>` detail re-emits type text, so only it needs imports;
                // resolving the others could fail on modules nothing imports.
                let event_type = match e.type_ {
                    Some(ty) => Some(TypeRef {
                        imports: match classify_props::detail(&ty.text) {
                            Some(_) => type_imports(&ty, &self.module, pkg, cfg, &tag)?,
                            None => Vec::new(),
                        },
                        text: ty.text,
                    }),
                    // A `@fires` tag without a dispatch has no type (D-055).
                    None => None,
                };
                Ok(Event {
                    deprecated: deprecated!(e.deprecated, EventDeprecated),
                    dom_name: e.name,
                    base_name: String::new(),
                    detail: None,
                    event_type,
                    description: e.description,
                })
            })
            .collect::<Result<_, IrError>>()?;
        Ok(Component {
            name: String::new(),
            class_name: self.name,
            module,
            export_name,
            definition,
            summary: self.summary,
            description: self.description,
            deprecated: self.deprecated,
            props,
            events,
            slots: self
                .slots
                .into_iter()
                .map(|s| Slot {
                    deprecated: deprecated!(s.deprecated, SlotDeprecated),
                    name: s.name,
                    description: s.description,
                })
                .collect(),
            methods,
            css_parts: self
                .css_parts
                .into_iter()
                .map(|p| CssPart {
                    deprecated: deprecated!(p.deprecated, CssPartDeprecated),
                    name: p.name,
                    description: p.description,
                })
                .collect(),
            css_props: self
                .css_props
                .into_iter()
                .map(|p| CssProp {
                    deprecated: deprecated!(p.deprecated, CssCustomPropertyDeprecated),
                    name: p.name,
                    syntax: p.syntax,
                    default: p.default,
                    description: p.description,
                })
                .collect(),
            css_states: self
                .css_states
                .into_iter()
                .map(|s| CssState {
                    deprecated: deprecated!(s.deprecated, CssCustomStateDeprecated),
                    name: s.name,
                    description: s.description,
                })
                .collect(),
            form_associated: self.form_associated,
            bindings,
            tag,
        })
    }
}

/// HTML's valid custom element name: starts with `a-z`, contains `-`, no ASCII uppercase,
/// and not one of the reserved SVG/MathML names.
fn is_custom_element_name(tag: &str) -> bool {
    const RESERVED: [&str; 8] = [
        "annotation-xml",
        "color-profile",
        "font-face",
        "font-face-src",
        "font-face-uri",
        "font-face-format",
        "font-face-name",
        "missing-glyph",
    ];
    tag.starts_with(|c: char| c.is_ascii_lowercase())
        && tag.contains('-')
        && tag.chars().all(|c| {
            !c.is_ascii() || c.is_ascii_lowercase() || c.is_ascii_digit() || "-._".contains(c)
        })
        && !RESERVED.contains(&tag)
}

/// References without a module or package are globals and need no import.
fn type_imports(
    ty: &cem::Type,
    module: &str,
    pkg: &PackageInfo,
    cfg: &Config,
    tag: &str,
) -> Result<Vec<TypeImport>, IrError> {
    let mut imports = Vec::new();
    for r in &ty.references {
        let specifier = match (&r.package, &r.module) {
            // shortcut: imports from the package root; cross-package resolution comes in M2.
            (Some(package), _) => package.clone(),
            (None, Some(m)) => {
                let m = module_path::resolve_ref(module, m);
                module_path::specifier(&m, pkg, cfg)
                    .map_err(|file| unexported(&file, Some(tag), &m))?
            }
            (None, None) => continue,
        };
        imports.push(TypeImport {
            name: r.name.clone(),
            specifier,
        });
    }
    Ok(imports)
}

fn unexported(file: &str, tag: Option<&str>, module: &str) -> IrError {
    IrError::new(
        "E002",
        format!(
            "module `{module}` (published as `{file}`) matches no `exports` entry in package.json"
        ),
        tag,
        Some(module),
        format!(
            "export `./{file}` in package.json, re-export it from an exported module, or map the source path with `moduleRewrite` in holyfloe.config.json"
        ),
    )
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::test_util::{build_json, build_json_with, cem_one, element};
    use crate::{Config, PackageInfo};

    #[test]
    fn custom_element_names() {
        use super::is_custom_element_name as ok;
        assert!(ok("x-a") && ok("fx-button") && ok("x-el.v2") && ok("math-α"));
        assert!(!ok("tagString") && !ok("wrapperTagName") && !ok("button"));
        assert!(!ok("X-a") && !ok("1-a") && !ok("x-A") && !ok("font-face"));
    }

    #[test]
    fn literal_quotes_are_stripped_from_tags() {
        assert_eq!(super::unquote("`lion-input-range`"), "lion-input-range");
        assert_eq!(super::unquote("'x-a'"), "x-a");
        assert_eq!(super::unquote("`x-${p}`"), "`x-${p}`");
        let mut cem = cem_one(element("`x-a`", "XA"));
        cem["modules"][0]["exports"] = json!([define("`x-a`")]);
        let tags: Vec<_> = build_json(cem)
            .unwrap()
            .components
            .into_iter()
            .map(|c| c.tag)
            .collect();
        assert_eq!(tags, ["x-a"]);
    }

    #[test]
    fn variable_name_tag_is_e008_before_module_errors() {
        let cem = crate::test_util::cem(cem_one(element("tagString", "XA"))).unwrap();
        let pkg = PackageInfo {
            name: "pkg".into(),
            exports: Some(json!({})),
            ..PackageInfo::default()
        };
        let err = crate::build(&cem, &pkg, &[], &Config::default()).unwrap_err();
        assert_eq!(err.code, "E008");
        assert!(err.hint.contains("exclude"));
    }

    #[test]
    fn unexported_module_is_e002() {
        let cem = crate::test_util::cem(cem_one(element("x-a", "XA"))).unwrap();
        let pkg = PackageInfo {
            name: "pkg".into(),
            exports: Some(json!({ ".": "./dist/index.js" })),
            ..PackageInfo::default()
        };
        let err = crate::build(&cem, &pkg, &[], &Config::default()).unwrap_err();
        assert_eq!(err.code, "E002");
        assert_eq!(err.tag.as_deref(), Some("x-a"));
        assert_eq!(err.module.as_deref(), Some("src/x.ts"));
        assert!(err.hint.contains("moduleRewrite"));
    }

    fn define(tag: &str) -> serde_json::Value {
        json!({ "kind": "custom-element-definition", "name": tag,
                "declaration": { "name": "XA", "module": "src/x.ts" } })
    }

    #[test]
    fn one_class_under_two_tags_gives_two_components() {
        let mut cem = cem_one(element("x-a", "XA"));
        cem["modules"][0]["exports"] = json!([define("x-b")]);
        let lib = build_json(cem).unwrap();
        let tags: Vec<_> = lib
            .components
            .iter()
            .map(|c| (c.tag.as_str(), c.class_name.as_str()))
            .collect();
        assert_eq!(tags, [("x-a", "XA"), ("x-b", "XA")]);

        // The same tag twice (tagName and export) is one component.
        let mut cem = cem_one(element("x-a", "XA"));
        cem["modules"][0]["exports"] = json!([define("x-a")]);
        assert_eq!(build_json(cem).unwrap().components.len(), 1);
    }

    /// Lion's layout: class files are not exported; `exports/*.js` barrels re-export them with
    /// relative paths, and `exports/define/*.js` registers tags against the barrel.
    fn barrel_layout() -> serde_json::Value {
        let mut base = element("x-base-unused", "Base");
        base.as_object_mut().unwrap().remove("tagName");
        base["members"] = json!([{ "kind": "field", "name": "size" }]);
        let mut el = element("x-a", "XA");
        el["superclass"] = json!({ "name": "Base", "module": "/exports/base.js" });
        let reexport = |name: &str, module: &str| json!({ "kind": "js", "name": name, "declaration": { "name": name, "module": module } });
        json!({
            "schemaVersion": "1.0.0",
            "modules": [
                { "kind": "javascript-module", "path": "components/a/src/XA.js", "declarations": [el] },
                { "kind": "javascript-module", "path": "components/base/Base.js", "declarations": [base] },
                { "kind": "javascript-module", "path": "exports/a.js",
                  "exports": [reexport("XA", "../components/a/src/XA.js")] },
                { "kind": "javascript-module", "path": "exports/base.js",
                  "exports": [reexport("Base", "../components/base/Base.js")] },
                { "kind": "javascript-module", "path": "exports/define/x-a.js", "exports": [{
                    "kind": "custom-element-definition", "name": "x-a",
                    "declaration": { "name": "XA", "module": "/exports/a.js" } }] }
            ]
        })
    }

    #[test]
    fn barrel_reexports_give_import_and_definition_modules() {
        let cem = crate::test_util::cem(barrel_layout()).unwrap();
        let pkg = PackageInfo {
            name: "@lion/ui".into(),
            exports: Some(json!({ "./*": "./exports/*" })),
            ..PackageInfo::default()
        };
        let lib = crate::build(&cem, &pkg, &[], &Config::default()).unwrap();
        assert_eq!(
            lib.components.len(),
            1,
            "XA has tagName and a define export: one component"
        );
        let c = &lib.components[0];
        assert_eq!(c.module.specifier, "@lion/ui/a.js");
        assert_eq!(c.definition.specifier, "@lion/ui/define/x-a.js");
        // The superclass is found through the `exports/base.js` re-export.
        assert_eq!(c.props[0].name, "size");
        assert_eq!(c.export_name, "XA");
    }

    #[test]
    fn export_name_comes_from_the_declaring_module() {
        let export_name = |exports: serde_json::Value| {
            let mut cem = cem_one(element("x-a", "XA"));
            cem["modules"][0]["exports"] = exports;
            build_json(cem).unwrap().components[0].export_name.clone()
        };
        let js = |name: &str| json!({ "kind": "js", "name": name, "declaration": { "name": "XA", "module": "src/x.ts" } });
        assert_eq!(export_name(json!([js("default")])), "default", "Shoelace");
        assert_eq!(
            export_name(json!([js("default"), js("XA")])),
            "XA",
            "own name wins"
        );
        assert_eq!(export_name(json!([])), "XA", "nothing listed");
    }

    #[test]
    fn no_exported_reexport_is_e002() {
        let cem = crate::test_util::cem(barrel_layout()).unwrap();
        let pkg = PackageInfo {
            name: "@lion/ui".into(),
            exports: Some(json!({ "./define/*": "./exports/define/*" })),
            ..PackageInfo::default()
        };
        let err = crate::build(&cem, &pkg, &[], &Config::default()).unwrap_err();
        assert_eq!(err.code, "E002");
        assert_eq!(err.module.as_deref(), Some("components/a/src/XA.js"));
        assert!(err.hint.contains("re-export"));
    }

    #[test]
    fn exclude_drops_matching_tags_but_keeps_them_as_bases() {
        let base = element("internal-base", "Base");
        let mut derived = element("x-a", "XA");
        derived["superclass"] = json!({ "name": "Base" });
        derived["members"] = json!([]);
        let mut cem = cem_one(base);
        cem["modules"][0]["declarations"]
            .as_array_mut()
            .unwrap()
            .push(derived);
        cem["modules"][0]["declarations"][0]["members"] =
            json!([{ "kind": "field", "name": "size" }]);
        let cfg = Config {
            exclude: vec!["internal-*".into()],
            ..Config::default()
        };
        let lib = build_json_with(cem, &cfg).unwrap();
        let tags: Vec<_> = lib.components.iter().map(|c| c.tag.as_str()).collect();
        assert_eq!(tags, ["x-a"]);
        assert_eq!(lib.components[0].props[0].name, "size");
    }

    #[test]
    fn only_detail_types_resolve_imports() {
        let mut el = element("x-a", "XA");
        el["events"] = json!([
            { "name": "a", "type": { "text": "CustomEvent<Detail>",
                "references": [{ "name": "Detail", "module": "src/types.ts" }] } },
            { "name": "b", "type": { "text": "BEvent",
                "references": [{ "name": "BEvent", "module": "src/private.ts" }] } }
        ]);
        let cem = crate::test_util::cem(cem_one(el)).unwrap();
        let pkg = PackageInfo {
            name: "pkg".into(),
            exports: Some(json!({ "./x.js": "./src/x.js", "./types.js": "./src/types.js" })),
            ..PackageInfo::default()
        };
        // `src/private.ts` is not exported, but nothing imports BEvent, so no E002.
        let c = crate::build(&cem, &pkg, &[], &Config::default())
            .unwrap()
            .components
            .remove(0);
        let detail = c.events[0].detail.as_ref().unwrap();
        assert_eq!(detail.imports[0].specifier, "pkg/types.js");
        assert!(c.events[1].event_type.as_ref().unwrap().imports.is_empty());
    }

    #[test]
    fn config_errors_come_before_manifest_errors() {
        let cem = crate::test_util::cem(cem_one(element("x-a", "XA"))).unwrap();
        let pkg = PackageInfo {
            name: "pkg".into(),
            exports: Some(json!({})),
            ..PackageInfo::default()
        };
        let cfg = Config {
            tag_formatter: Some(json!("x")),
            ..Config::default()
        };
        assert_eq!(
            crate::build(&cem, &pkg, &[], &cfg).unwrap_err().code,
            "E010"
        );
    }

    #[test]
    fn definition_module_can_differ_from_declaration() {
        let mut class = element("x-a", "XA");
        class.as_object_mut().unwrap().remove("tagName");
        let cem = crate::test_util::cem(json!({
            "schemaVersion": "1.0.0",
            "modules": [
                { "kind": "javascript-module", "path": "src/x-a.ts", "declarations": [class] },
                { "kind": "javascript-module", "path": "src/define.ts", "exports": [{
                    "kind": "custom-element-definition", "name": "x-a",
                    "declaration": { "name": "XA", "module": "src/x-a.ts" }
                }] }
            ]
        }))
        .unwrap();
        let pkg = PackageInfo {
            name: "pkg".into(),
            ..PackageInfo::default()
        };
        let c = crate::build(&cem, &pkg, &[], &Config::default())
            .unwrap()
            .components
            .remove(0);
        assert_eq!(c.tag, "x-a");
        assert_eq!(c.module.specifier, "pkg/src/x-a.js");
        assert_eq!(c.definition.specifier, "pkg/src/define.js");
    }
}
