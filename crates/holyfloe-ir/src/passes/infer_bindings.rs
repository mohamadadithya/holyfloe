//! Pass 5: two-way bindings from the heuristic in `docs/ir.md`. Explicit `@model` and config
//! bindings wait for D-026 (M2); `bindings: "explicit-only"` turns inference off.

use crate::{BindingsMode, Config, Library, TwoWayBinding, ValuePath};

const PROPS: [&str; 4] = ["value", "checked", "open", "selected"];

pub(crate) fn run(lib: &mut Library, cfg: &Config) {
    if cfg.bindings == BindingsMode::ExplicitOnly {
        return;
    }
    for c in &mut lib.components {
        for p in c.props.iter().filter(|p| PROPS.contains(&p.name.as_str())) {
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
            c.bindings.push(TwoWayBinding {
                prop: p.name.clone(),
                event,
                value,
                inferred: true,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::test_util::{build_json_with, cem_one, element};
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
        super::run(&mut lib, cfg);
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
}
