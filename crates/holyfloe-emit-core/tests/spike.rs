//! D-032 spike: minijinja 3 renders serde data (via the `Serde` wrapper) and custom filters.

use minijinja::{Environment, context, value::Serde};

#[derive(serde::Serialize)]
struct Component {
    tag: &'static str,
    props: Vec<&'static str>,
}

#[test]
fn renders_serde_struct_with_custom_filter() {
    let mut env = Environment::new();
    env.add_filter("shout", |s: String| s.to_uppercase());
    env.add_template(
        "c",
        "{{ c.tag | shout }}:{% for p in c.props %} {{ p }}{% endfor %}",
    )
    .unwrap();
    let c = Component {
        tag: "fx-button",
        props: vec!["variant", "disabled"],
    };
    let out = env
        .get_template("c")
        .unwrap()
        .render(context!(c => Serde(&c)))
        .unwrap();
    assert_eq!(out, "FX-BUTTON: variant disabled");
}
