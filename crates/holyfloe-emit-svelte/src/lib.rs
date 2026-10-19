//! Svelte 5 runes emitter. See `docs/emit-svelte.md`.

pub use holyfloe_emit_core::EmitError;
use holyfloe_emit_core::{OutputFile, camel_ident, check_names, detail_imports};
use holyfloe_ir::{Component, Library};
use minijinja::value::Serde;
use minijinja::{Environment, Value, context};

const TEMPLATES: [(&str, &str); 4] = [
    (
        "component.svelte.j2",
        include_str!("../templates/component.svelte.j2"),
    ),
    ("index.ts.j2", include_str!("../templates/index.ts.j2")),
    (
        "svelte-utils.ts.j2",
        include_str!("../templates/svelte-utils.ts.j2"),
    ),
    (
        "svelte-elements.d.ts.j2",
        include_str!("../templates/svelte-elements.d.ts.j2"),
    ),
];

/// Event names Svelte 5.57.2 delegates to the root (`DELEGATED_EVENTS` in `svelte/src/utils.js`).
/// Keep in sync when the svelte pin moves.
const DELEGATED: [&str; 23] = [
    "beforeinput",
    "click",
    "change",
    "dblclick",
    "contextmenu",
    "focusin",
    "focusout",
    "input",
    "keydown",
    "keyup",
    "mousedown",
    "mousemove",
    "mouseout",
    "mouseover",
    "mouseup",
    "pointerdown",
    "pointermove",
    "pointerout",
    "pointerover",
    "pointerup",
    "touchend",
    "touchmove",
    "touchstart",
];

/// `<Name>.svelte` per component, `index.ts`, `svelte-utils.ts` and `svelte-elements.d.ts`.
pub fn emit(lib: &Library) -> Result<Vec<OutputFile>, EmitError> {
    let mut env = holyfloe_emit_core::env()?;
    env.add_filter("svelte_on", |name: &str| format!("on{name}"));
    env.add_test("svelte_capture", |dom: &str| capture(dom));
    for (name, source) in TEMPLATES {
        env.add_template(name, source)?;
    }
    let comps = &lib.components;
    let mut files = Vec::with_capacity(comps.len() + 3);
    for c in comps {
        let snippets = snippet_props(c);
        check_names(c, &generated_names(c, &snippets))?;
        let ctx = context! {
            c => Serde(c),
            snippets => snippets.iter().map(|(slot, prop)| context! { slot => slot, prop => prop }).collect::<Vec<_>>(),
            imports => Serde(detail_imports(std::slice::from_ref(c), &component_names(c))?),
        };
        files.push(file(
            &env,
            &format!("{}.svelte", c.name),
            "component.svelte.j2",
            ctx,
        )?);
    }
    let ctx = context! { components => Serde(comps) };
    files.push(file(&env, "index.ts", "index.ts.j2", ctx)?);
    files.push(file(
        &env,
        "svelte-utils.ts",
        "svelte-utils.ts.j2",
        context! {},
    )?);
    let ctx = context! {
        components => Serde(comps),
        imports => Serde(detail_imports(comps, &types_names(comps))?),
    };
    files.push(file(
        &env,
        "svelte-elements.d.ts",
        "svelte-elements.d.ts.j2",
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

/// Listen with `on<name>capture`: a delegated name only reaches the root if the event bubbles,
/// and a name ending in `capture` would be read as the capture flag (D-075).
fn capture(dom: &str) -> bool {
    DELEGATED.contains(&dom) || dom.ends_with("capture")
}

/// (slot name, snippet prop) for each named slot. The prop is the slot name as an identifier
/// (`help-text` -> `helpText`), with `Snippet` appended when that is already a prop or another
/// name the wrapper declares: Shoelace has a `label` prop and a `label` slot (D-081).
fn snippet_props(c: &Component) -> Vec<(String, String)> {
    let mut used: Vec<String> = c
        .props
        .iter()
        .filter(|p| !p.readonly)
        .map(|p| p.name.clone())
        .collect();
    used.extend(["element", "children", "rest"].map(String::from));
    used.extend(c.events.iter().map(|e| format!("on{}", e.base_name)));
    let mut out = Vec::new();
    for s in c.slots.iter().filter(|s| !s.name.is_empty()) {
        let base = camel_ident(&s.name, &[]);
        let prop = if base.ends_with('_') || used.contains(&base) {
            format!("{}Snippet", base.trim_end_matches('_'))
        } else {
            base
        };
        used.push(prop.clone());
        out.push((s.name.clone(), prop));
    }
    out
}

/// Props the wrapper declares next to the element's own (E013).
fn generated_names(c: &Component, snippets: &[(String, String)]) -> Vec<(String, String)> {
    let mut names = vec![
        ("element".to_owned(), "the wrapper's `element`".to_owned()),
        (
            "children".to_owned(),
            "the default slot snippet `children`".to_owned(),
        ),
    ];
    for e in &c.events {
        let what = format!("the Svelte callback prop for event `{}`", e.dom_name);
        names.push((format!("on{}", e.base_name), what));
    }
    for (slot, prop) in snippets {
        names.push((prop.clone(), format!("the snippet for slot `{slot}`")));
    }
    names
}

/// Names `component.svelte.j2` declares or imports. Keep in sync with the template.
fn component_names(c: &Component) -> Vec<String> {
    let mut names: Vec<String> = ["Snippet", "HTMLAttributes", "TypedEvent"]
        .map(String::from)
        .into();
    names.extend([format!("{}Props", c.name), format!("{}Element", c.name)]);
    names
}

/// Names `svelte-elements.d.ts.j2` declares or imports. Keep in sync with the template.
fn types_names(comps: &[Component]) -> Vec<String> {
    let mut names = vec!["HTMLAttributes".to_owned(), "TypedEvent".to_owned()];
    names.extend(comps.iter().map(|c| format!("{}Element", c.name)));
    names
}

#[cfg(test)]
mod tests {
    use super::capture;

    #[test]
    fn capture_names() {
        assert!(capture("change"));
        assert!(capture("click"));
        assert!(capture("screen-capture"));
        assert!(!capture("fx-change"));
        assert!(!capture("Change"));
        assert!(!capture("gotpointer"));
    }
}
