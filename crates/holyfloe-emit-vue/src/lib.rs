//! Vue 3 `defineComponent` wrapper emitter. See `docs/emit-vue.md`.

pub use holyfloe_emit_core::EmitError;
use holyfloe_emit_core::{OutputFile, camel_ident, check_names, detail_imports};
use holyfloe_ir::{Component, Event, Library};
use minijinja::value::Serde;
use minijinja::{Environment, Value, context};

const TEMPLATES: [(&str, &str); 4] = [
    (
        "component.ts.j2",
        include_str!("../templates/component.ts.j2"),
    ),
    ("index.ts.j2", include_str!("../templates/index.ts.j2")),
    (
        "vue-utils.ts.j2",
        include_str!("../templates/vue-utils.ts.j2"),
    ),
    (
        "global-components.d.ts.j2",
        include_str!("../templates/global-components.d.ts.j2"),
    ),
];

/// `<Name>.ts` per component, `index.ts`, `vue-utils.ts` and `global-components.d.ts`.
pub fn emit(lib: &Library) -> Result<Vec<OutputFile>, EmitError> {
    let mut env = holyfloe_emit_core::env()?;
    env.add_test("vue_raw_event", |dom: &str| vue_raw_listener(dom).is_some());
    env.add_filter("vue_raw_listener", |dom: &str| {
        vue_raw_listener(dom).unwrap_or_default()
    });
    for (name, source) in TEMPLATES {
        env.add_template(name, source)?;
    }
    let comps = &lib.components;
    let mut files = Vec::with_capacity(comps.len() + 3);
    for c in comps {
        check_names(c, &generated_names(c))?;
        let late: Vec<&Event> = c
            .events
            .iter()
            .filter(|e| late_event(&e.dom_name))
            .collect();
        let ctx = context! {
            c => Serde(c),
            imports => Serde(detail_imports(std::slice::from_ref(c), &component_names(c))?),
            late => Serde(&late),
            late_names => late.iter().map(|e| e.dom_name.clone()).collect::<Vec<_>>(),
            slots => slot_locals(c),
        };
        files.push(file(
            &env,
            &format!("{}.ts", c.name),
            "component.ts.j2",
            ctx,
        )?);
    }
    let ctx = context! { components => Serde(comps) };
    files.push(file(&env, "index.ts", "index.ts.j2", ctx)?);
    files.push(file(&env, "vue-utils.ts", "vue-utils.ts.j2", context! {})?);
    let ctx = context! {
        components => Serde(comps),
        imports => Serde(detail_imports(comps, &types_names(comps))?),
    };
    files.push(file(
        &env,
        "global-components.d.ts",
        "global-components.d.ts.j2",
        ctx,
    )?);
    Ok(files)
}

fn file(
    env: &Environment,
    path: &str,
    template: &str,
    ctx: Value,
) -> Result<OutputFile, EmitError> {
    Ok(OutputFile {
        path: path.to_owned(),
        contents: env.get_template(template)?.render(ctx)?,
    })
}

/// Named slots and the locals `render()` destructures them into: `help-text` -> `helpText`
/// (`{ 'help-text': helpText } = this.$slots`). `body` holds the default slot.
fn slot_locals(c: &Component) -> Vec<Value> {
    let taken = ["body", "h", "defined"].map(String::from);
    c.slots
        .iter()
        .filter(|s| !s.name.is_empty())
        .map(|s| context! { slot => Serde(s), local => camel_ident(&s.name, &taken) })
        .collect()
}

/// Names the wrapper puts on the component instance next to the props (E013).
fn generated_names(c: &Component) -> Vec<(String, String)> {
    let mut names = vec![("element".to_owned(), "the wrapper's `element`".to_owned())];
    for e in &c.events {
        let what = format!("the Vue listener prop for event `{}`", e.dom_name);
        names.push((format!("on{}", e.base_name), what));
    }
    if c.bindings.iter().any(|b| b.prop == "value") {
        names.push((
            "modelValue".to_owned(),
            "the `v-model` prop `modelValue`".to_owned(),
        ));
    }
    for m in &c.methods {
        let what = format!("the forwarded method `{}`", m.name);
        names.push((m.name.clone(), what));
    }
    names
}

