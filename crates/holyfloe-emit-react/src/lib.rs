//! React wrapper and types emitter. See `docs/emit-react.md`.

use std::collections::{BTreeMap, BTreeSet};

use holyfloe_emit_core::{OutputFile, ts_string};
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

#[derive(Debug, thiserror::Error)]
pub enum EmitError {
    #[error("react template: {0}")]
    Template(#[from] minijinja::Error),
    /// Detail type text is re-emitted verbatim, so an imported name cannot be renamed.
    #[error("event detail type `{name}` of `{tag}` {clash}")]
    DetailImportClash {
        tag: String,
        name: String,
        clash: String,
    },
    /// A prop such as an `onChange` callback and the `on<base_name>` prop of an event would be
    /// the same destructured binding.
    #[error("prop `{prop}` of `{tag}` has the same name as the React prop for event `{event}`")]
    PropEventClash {
        tag: String,
        prop: String,
        event: String,
    },
}

impl EmitError {
    /// The `E0xx` code the CLI prints, continuing the `IrError` table in `docs/ir.md`.
    /// `None` for template errors, which are holyfloe bugs rather than input a user can fix.
    pub fn code(&self) -> Option<&'static str> {
        match self {
            Self::Template(_) => None,
            Self::DetailImportClash { .. } => Some("E012"),
            Self::PropEventClash { .. } => Some("E013"),
        }
    }

    pub fn hint(&self) -> Option<String> {
        match self {
            Self::Template(_) => None,
            Self::DetailImportClash { .. } => {
                Some("rename the type in the library, or exclude the event's component".to_owned())
            }
            Self::PropEventClash { tag, .. } => Some(format!(
                "rename the event with `overrides.{tag}.events`, or drop the prop with `overrides.{tag}.excludeProps`"
            )),
        }
    }
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
                prop_event_clash(c)?;
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

fn prop_event_clash(c: &Component) -> Result<(), EmitError> {
    let props = c.props.iter().filter(|p| !p.readonly);
    for p in props {
        if let Some(e) = c
            .events
            .iter()
            .find(|e| format!("on{}", e.base_name) == p.name)
        {
            return Err(EmitError::PropEventClash {
                tag: c.tag.clone(),
                prop: p.name.clone(),
                event: e.dom_name.clone(),
            });
        }
    }
    Ok(())
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

/// Types named by event details, grouped by specifier: the one place CEM type text is re-emitted.
/// A name imported from two modules, or one the file already uses (`own`), would not compile.
fn detail_imports<'a>(
    comps: &'a [Component],
    own: &[String],
) -> Result<BTreeMap<&'a str, BTreeSet<&'a str>>, EmitError> {
    let mut out: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    let mut seen: BTreeMap<&str, &str> = BTreeMap::new();
    for c in comps {
        let events = c.events.iter().filter(|e| e.detail.is_some());
        for i in events
            .filter_map(|e| e.event_type.as_ref())
            .flat_map(|t| &t.imports)
        {
            let clash = |clash: String| EmitError::DetailImportClash {
                tag: c.tag.clone(),
                name: i.name.clone(),
                clash,
            };
            if own.contains(&i.name) {
                return Err(clash("is reserved by the generated file".into()));
            }
            match seen.insert(&i.name, &i.specifier) {
                Some(prev) if prev != i.specifier => {
                    return Err(clash(format!(
                        "is imported from both `{prev}` and `{}`",
                        i.specifier
                    )));
                }
                _ => {}
            }
            out.entry(&i.specifier).or_default().insert(&i.name);
        }
    }
    Ok(out)
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
