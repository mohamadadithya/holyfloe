//! React wrapper and types emitter. See `docs/emit-react.md`.

use std::collections::{BTreeMap, BTreeSet};

use holyfloe_emit_core::{OutputFile, ts_string};
use holyfloe_ir::{Component, Library};
use minijinja::value::Serde;
use minijinja::{Environment, Value, context};
use serde::Deserialize;

/// The `react` key of `holyfloe.config.json`.
#[derive(Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase", default)]
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

#[derive(Debug, thiserror::Error)]
pub enum EmitError {
    #[error("react template: {0}")]
    Template(#[from] minijinja::Error),
}

const TEMPLATES: [(&str, &str); 5] = [
    ("macros.j2", include_str!("../templates/macros.j2")),
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
/// Bindings are ignored until M2.
pub fn emit(lib: &Library, cfg: &ReactConfig) -> Result<Vec<OutputFile>, EmitError> {
    let mut env = holyfloe_emit_core::env()?;
    env.add_filter("ts_key", |s: &str| ts_key(s));
    for (name, source) in TEMPLATES {
        env.add_template(name, source)?;
    }
    let comps = &lib.components;
    match cfg.mode {
        ReactMode::Wrapper => {
            let mut files = Vec::with_capacity(comps.len() + 2);
            for c in comps {
                let ctx = context! {
                    c => Serde(c),
                    imports => Serde(detail_imports(std::slice::from_ref(c))),
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
                imports => Serde(detail_imports(comps)),
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

/// Types named by event details, grouped by specifier: the one place CEM type text is re-emitted.
fn detail_imports(comps: &[Component]) -> BTreeMap<&str, BTreeSet<&str>> {
    let mut out: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    let events = comps
        .iter()
        .flat_map(|c| &c.events)
        .filter(|e| e.detail.is_some());
    for i in events
        .filter_map(|e| e.event_type.as_ref())
        .flat_map(|t| &t.imports)
    {
        out.entry(&i.specifier).or_default().insert(&i.name);
    }
    out
}

/// An object key: bare when it is an identifier, else a string literal (`'badge-count'`).
fn ts_key(s: &str) -> String {
    let mut chars = s.chars();
    let ident = chars
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_' || c == '$')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '$');
    if ident { s.to_owned() } else { ts_string(s) }
}

#[cfg(test)]
mod tests {
    use super::ts_key;

    #[test]
    fn ts_keys() {
        assert_eq!(ts_key("tone"), "tone");
        assert_eq!(ts_key("onmyChange"), "onmyChange");
        assert_eq!(ts_key("$x_1"), "$x_1");
        assert_eq!(ts_key("badge-count"), "'badge-count'");
        assert_eq!(ts_key("--fx-card-padding"), "'--fx-card-padding'");
        assert_eq!(ts_key("1a"), "'1a'");
        assert_eq!(ts_key(""), "''");
    }
}
