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
    env.add_filter("js_local", |s: &str, taken: Option<Vec<String>>| {
        js_local(s, taken.as_deref().unwrap_or_default())
    });
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

/// Words that cannot name a local in module code: reserved words, strict-mode reserved words,
/// and `arguments`/`eval`, which strict mode forbids as binding names.
const RESERVED: [&str; 48] = [
    "arguments",
    "await",
    "break",
    "case",
    "catch",
    "class",
    "const",
    "continue",
    "debugger",
    "default",
    "delete",
    "do",
    "else",
    "enum",
    "eval",
    "export",
    "extends",
    "false",
    "finally",
    "for",
    "function",
    "if",
    "implements",
    "import",
    "in",
    "instanceof",
    "interface",
    "let",
    "new",
    "null",
    "package",
    "private",
    "protected",
    "public",
    "return",
    "static",
    "super",
    "switch",
    "this",
    "throw",
    "true",
    "try",
    "typeof",
    "var",
    "void",
    "while",
    "with",
    "yield",
];

/// A local variable name for a member: `name`, or `name_` when `name` is a reserved word (Web
/// Awesome's `for`) or one of `taken`, the locals the template binds itself. Templates write
/// `{ for: for_ }` when the two differ.
// shortcut: `name_` is not checked against other members' names; check it if a library ever has both.
pub fn js_local(name: &str, taken: &[String]) -> String {
    if RESERVED.contains(&name) || taken.iter().any(|t| t == name) {
        format!("{name}_")
    } else {
        name.to_owned()
    }
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
    fn js_local_filter() {
        assert_eq!(js_local("tone", &[]), "tone");
        assert_eq!(js_local("for", &[]), "for_");
        assert_eq!(js_local("case", &[]), "case_");
        assert_eq!(js_local("eval", &[]), "eval_");
        assert_eq!(js_local("rest", &["ref".into(), "rest".into()]), "rest_");
        assert_eq!(
            js_local("type", &[]),
            "type",
            "contextual keywords are fine"
        );
    }

    #[test]
    fn env_registers_filters() {
        let mut env = env().unwrap();
        env.add_template(
            "t.tsx.j2",
            "{{ \"<a & 'b'>\" | ts_string }}{{ 'for' | js_local }}{{ 'ref' | js_local(['ref']) }}\n{% if true %}\n  {{ doc | jsdoc | indent(2) }}\n{% endif %}\n",
        )
        .unwrap();
        let out = env
            .get_template("t.tsx.j2")
            .unwrap()
            .render(context!(doc => "Card.\n\nSlots:"))
            .unwrap();
        assert_eq!(
            out,
            "'<a & \\'b\\'>'for_ref_\n  /**\n   * Card.\n   *\n   * Slots:\n   */\n"
        );
    }

    #[test]
    fn env_rejects_undefined() {
        let mut env = env().unwrap();
        env.add_template("t", "{{ missing }}").unwrap();
        assert!(env.get_template("t").unwrap().render(context!()).is_err());
    }
}
