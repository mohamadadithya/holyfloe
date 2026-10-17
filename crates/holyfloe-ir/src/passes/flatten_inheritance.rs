//! Pass 1: merge superclass members, events, slots, parts and CSS from the same package.
//! The analyzer already inlines inherited members (with `inheritedFrom`), so only missing
//! names are added and own entries win (OQ-015).

use std::collections::HashMap;

use crate::IrError;
use crate::class::Class;
use crate::module_path::normalize;

/// Returns the flattened components; bases without a tag are consumed here.
pub(crate) fn run(classes: &[Class]) -> Result<Vec<Class>, IrError> {
    let index: HashMap<(&str, &str), &Class> = classes
        .iter()
        .map(|c| ((normalize(&c.module), c.name.as_str()), c))
        .collect();
    classes
        .iter()
        .filter(|c| c.tag.is_some())
        .map(|c| flatten(c, &index))
        .collect()
}

fn flatten(class: &Class, index: &HashMap<(&str, &str), &Class>) -> Result<Class, IrError> {
    let mut out = class.clone();
    let mut chain = vec![key(class)];
    let mut current = class;
    while let Some(base) = superclass(current, index) {
        if chain.contains(&key(base)) {
            let names: Vec<_> = chain.iter().map(|(_, n)| *n).collect();
            return Err(IrError::new(
                "E003",
                format!("inheritance cycle: {} -> {}", names.join(" -> "), base.name),
                class.tag.as_deref(),
                Some(&class.module),
                "fix the `superclass` entries in the manifest; a class cannot extend itself",
            ));
        }
        chain.push(key(base));
        merge(&mut out.members, &base.members, |m| &m.name);
        merge(&mut out.events, &base.events, |e| &e.name);
        merge(&mut out.slots, &base.slots, |s| &s.name);
        merge(&mut out.css_parts, &base.css_parts, |p| &p.name);
        merge(&mut out.css_props, &base.css_props, |p| &p.name);
        merge(&mut out.css_states, &base.css_states, |s| &s.name);
        current = base;
    }
    Ok(out)
}

fn key(c: &Class) -> (&str, &str) {
    (normalize(&c.module), &c.name)
}

fn superclass<'a>(c: &Class, index: &HashMap<(&str, &str), &'a Class>) -> Option<&'a Class> {
    let sup = c.superclass.as_ref()?;
    // shortcut: superclasses from other packages (LitElement, dependency manifests) are not
    // merged; cross-package flatten comes with fx-mixin in M2.
    if sup.package.is_some() {
        return None;
    }
    let module = sup.module.as_deref().unwrap_or(&c.module);
    index.get(&(normalize(module), sup.name.as_str())).copied()
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
        // spelled `/src/base.js` while the base lives at `src/base.ts` (OQ-015).
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
        // FxBase-style class without a tag is a base, not a component (OQ-015).
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
}
