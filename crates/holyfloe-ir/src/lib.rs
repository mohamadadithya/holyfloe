//! IR structs and normalization passes. Pure, no I/O.

mod class;
mod config;
mod error;
mod model;
mod module_path;
mod passes;

pub use config::{BindingOverride, BindingsMode, ComponentOverride, Config, PackageInfo};
pub use error::IrError;
pub use model::*;

use passes::*;

/// The manifest of another package that a superclass or mixin comes from (D-069).
#[derive(Clone, Debug)]
pub struct Dependency {
    /// Package name, e.g. `@holyfloe/fixtures-dep`.
    pub name: String,
    pub cem: holyfloe_cem::Package,
}

/// Packages that superclasses and mixins in `cem` come from, sorted. The CLI loads the
/// manifests of those that have one and passes them to `build` as `Dependency`s.
pub fn referenced_packages(cem: &holyfloe_cem::Package) -> Vec<String> {
    use holyfloe_cem::JavaScriptModuleDeclarationsItem as Decl;
    let mut names: Vec<String> = cem
        .modules
        .iter()
        .flat_map(|m| &m.declarations)
        .flat_map(|d| match d {
            Decl::CustomElementDeclaration(d) => d.superclass.iter().chain(&d.mixins).collect(),
            Decl::ClassDeclaration(d) => d.superclass.iter().chain(&d.mixins).collect(),
            Decl::CustomElementMixinDeclaration(d) => {
                d.superclass.iter().chain(&d.mixins).collect()
            }
            _ => Vec::new(),
        })
        .filter_map(|r| r.package.as_deref())
        .map(|p| module_path::package_name(p).to_owned())
        .collect();
    names.sort();
    names.dedup();
    names
}

/// Manifest -> IR, running the six passes of `docs/ir.md` in order.
pub fn build(
    cem: &holyfloe_cem::Package,
    pkg: &PackageInfo,
    deps: &[Dependency],
    cfg: &Config,
) -> Result<Library, IrError> {
    name_derive::check_config(cfg)?;
    let mut classes = class::discover(cem);
    for dep in deps {
        classes.extend(class::discover(&dep.cem).into_iter().map(|mut c| {
            c.package = Some(dep.name.clone());
            // Type references are module paths in the dependency, not in this package.
            // shortcut: imported from the dependency's root, like other `package` references;
            // upgrade to resolve the module through its `exports` when a library needs it.
            for r in c
                .events
                .iter_mut()
                .filter_map(|e| e.type_.as_mut())
                .flat_map(|t| &mut t.references)
            {
                if r.package.is_none() && r.module.take().is_some() {
                    r.package = Some(dep.name.clone());
                }
            }
            c
        }));
    }
    let mut classes = flatten_inheritance::run(&classes)?;
    classes.retain(|c| !c.tag.as_deref().is_some_and(|t| cfg.excludes(t)));
    drop_non_public::run(&mut classes);
    let components = classes
        .into_iter()
        .map(|c| c.lower(pkg, cfg))
        .collect::<Result<_, _>>()?;
    let mut lib = Library {
        package: pkg.clone(),
        components,
    };
    name_derive::run(&mut lib, cfg)?;
    classify_props::run(&mut lib);
    infer_bindings::run(&mut lib, cfg)?;
    validate::run(&lib)?;
    Ok(lib)
}

#[cfg(test)]
pub(crate) mod test_util {
    use serde_json::{Value, json};

    use crate::{Component, Config, IrError, Library, PackageInfo};

    /// Through `holyfloe_cem::parse`, so a malformed test element fails instead of becoming a plain class.
    pub fn cem(v: Value) -> Result<holyfloe_cem::Package, holyfloe_cem::ParseError> {
        holyfloe_cem::parse(&v.to_string())
    }

    /// A minimal custom element declaration.
    pub fn element(tag: &str, name: &str) -> Value {
        json!({ "kind": "class", "name": name, "tagName": tag, "customElement": true })
    }

    /// A manifest with one module holding `decl`.
    pub fn cem_one(decl: Value) -> Value {
        json!({
            "schemaVersion": "1.0.0",
            "modules": [{ "kind": "javascript-module", "path": "src/x.ts", "declarations": [decl] }]
        })
    }

    pub fn build_json_with(cem: Value, cfg: &Config) -> Result<Library, IrError> {
        let cem = self::cem(cem).expect("valid test manifest");
        let pkg = PackageInfo {
            name: "pkg".into(),
            ..PackageInfo::default()
        };
        crate::build(&cem, &pkg, &[], cfg)
    }

    pub fn build_json(cem: Value) -> Result<Library, IrError> {
        build_json_with(cem, &Config::default())
    }

    pub fn build_one(decl: Value) -> Component {
        build_json(cem_one(decl)).unwrap().components.remove(0)
    }
}
