//! Pass 5: two-way bindings. The heuristic in `docs/ir.md` first (off with
//! `bindings: "explicit-only"`), then `@model` tags (lowered into `bindings` with
//! `inferred: false`), then config `overrides.<tag>.bindings`. A later source replaces an
//! earlier binding on the same prop (D-062).

use crate::{BindingsMode, Component, Config, IrError, Library, TwoWayBinding, ValuePath};

const PROPS: [&str; 4] = ["value", "checked", "open", "selected"];

pub(crate) fn run(lib: &mut Library, cfg: &Config) -> Result<(), IrError> {
    for c in &mut lib.components {
        let jsdoc = std::mem::take(&mut c.bindings);
        check(c, &jsdoc, "`@model`")?;
        let source = format!("`overrides.{}.bindings`", c.tag);
        let config = cfg
            .overrides
            .get(&c.tag)
            .into_iter()
            .flat_map(|o| &o.bindings)
            .map(|b| {
                Ok(TwoWayBinding {
                    prop: b.prop.clone(),
                    event: b.event.clone(),
                    value: value_path(b.value.as_deref()).ok_or_else(|| {
                        bad_path(b.value.as_deref(), &c.tag, &c.module.path, &source)
                    })?,
                    inferred: false,
                })
            })
            .collect::<Result<Vec<_>, IrError>>()?;
        check(c, &config, &source)?;
        let mut bindings = match cfg.bindings {
            BindingsMode::Infer => infer(c),
            BindingsMode::ExplicitOnly => Vec::new(),
        };
        for b in jsdoc.into_iter().chain(config) {
            bindings.retain(|x| x.prop != b.prop);
            bindings.push(b);
        }
        c.bindings = bindings;
    }
    Ok(())
}

fn infer(c: &Component) -> Vec<TwoWayBinding> {
    let mut out = Vec::new();
    // Read-only props are not wrapper props, so they cannot be bound.
    for p in c
        .props
        .iter()
        .filter(|p| PROPS.contains(&p.name.as_str()) && !p.readonly)
    {
        // Prop-specific events first, then the generic ones.
        let candidates = [
            format!("{}-change", p.name),
            format!("{}Change", p.name),
            "change".to_owned(),
            "input".to_owned(),
        ];
        let Some(event) = candidates
            .into_iter()
            .find(|n| c.events.iter().any(|e| e.dom_name == *n))
        else {
            continue;
        };
        let value = match (p.name.as_str(), event.as_str()) {
            ("value", "input" | "change") if c.form_associated => {
                ValuePath::TargetProp("value".to_owned())
            }
            ("checked", "change") => ValuePath::TargetProp("checked".to_owned()),
            _ => ValuePath::Detail,
        };
        out.push(TwoWayBinding {
            prop: p.name.clone(),
            event,
            value,
            inferred: true,
        });
    }
    out
}

/// `detail` (also when absent), `detail.<field>` or `target.<prop>`. Fields must be identifiers:
/// emitters print them as `e.detail.<field>`.
pub(crate) fn value_path(text: Option<&str>) -> Option<ValuePath> {
    let text = text.unwrap_or("detail");
    if text == "detail" {
        return Some(ValuePath::Detail);
    }
    let (head, field) = text.split_once('.')?;
    let ident = field.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_' || c == '$')
        && field
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '$');
    match head {
        "detail" if ident => Some(ValuePath::DetailField(field.to_owned())),
        "target" if ident => Some(ValuePath::TargetProp(field.to_owned())),
        _ => None,
    }
}

pub(crate) fn bad_path(text: Option<&str>, tag: &str, module: &str, source: &str) -> IrError {
    IrError::new(
        "E015",
        format!(
            "{source} binding value `{}` is not a value path",
            text.unwrap_or_default()
        ),
        Some(tag),
        Some(module),
        "use `detail`, `detail.<field>` or `target.<prop>`",
    )
}

