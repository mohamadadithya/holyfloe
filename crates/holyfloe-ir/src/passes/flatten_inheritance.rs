//! Pass 1: merge superclass and mixin members, events, slots, parts, CSS and `@model` bindings,
//! from this package or a dependency manifest (D-069). The analyzer already inlines inherited
//! members (with `inheritedFrom`), so only missing names are added: own entries win, then the
//! class's mixins (outermost first), then the superclass and its mixins (D-055).

use std::collections::HashMap;

use crate::IrError;
use crate::class::{Base, Class, Key, key};

type Global = (Option<String>, Key);

/// `classes` holds this package's classes and the dependencies' (`package: Some`). Returns this
/// package's components; bases, mixins and dependency classes are consumed here.
pub(crate) fn run(classes: &[Class]) -> Result<Vec<Class>, IrError> {
    let index: HashMap<Global, &Class> = classes.iter().map(|c| (global(c), c)).collect();
    let resolve = |base: &Base, from: &Class| match base {
        Base::Local(k) => index.get(&(from.package.clone(), k.clone())).copied(),
        Base::Package { package, name } => classes
            .iter()
            .find(|c| c.package.as_deref() == Some(package) && c.name == *name),
    };
    classes
        .iter()
        .filter(|c| c.tag.is_some() && c.package.is_none())
        .map(|c| flatten(c, &resolve))
        .collect()
}

fn global(c: &Class) -> Global {
    (c.package.clone(), key(&c.module, &c.name))
}

fn flatten<'a>(
    class: &'a Class,
    resolve: &impl Fn(&Base, &Class) -> Option<&'a Class>,
) -> Result<Class, IrError> {
    let mut out = class.clone();
    let mut chain = vec![global(class)];
    let mut current = class;
    loop {
        // shortcut: mixins applied inside a mixin are not followed; upgrade when a library needs it.
        for mixin in current
            .mixins
            .iter()
            .rev()
            .filter_map(|m| resolve(m, current))
        {
            merge_all(&mut out, mixin);
        }
        let Some(base) = current
            .superclass
            .as_ref()
            .and_then(|b| resolve(b, current))
        else {
            break;
        };
        if chain.contains(&global(base)) {
            let names: Vec<_> = chain.iter().map(|(_, (_, n))| n.as_str()).collect();
            return Err(IrError::new(
                "E003",
                format!("inheritance cycle: {} -> {}", names.join(" -> "), base.name),
                class.tag.as_deref(),
                Some(&class.module),
                "fix the `superclass` entries in the manifest; a class cannot extend itself",
            ));
        }
        chain.push(global(base));
        merge_all(&mut out, base);
        current = base;
    }
    Ok(out)
}

fn merge_all(out: &mut Class, base: &Class) {
    merge(&mut out.members, &base.members, |m| &m.name);
    merge(&mut out.events, &base.events, |e| &e.name);
    merge(&mut out.slots, &base.slots, |s| &s.name);
    merge(&mut out.css_parts, &base.css_parts, |p| &p.name);
    merge(&mut out.css_props, &base.css_props, |p| &p.name);
    merge(&mut out.css_states, &base.css_states, |s| &s.name);
    merge(&mut out.models, &base.models, |m| &m.prop);
}

