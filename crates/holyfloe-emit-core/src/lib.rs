//! Shared minijinja environment and filters.

use minijinja::syntax::SyntaxConfig;
use minijinja::{AutoEscape, Environment, Error, UndefinedBehavior};

pub use holyfloe_ir::pascal;

/// One generated file. `path` is relative to the output root, `/`-separated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputFile {
    pub path: String,
    pub contents: String,
}

/// The environment every emitter adds its templates to.
///
/// Block tags eat their own line, the trailing newline is kept, nothing is escaped,
/// and undefined values are errors so template typos fail instead of printing nothing.
pub fn env() -> Result<Environment<'static>, Error> {
    let mut env = Environment::new();
    env.set_syntax(
        SyntaxConfig::builder()
            .trim_blocks(true)
            .lstrip_blocks(true)
            .keep_trailing_newline(true)
            .build()?,
    );
    env.set_auto_escape_callback(|_| AutoEscape::None);
    env.set_undefined_behavior(UndefinedBehavior::Strict);
    env.add_filter("pascal", |s: &str| pascal(s));
    env.add_filter("camel", |s: &str| camel(s));
    env.add_filter("kebab", |s: &str| kebab(s));
    env.add_filter("ts_string", |s: &str| ts_string(s));
    env.add_filter("jsdoc", |s: &str, indent: Option<usize>| {
        jsdoc(s, indent.unwrap_or(0))
    });
    Ok(env)
}

/// [`pascal`] with the first letter lowercased: `my-change` -> `myChange`.
pub fn camel(name: &str) -> String {
    let p = pascal(name);
    let mut chars = p.chars();
    chars
        .next()
        .into_iter()
        .flat_map(char::to_lowercase)
        .chain(chars)
        .collect()
}

/// `FxButton` -> `fx-button`, `HTMLElement` -> `html-element`, `my_change` -> `my-change`.
pub fn kebab(name: &str) -> String {
    let chars: Vec<char> = name.chars().collect();
    let mut out = String::with_capacity(name.len() + 4);
    for (i, &c) in chars.iter().enumerate() {
        if c == '-' || c == '_' {
            out.push('-');
            continue;
        }
        if c.is_uppercase() && i > 0 && !out.ends_with('-') {
            let prev = chars[i - 1];
            let next_lower = chars.get(i + 1).is_some_and(|n| n.is_lowercase());
            if prev.is_lowercase() || prev.is_ascii_digit() || (prev.is_uppercase() && next_lower) {
                out.push('-');
            }
        }
        out.extend(c.to_lowercase());
    }
    out
}

/// A single-quoted TypeScript string literal.
pub fn ts_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('\'');
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '\'' => out.push_str("\\'"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            _ => out.push(c),
        }
    }
    out.push('\'');
    out
}

/// A `/** ... */` block. The opening line is left where the template puts it; later lines get
/// `indent` spaces. Blank lines become ` *`, and `*/` in the text is escaped so it cannot end the comment.
pub fn jsdoc(text: &str, indent: usize) -> String {
    let pad = " ".repeat(indent);
    let mut out = String::from("/**\n");
    for line in text.trim().lines() {
        let line = line.trim_end().replace("*/", "*\\/");
        out.push_str(&pad);
        out.push_str(" *");
        if !line.is_empty() {
            out.push(' ');
            out.push_str(&line);
        }
        out.push('\n');
    }
    out.push_str(&pad);
    out.push_str(" */");
    out
}

#[cfg(test)]
mod tests {
    use minijinja::context;

    use super::*;

    #[test]
    fn pascal_filter() {
        assert_eq!(pascal("fx-button"), "FxButton");
        assert_eq!(pascal("my_change"), "MyChange");
    }

    #[test]
    fn camel_filter() {
        assert_eq!(camel("fx-button"), "fxButton");
        assert_eq!(camel("MyChange"), "myChange");
        assert_eq!(camel("myChange"), "myChange");
        assert_eq!(camel(""), "");
    }

    #[test]
    fn kebab_filter() {
        assert_eq!(kebab("FxButton"), "fx-button");
        assert_eq!(kebab("myChange"), "my-change");
        assert_eq!(kebab("HTMLElement"), "html-element");
        assert_eq!(kebab("fx-button"), "fx-button");
        assert_eq!(kebab("my_change"), "my-change");
        assert_eq!(kebab("Fx-Button"), "fx-button");
        assert_eq!(kebab("item2Label"), "item2-label");
        assert_eq!(kebab("MYEVENT"), "myevent");
    }

    #[test]
    fn ts_string_filter() {
        assert_eq!(ts_string("fx-button"), "'fx-button'");
        assert_eq!(ts_string(r"it's a\b"), r"'it\'s a\\b'");
        assert_eq!(ts_string("a\r\nb"), r"'a\r\nb'");
        assert_eq!(ts_string("\"x\""), "'\"x\"'");
    }

    #[test]
    fn jsdoc_filter() {
        assert_eq!(
            jsdoc("Basic action button.\n\nA clickable button.  \n", 0),
            "/**\n * Basic action button.\n *\n * A clickable button.\n */"
        );
        assert_eq!(jsdoc("Ends */ here", 2), "/**\n   * Ends *\\/ here\n   */");
    }

    #[test]
    fn env_registers_filters() {
        let mut env = env().unwrap();
        env.add_template(
            "t.tsx.j2",
            "{{ tag | pascal }} {{ tag | camel }} {{ 'FxCard' | kebab }} {{ \"<a & 'b'>\" | ts_string }}\n\
             {% if true %}\n  {{ doc | jsdoc(2) }}\n{% endif %}\n",
        )
        .unwrap();
        let out = env
            .get_template("t.tsx.j2")
            .unwrap()
            .render(context!(tag => "fx-card", doc => "Card.\n\nSlots:"))
            .unwrap();
        assert_eq!(
            out,
            "FxCard fxCard fx-card '<a & \\'b\\'>'\n  /**\n   * Card.\n   *\n   * Slots:\n   */\n"
        );
    }

    #[test]
    fn env_rejects_undefined() {
        let mut env = env().unwrap();
        env.add_template("t", "{{ missing }}").unwrap();
        assert!(env.get_template("t").unwrap().render(context!()).is_err());
    }
}