/// Names `component.ts.j2` declares or imports. Keep in sync with the template.
fn component_names(c: &Component) -> Vec<String> {
    let mut names: Vec<String> = [
        "defineComponent",
        "h",
        "shallowRef",
        "PropType",
        "SlotsType",
        "VNode",
        "defined",
        "TypedEvent",
    ]
    .map(String::from)
    .into();
    names.extend([c.name.clone(), format!("{}Element", c.name)]);
    names
}

/// Names `global-components.d.ts.j2` declares or imports. Keep in sync with the template.
fn types_names(comps: &[Component]) -> Vec<String> {
    let mut names = vec!["DefineComponent".to_owned(), "TypedEvent".to_owned()];
    names.extend(comps.iter().map(|c| format!("{}Element", c.name)));
    names
}

/// Vue strips a trailing `Once`, `Passive` or `Capture` from a listener key as an option, even
/// after `on:` (runtime-dom 3.5.44 `parseName`), so such events cannot use an `'on:<name>'` key.
fn late_event(dom: &str) -> bool {
    ["Once", "Passive", "Capture"]
        .iter()
        .any(|s| dom.ends_with(s) && dom.len() > s.len())
}

/// The prop a raw-tag listener `@<dom>` becomes in a Vue template (`fx-press` -> `onFxPress`),
/// when Vue maps it back to `<dom>` at runtime: Vue hyphenates the name again, so only lowercase
/// and kebab-case names survive (`myChange` would listen to `my-change`).
fn vue_raw_listener(dom: &str) -> Option<String> {
    let camel = camelize(dom);
    if hyphenate(&camel) != dom || late_event(&camel) {
        return None;
    }
    let mut chars = camel.chars();
    let first = chars.next()?;
    Some(format!(
        "on{}{}",
        first.to_ascii_uppercase(),
        chars.as_str()
    ))
}

/// Vue's `camelize`: `-(\w)` -> the character, uppercased.
fn camelize(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        match chars.peek() {
            Some(&n) if c == '-' && (n.is_ascii_alphanumeric() || n == '_') => {
                out.push(n.to_ascii_uppercase());
                chars.next();
            }
            _ => out.push(c),
        }
    }
    out
}

/// Vue's `hyphenate`: `\B([A-Z])` -> `-$1`, then lowercase.
fn hyphenate(s: &str) -> String {
    let word = |c: char| c.is_ascii_alphanumeric() || c == '_';
    let mut out = String::with_capacity(s.len() + 4);
    let mut prev: Option<char> = None;
    for c in s.chars() {
        if c.is_ascii_uppercase() && prev.is_some_and(word) {
            out.push('-');
        }
        out.push(c.to_ascii_lowercase());
        prev = Some(c);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_listeners() {
        assert_eq!(vue_raw_listener("fx-press").as_deref(), Some("onFxPress"));
        assert_eq!(vue_raw_listener("change").as_deref(), Some("onChange"));
        assert_eq!(vue_raw_listener("my-change").as_deref(), Some("onMyChange"));
        for dom in ["myChange", "MyChange", "MYEVENT", "fx--x"] {
            assert_eq!(vue_raw_listener(dom), None, "{dom}");
        }
        assert_eq!(
            vue_raw_listener("drag-capture"),
            None,
            "camelizes to dragCapture"
        );
    }

    #[test]
    fn late_events() {
        assert!(late_event("dragOnce"));
        assert!(late_event("zoomCapture"));
        assert!(!late_event("Once"));
        assert!(!late_event("fx-once"));
    }
}
