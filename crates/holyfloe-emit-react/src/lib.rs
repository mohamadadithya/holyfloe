//! React wrapper and types emitter. See `docs/emit-react.md`.

pub use holyfloe_emit_core::EmitError;
use holyfloe_emit_core::{OutputFile, check_names, detail_imports};
use holyfloe_ir::{Component, Library};
use minijinja::value::Serde;
use minijinja::{Environment, Value, context};
use serde::Deserialize;

/// The `react` key of `holyfloe.config.json`. Unknown keys are errors, so a typo or a key that
/// is not supported yet (`slotHelpers`, `attributeMapping`) fails instead of doing nothing.
#[derive(Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct ReactConfig {
    pub mode: ReactMode,
    /// Emit `'use client';` first in each wrapper.
    pub use_client: bool,
    #[serde(rename = "ref")]
    pub ref_style: RefStyle,
}

impl Default for ReactConfig {
    fn default() -> Self {
        Self {
            mode: ReactMode::default(),
            use_client: true,
            ref_style: RefStyle::default(),
        }
    }
}

#[derive(Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ReactMode {
    /// One component per element, React 18 and 19.
    #[default]
    Wrapper,
    /// A JSX `IntrinsicElements` augmentation only, React 19.
    Types,
}

#[derive(Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum RefStyle {
    /// `forwardRef`, works on React 18 and 19.
    #[default]
    ForwardRef,
    /// `ref` as a plain prop, React 19 only.
    Prop,
}

const TEMPLATES: [(&str, &str); 4] = [
    (
        "component.tsx.j2",
        include_str!("../templates/component.tsx.j2"),
    ),
    ("index.ts.j2", include_str!("../templates/index.ts.j2")),
    (
        "react-utils.ts.j2",
        include_str!("../templates/react-utils.ts.j2"),
    ),
    ("types.d.ts.j2", include_str!("../templates/types.d.ts.j2")),
];

/// Wrapper mode: `<Name>.tsx` per component, `index.ts`, `react-utils.ts`. Types mode: `react-jsx.d.ts`.
/// React has no two-way binding idiom, so `bindings` are not emitted (D-067).
pub fn emit(lib: &Library, cfg: &ReactConfig) -> Result<Vec<OutputFile>, EmitError> {
    let mut env = holyfloe_emit_core::env()?;
    for (name, source) in TEMPLATES {
        env.add_template(name, source)?;
    }
    let comps = &lib.components;
    match cfg.mode {
        ReactMode::Wrapper => {
            let mut files = Vec::with_capacity(comps.len() + 2);
            for c in comps {
                let events = c.events.iter().map(|e| {
                    let what = format!("the React prop for event `{}`", e.dom_name);
                    (format!("on{}", e.base_name), what)
                });
                check_names(c, &events.collect::<Vec<_>>())?;
                let own = wrapper_names(c);
                let ctx = context! {
                    c => Serde(c),
                    imports => Serde(detail_imports(std::slice::from_ref(c), &own)?),
                    use_client => cfg.use_client,
                    forward_ref => cfg.ref_style == RefStyle::ForwardRef,
                };
                files.push(file(
                    &env,
                    &format!("{}.tsx", c.name),
                    "component.tsx.j2",
                    ctx,
                )?);
            }
            let ctx = context! { components => Serde(comps) };
            files.push(file(&env, "index.ts", "index.ts.j2", ctx)?);
            files.push(file(
                &env,
                "react-utils.ts",
                "react-utils.ts.j2",
                context! {},
            )?);
            Ok(files)
        }
        ReactMode::Types => {
            let ctx = context! {
                components => Serde(comps),
                imports => Serde(detail_imports(comps, &types_names(comps))?),
            };
            Ok(vec![file(&env, "react-jsx.d.ts", "types.d.ts.j2", ctx)?])
        }
    }
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

/// Names `component.tsx.j2` declares or imports, whether or not this file needs them all.
/// Keep in sync with the template.
fn wrapper_names(c: &Component) -> Vec<String> {
    let mut names: Vec<String> = [
        "createElement",
        "forwardRef",
        "useImperativeHandle",
        "useRef",
        "CSSProperties",
        "HTMLAttributes",
        "Ref",
        "useEventListener",
        "useProperties",
        "TypedEvent",
    ]
    .map(String::from)
    .into();
    names.extend(["Element", "Props", "EventProps", "Style"].map(|s| format!("{}{s}", c.name)));
    names
}

/// Names `types.d.ts.j2` declares or imports, whether or not this file needs them all.
/// Keep in sync with the template.
fn types_names(comps: &[Component]) -> Vec<String> {
    let mut names: Vec<String> = [
        "CSSProperties",
        "DetailedHTMLProps",
        "HTMLAttributes",
        "TypedEvent",
    ]
    .map(String::from)
    .into();
    for c in comps {
        names.extend(["Element", "Props"].map(|s| format!("{}{s}", c.name)));
    }
    names
}
