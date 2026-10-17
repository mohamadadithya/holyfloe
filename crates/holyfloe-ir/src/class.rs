//! Working representation between CEM and the IR. Flatten and drop-non-public need
//! superclass links, privacy and `static`, which the IR does not keep.

use std::collections::HashMap;

use holyfloe_cem as cem;

use crate::model::*;
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
    pub module: String,
    pub tag: Option<String>,
    /// Module holding the `custom-element-definition` export, when the manifest has one.
    pub definition_module: Option<String>,
    pub superclass: Option<cem::Reference>,
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
/// (e.g. `FxBase`) is a base class (OQ-015).
pub(crate) fn discover(pkg: &cem::Package) -> Vec<Class> {
    // (declaring module, class name) -> (tag, module that defines it)
    let mut tags: HashMap<(&str, &str), (&str, &str)> = HashMap::new();
    for module in &pkg.modules {
        for export in &module.exports {
            if let cem::JavaScriptModuleExportsItem::CustomElementExport(e) = export {
                let decl_module = e.declaration.module.as_deref().unwrap_or(&module.path);
                tags.insert(
                    (module_path::normalize(decl_module), &e.declaration.name),
                    (&e.name, &module.path),
                );
            }
        }
    }
    let mut classes = Vec::new();
    for module in &pkg.modules {
        for decl in &module.declarations {
            let mut class = match decl {
                cem::JavaScriptModuleDeclarationsItem::CustomElementDeclaration(d) => {
                    from_element(d, &module.path)
                }
                cem::JavaScriptModuleDeclarationsItem::ClassDeclaration(d) => {
                    from_class(d, &module.path)
                }
                // shortcut: mixins are not flattened yet; fx-mixin brings them in M2.
                _ => continue,
            };
            let key = (module_path::normalize(&module.path), class.name.as_str());
            if let Some((tag, definition)) = tags.get(&key) {
                class.tag.get_or_insert_with(|| (*tag).to_owned());
                class.definition_module = Some((*definition).to_owned());
            }
            classes.push(class);
        }
    }
    classes
}

fn from_element(d: &cem::CustomElementDeclaration, module: &str) -> Class {
    let members = d
        .members
        .iter()
        .map(|m| match m {
            cem::CustomElementDeclarationMembersItem::CustomElementField(f) => Member {
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
            },
            cem::CustomElementDeclarationMembersItem::ClassMethod(m) => method(m),
        })
        .collect();
    Class {
        name: d.name.clone(),
        module: module.to_owned(),
        tag: d.tag_name.clone(),
        definition_module: None,
        superclass: d.superclass.clone(),
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
        module: module.to_owned(),
        tag: None,
        definition_module: None,
        superclass: d.superclass.clone(),
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
        let module_ref = |path: String| {
            module_path::specifier(&path, pkg, cfg)
                .map(|specifier| ModuleRef {
                    path: path.clone(),
                    specifier,
                })
                .map_err(|file| unexported(&file, Some(&tag), &path))
        };
        // A separate `define.ts` registers the element; importing the class module would not.
        let definition = module_ref(
            self.definition_module
                .unwrap_or_else(|| self.module.clone()),
        )?;
        let module = module_ref(self.module)?;
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
                let imports = type_imports(&e.type_, pkg, cfg, &tag)?;
                Ok(Event {
                    deprecated: deprecated!(e.deprecated, EventDeprecated),
                    dom_name: e.name,
                    base_name: String::new(),
                    detail: None,
                    event_type: Some(TypeRef {
                        text: e.type_.text,
                        imports,
                    }),
                    description: e.description,
                })
            })
            .collect::<Result<_, IrError>>()?;
        Ok(Component {
            name: String::new(),
            class_name: self.name,
            module,
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
            // Not in CEM. Config flag or `static formAssociated` detection come with fx-input (M2).
            form_associated: false,
            bindings: Vec::new(),
            tag,
        })
    }
}

/// References without a module or package are globals and need no import.
fn type_imports(
    ty: &cem::Type,
    pkg: &PackageInfo,
    cfg: &Config,
    tag: &str,
) -> Result<Vec<TypeImport>, IrError> {
    let mut imports = Vec::new();
    for r in &ty.references {
        let specifier = match (&r.package, &r.module) {
            // shortcut: imports from the package root; cross-package resolution comes in M2.
            (Some(package), _) => package.clone(),
            (None, Some(m)) => module_path::specifier(m, pkg, cfg)
                .map_err(|file| unexported(&file, Some(tag), m))?,
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
            "export `./{file}` in package.json, or map the source path with `moduleRewrite` in holyfloe.config.json"
        ),
    )
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::test_util::{cem_one, element};
    use crate::{Config, PackageInfo};

    #[test]
    fn unexported_module_is_e002() {
        let cem = serde_json::from_value(cem_one(element("x-a", "XA"))).unwrap();
        let pkg = PackageInfo {
            name: "pkg".into(),
            exports: Some(json!({ ".": "./dist/index.js" })),
            ..PackageInfo::default()
        };
        let err = crate::build(&cem, &pkg, &Config::default()).unwrap_err();
        assert_eq!(err.code, "E002");
        assert_eq!(err.tag.as_deref(), Some("x-a"));
        assert_eq!(err.module.as_deref(), Some("src/x.ts"));
        assert!(err.hint.contains("moduleRewrite"));
    }

    #[test]
    fn definition_module_can_differ_from_declaration() {
        let mut class = element("x-a", "XA");
        class.as_object_mut().unwrap().remove("tagName");
        let cem = serde_json::from_value(json!({
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
        let c = crate::build(&cem, &pkg, &Config::default())
            .unwrap()
            .components
            .remove(0);
        assert_eq!(c.tag, "x-a");
        assert_eq!(c.module.specifier, "pkg/src/x-a.js");
        assert_eq!(c.definition.specifier, "pkg/src/define.js");
    }
}
