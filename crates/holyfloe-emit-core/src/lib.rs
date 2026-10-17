//! Shared minijinja environment and filters.

use minijinja::syntax::SyntaxConfig;
use minijinja::{AutoEscape, Environment, Error, UndefinedBehavior};

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
    env.add_filter("ts_string", |s: &str| ts_string(s));
    env.add_filter("jsdoc", |s: &str| jsdoc(s));
    Ok(env)
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

/// A `/** ... */` block, or `""` for blank text. Blank lines become ` *`, and `*/` is escaped so
/// it cannot end the comment. Indent it with minijinja's `indent`: `{{ d | jsdoc | indent(6) }}`.
pub fn jsdoc(text: &str) -> String {
    let text = text.trim();
    if text.is_empty() {
        return String::new();
    }
    let mut out = String::from("/**\n");
    for line in text.lines() {
        let line = line.trim_end().replace("*/", "*\\/");
        out.push_str(" *");
        if !line.is_empty() {
            out.push(' ');
            out.push_str(&line);
        }
        out.push('\n');
    }
    out.push_str(" */");
    out
}

#[cfg(test)]
mod tests {
    use minijinja::context;

    use super::*;

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
            jsdoc("Basic action button.\n\nA clickable button.  \n"),
            "/**\n * Basic action button.\n *\n * A clickable button.\n */"
        );
        assert_eq!(jsdoc("Ends */ here"), "/**\n * Ends *\\/ here\n */");
        assert_eq!(jsdoc(""), "");
        assert_eq!(jsdoc(" \n  "), "");
    }

    #[test]
    fn env_registers_filters() {
        let mut env = env().unwrap();
        env.add_template(
            "t.tsx.j2",
            "{{ \"<a & 'b'>\" | ts_string }}\n{% if true %}\n  {{ doc | jsdoc | indent(2) }}\n{% endif %}\n",
        )
        .unwrap();
        let out = env
            .get_template("t.tsx.j2")
            .unwrap()
            .render(context!(doc => "Card.\n\nSlots:"))
            .unwrap();
        assert_eq!(
            out,
            "'<a & \\'b\\'>'\n  /**\n   * Card.\n   *\n   * Slots:\n   */\n"
        );
    }

    #[test]
    fn env_rejects_undefined() {
        let mut env = env().unwrap();
        env.add_template("t", "{{ missing }}").unwrap();
        assert!(env.get_template("t").unwrap().render(context!()).is_err());
    }
}