fn check(c: &Component, bindings: &[TwoWayBinding], source: &str) -> Result<(), IrError> {
    let err = |message: String, hint: String| {
        IrError::new("E015", message, Some(&c.tag), Some(&c.module.path), hint)
    };
    for (i, b) in bindings.iter().enumerate() {
        if bindings[..i].iter().any(|x| x.prop == b.prop) {
            return Err(err(
                format!("{source} binds prop `{}` twice", b.prop),
                "keep one binding per prop".to_owned(),
            ));
        }
        match c.props.iter().find(|p| p.name == b.prop) {
            None => {
                let props: Vec<_> = c.props.iter().map(|p| p.name.as_str()).collect();
                return Err(err(
                    format!("{source} binds `{}`, which is not a prop", b.prop),
                    format!("props are: {}", props.join(", ")),
                ));
            }
            Some(p) if p.readonly => {
                return Err(err(
                    format!("{source} binds `{}`, which is read-only", b.prop),
                    "bind a prop with a setter".to_owned(),
                ));
            }
            Some(_) => {}
        }
        if let ValuePath::TargetProp(t) = &b.value
            && !c.props.iter().any(|p| p.name == *t)
        {
            return Err(err(
                format!("{source} reads `target.{t}`, which is not a prop"),
                "read a prop of the element, or use `detail` / `detail.<field>`".to_owned(),
            ));
        }
        if !c.events.iter().any(|e| e.dom_name == b.event) {
            let events: Vec<_> = c.events.iter().map(|e| e.dom_name.as_str()).collect();
            return Err(err(
                format!(
                    "{source} names event `{}`, which the component does not fire",
                    b.event
                ),
                format!("events are: {}", events.join(", ")),
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::test_util::{build_json, build_json_with, cem_one, element};
    use crate::{BindingsMode, Config, TwoWayBinding, ValuePath};

    fn bindings(
        props: &[&str],
        events: &[&str],
        form_associated: bool,
        cfg: &Config,
    ) -> Vec<TwoWayBinding> {
        let mut el = element("x-a", "XA");
        el["members"] = props
            .iter()
            .map(|p| json!({ "kind": "field", "name": p, "type": { "text": "string" } }))
            .collect();
        el["events"] = events
            .iter()
            .map(|e| json!({ "name": e, "type": { "text": "CustomEvent" } }))
            .collect();
        let mut lib = build_json_with(
            cem_one(el),
            &Config {
                bindings: BindingsMode::ExplicitOnly,
                ..Config::default()
            },
        )
        .unwrap();
        lib.components[0].form_associated = form_associated;
        super::run(&mut lib, cfg).unwrap();
        lib.components.remove(0).bindings
    }

    fn one(prop: &str, event: &str, value: ValuePath) -> Vec<TwoWayBinding> {
        vec![TwoWayBinding {
            prop: prop.into(),
            event: event.into(),
            value,
            inferred: true,
        }]
    }

    #[test]
    fn heuristic() {
        let cfg = Config::default();
        assert_eq!(
            bindings(&["value"], &["input"], true, &cfg),
            one("value", "input", ValuePath::TargetProp("value".into()))
        );
        assert_eq!(
            bindings(&["value"], &["change"], false, &cfg),
            one("value", "change", ValuePath::Detail)
        );
        assert_eq!(
            bindings(&["checked"], &["change"], false, &cfg),
            one("checked", "change", ValuePath::TargetProp("checked".into()))
        );
        assert_eq!(
            bindings(&["open"], &["change", "open-change"], false, &cfg),
            one("open", "open-change", ValuePath::Detail)
        );
        assert_eq!(
            bindings(&["selected"], &["selectedChange"], false, &cfg),
            one("selected", "selectedChange", ValuePath::Detail)
        );
        assert_eq!(bindings(&["label"], &["change"], false, &cfg), []);
        // A getter-only `value` is not a wrapper prop, so it is never bound.
        let mut el = element("x-r", "XR");
        el["members"] = json!([{ "kind": "field", "name": "value", "type": { "text": "string" }, "readonly": true }]);
        el["events"] = json!([{ "name": "change", "type": { "text": "Event" } }]);
        assert_eq!(build_json(cem_one(el)).unwrap().components[0].bindings, []);
        assert_eq!(bindings(&["value"], &["fx-press"], false, &cfg), []);
    }

    #[test]
    fn explicit_only_disables_inference() {
        let cfg = Config {
            bindings: BindingsMode::ExplicitOnly,
            ..Config::default()
        };
        assert_eq!(bindings(&["value"], &["change"], false, &cfg), []);
    }

    /// fx-dialog shape: `open` + `open-change`, which the heuristic reads as `Detail`.
    fn dialog(models: serde_json::Value) -> serde_json::Value {
        let mut el = element("x-d", "XD");
        el["members"] = json!([
            { "kind": "field", "name": "open", "type": { "text": "boolean" } },
            { "kind": "field", "name": "size", "type": { "text": "string" }, "readonly": true }
        ]);
        el["events"] = json!([
            { "name": "open-change", "type": { "text": "CustomEvent<{ open: boolean }>" } },
            { "name": "toggle", "type": { "text": "Event" } }
        ]);
        el["x-holyfloe-models"] = models;
        el
    }

    fn with_config(overrides: serde_json::Value) -> Config {
        serde_json::from_value(json!({ "overrides": { "x-d": { "bindings": overrides } } }))
            .unwrap()
    }

    fn explicit(prop: &str, event: &str, value: ValuePath) -> TwoWayBinding {
        TwoWayBinding {
            prop: prop.into(),
            event: event.into(),
            value,
            inferred: false,
        }
    }

    #[test]
    fn model_beats_inferred_and_config_beats_model() {
        let el =
            dialog(json!([{ "prop": "open", "event": "open-change", "value": "detail.open" }]));
        assert_eq!(
            build_json(cem_one(el.clone())).unwrap().components[0].bindings,
            [explicit(
                "open",
                "open-change",
                ValuePath::DetailField("open".into())
            )]
        );
        let cfg =
            with_config(json!([{ "prop": "open", "event": "toggle", "value": "target.open" }]));
        assert_eq!(
            build_json_with(cem_one(el), &cfg).unwrap().components[0].bindings,
            [explicit(
                "open",
                "toggle",
                ValuePath::TargetProp("open".into())
            )]
        );
    }

    #[test]
    fn explicit_bindings_survive_explicit_only() {
        let el = dialog(json!([{ "prop": "open", "event": "open-change" }]));
        let cfg = Config {
            bindings: BindingsMode::ExplicitOnly,
            ..Config::default()
        };
        assert_eq!(
            build_json_with(cem_one(el), &cfg).unwrap().components[0].bindings,
            [explicit("open", "open-change", ValuePath::Detail)]
        );
    }

    #[test]
    fn bad_explicit_bindings_are_e015() {
        let cases = [
            json!([{ "prop": "nope", "event": "open-change" }]),
            json!([{ "prop": "size", "event": "open-change" }]),
            json!([{ "prop": "open", "event": "nope" }]),
            json!([{ "prop": "open", "event": "open-change", "value": "detail." }]),
            json!([{ "prop": "open", "event": "open-change", "value": "target.nope" }]),
            json!([{ "prop": "open", "event": "open-change" }, { "prop": "open", "event": "toggle" }]),
        ];
        for models in cases {
            let err = build_json(cem_one(dialog(models.clone()))).unwrap_err();
            assert_eq!(err.code, "E015", "@model {models}");
            let cfg = with_config(models.clone());
            let err = build_json_with(cem_one(dialog(json!([]))), &cfg).unwrap_err();
            assert_eq!(err.code, "E015", "config {models}");
        }
    }

    #[test]
    fn misspelled_config_key_is_rejected() {
        let r: Result<Config, _> = serde_json::from_value(
            json!({ "overrides": { "x-d": { "bindings": [{ "prop": "open", "event": "e", "vale": "detail" }] } } }),
        );
        assert!(r.is_err());
    }

    #[test]
    fn value_paths() {
        use super::value_path;
        assert_eq!(value_path(None), Some(ValuePath::Detail));
        assert_eq!(value_path(Some("detail")), Some(ValuePath::Detail));
        assert_eq!(
            value_path(Some("detail.value")),
            Some(ValuePath::DetailField("value".into()))
        );
        assert_eq!(
            value_path(Some("target.$v_1")),
            Some(ValuePath::TargetProp("$v_1".into()))
        );
        for bad in [
            "",
            "details",
            "detail.",
            "detail.a.b",
            "target.1x",
            "event.x",
            "detail.a-b",
        ] {
            assert_eq!(value_path(Some(bad)), None, "{bad}");
        }
    }
}