fn merge<T: Clone>(own: &mut Vec<T>, base: &[T], name: impl Fn(&T) -> &String) {
    let missing: Vec<T> = base
        .iter()
        .filter(|b| !own.iter().any(|o| name(o) == name(b)))
        .cloned()
        .collect();
    own.extend(missing);
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::test_util::{build_json, element};

    #[test]
    fn inherited_members_are_not_duplicated() {
        // The analyzer inlines `size` with `inheritedFrom`, and the superclass module is
        // spelled `/src/base.js` while the base lives at `src/base.ts` (D-055).
        let base = json!({
            "kind": "class", "name": "Base", "customElement": true,
            "superclass": { "name": "LitElement", "package": "lit" },
            "members": [
                { "kind": "field", "name": "size", "type": { "text": "string" } },
                { "kind": "field", "name": "hidden2", "type": { "text": "string" }, "description": "from base" }
            ],
            "events": [{ "name": "ready", "type": { "text": "CustomEvent" } }],
            "slots": [{ "name": "icon" }]
        });
        let mut derived = element("x-derived", "Derived");
        derived["superclass"] = json!({ "name": "Base", "module": "/src/base.js" });
        derived["members"] = json!([
            { "kind": "field", "name": "size", "type": { "text": "string" }, "description": "own",
              "inheritedFrom": { "name": "Base", "module": "src/base.ts" } }
        ]);
        let lib = build_json(json!({
            "schemaVersion": "1.0.0",
            "modules": [
                { "kind": "javascript-module", "path": "src/base.ts", "declarations": [base] },
                { "kind": "javascript-module", "path": "src/derived.ts", "declarations": [derived] }
            ]
        }))
        .unwrap();
        // FxBase-style class without a tag is a base, not a component (D-055).
        assert_eq!(lib.components.len(), 1);
        let c = &lib.components[0];
        let props: Vec<_> = c
            .props
            .iter()
            .map(|p| (p.name.as_str(), p.description.as_deref()))
            .collect();
        assert_eq!(
            props,
            [("size", Some("own")), ("hidden2", Some("from base"))]
        );
        assert_eq!(c.events.len(), 1);
        assert_eq!(c.slots[0].name, "icon");
    }

    #[test]
    fn cycle_is_e003() {
        let mut a = element("x-a", "A");
        a["superclass"] = json!({ "name": "B" });
        let mut b = element("x-b", "B");
        b["superclass"] = json!({ "name": "A" });
        let err = build_json(json!({
            "schemaVersion": "1.0.0",
            "modules": [{ "kind": "javascript-module", "path": "src/a.ts", "declarations": [a, b] }]
        }))
        .unwrap_err();
        assert_eq!(err.code, "E003");
        assert_eq!(err.tag.as_deref(), Some("x-a"));
    }

    fn mixin(name: &str, member: &str, event: &str) -> serde_json::Value {
        json!({
            "kind": "mixin", "name": name,
            "members": [{ "kind": "field", "name": member, "type": { "text": "boolean" },
                          "attribute": member, "reflects": true }],
            "events": [{ "name": event, "type": { "text": "CustomEvent" } }]
        })
    }

    #[test]
    fn mixins_merge_from_this_package_and_dependencies() {
        let mut el = element("x-m", "M");
        el["members"] = json!([{ "kind": "field", "name": "own", "type": { "text": "string" } }]);
        el["mixins"] = json!([
            { "name": "LocalMixin", "module": "src/local.ts" },
            { "name": "FocusableMixin", "package": "@dep/pkg/focusable-mixin.js" }
        ]);
        el["superclass"] = json!({ "name": "LitElement", "package": "lit" });
        let manifest = json!({
            "schemaVersion": "1.0.0",
            "modules": [
                { "kind": "javascript-module", "path": "src/local.ts",
                  "declarations": [mixin("LocalMixin", "local", "local-change")] },
                { "kind": "javascript-module", "path": "src/m.ts", "declarations": [el] }
            ]
        });
        let cem = crate::test_util::cem(manifest).unwrap();
        assert_eq!(crate::referenced_packages(&cem), ["@dep/pkg", "lit"]);
        let dep = crate::Dependency {
            name: "@dep/pkg".into(),
            cem: crate::test_util::cem(crate::test_util::cem_one(mixin(
                "FocusableMixin",
                "focused",
                "focus-change",
            )))
            .unwrap(),
        };
        let pkg = crate::PackageInfo {
            name: "pkg".into(),
            ..crate::PackageInfo::default()
        };
        let lib = crate::build(&cem, &pkg, &[dep], &crate::Config::default()).unwrap();
        // Mixins are not components; the dependency's classes are not either.
        assert_eq!(lib.components.len(), 1);
        let c = &lib.components[0];
        let props: Vec<_> = c
            .props
            .iter()
            .map(|p| (p.name.as_str(), p.reflects))
            .collect();
        // Own first, then the outermost mixin (the dependency), then the inner one.
        assert_eq!(props, [("own", false), ("focused", true), ("local", true)]);
        let events: Vec<_> = c.events.iter().map(|e| e.dom_name.as_str()).collect();
        assert_eq!(events, ["focus-change", "local-change"]);
        // Without the dependency manifest its members are missing, not an error.
        let lib = crate::build(&cem, &pkg, &[], &crate::Config::default()).unwrap();
        assert_eq!(lib.components[0].props.len(), 2);
    }
}
