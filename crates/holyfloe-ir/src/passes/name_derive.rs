//! Pass 3: component names and event base names. Emitters add their own prefix (React: `on`).

use crate::{Config, IrError, Library};

pub(crate) fn run(lib: &mut Library, cfg: &Config) -> Result<(), IrError> {
    for (key, value) in [
        ("tagFormatter", &cfg.tag_formatter),
        ("componentNameFormatter", &cfg.component_name_formatter),
    ] {
        if value.is_some() {
            return Err(IrError::new(
                "E010",
                format!("config `{key}` is not supported yet"),
                None,
                None,
                format!("remove `{key}` from holyfloe.config.json"),
            ));
        }
    }
    for c in &mut lib.components {
        c.name = pascal(&c.tag);
        let overrides = cfg.overrides.get(&c.tag);
        for e in &mut c.events {
            e.base_name = overrides
                .and_then(|o| o.events.get(&e.dom_name))
                .cloned()
                .unwrap_or_else(|| pascal(&e.dom_name));
        }
    }
    Ok(())
}

/// Split on `-` and `_`, uppercase each segment's first letter, keep the rest as written:
/// `my-change` and `myChange` -> `MyChange`, `MYEVENT` -> `MYEVENT`.
pub(crate) fn pascal(name: &str) -> String {
    name.split(['-', '_'])
        .flat_map(|seg| {
            let mut chars = seg.chars();
            chars
                .next()
                .into_iter()
                .flat_map(char::to_uppercase)
                .chain(chars)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::pascal;
    use crate::test_util::{build_json_with, cem_one, element};
    use crate::{ComponentOverride, Config};

    #[test]
    fn pascal_cases() {
        assert_eq!(pascal("fx-button"), "FxButton");
        assert_eq!(pascal("my-change"), "MyChange");
        assert_eq!(pascal("myChange"), "MyChange");
        assert_eq!(pascal("MyChange"), "MyChange");
        assert_eq!(pascal("MYEVENT"), "MYEVENT");
        assert_eq!(pascal("a_b--c"), "ABC");
    }

    #[test]
    fn overrides_replace_base_names() {
        let mut el = element("x-a", "XA");
        el["events"] = json!([
            { "name": "my-change", "type": { "text": "CustomEvent" } },
            { "name": "myChange", "type": { "text": "CustomEvent" } }
        ]);
        let cfg = Config {
            overrides: [(
                "x-a".to_owned(),
                ComponentOverride {
                    events: [("myChange".to_owned(), "MyChangeCamel".to_owned())].into(),
                },
            )]
            .into(),
            ..Config::default()
        };
        let lib = build_json_with(cem_one(el), &cfg).unwrap();
        let c = &lib.components[0];
        assert_eq!(c.name, "XA");
        let names: Vec<_> = c.events.iter().map(|e| e.base_name.as_str()).collect();
        assert_eq!(names, ["MyChange", "MyChangeCamel"]);
    }

    #[test]
    fn formatters_are_e010() {
        let cfg = Config {
            tag_formatter: Some(json!("{tag}")),
            ..Config::default()
        };
        let err = build_json_with(cem_one(element("x-a", "XA")), &cfg).unwrap_err();
        assert_eq!(err.code, "E010");
    }
}
