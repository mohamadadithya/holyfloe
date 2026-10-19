//! Shared minijinja environment, filters, macros and emitter errors.

use std::collections::{BTreeMap, BTreeSet};

use holyfloe_ir::Component;
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
    env.add_template("macros.j2", include_str!("../macros.j2"))?;
    env.add_filter("ts_string", |s: &str| ts_string(s));
    env.add_filter("ts_key", |s: &str| ts_key(s));
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

/// An object key: bare when it is an identifier, else a string literal (`'badge-count'`).
pub fn ts_key(s: &str) -> String {
    let mut chars = s.chars();
    let ident = chars
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_' || c == '$')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '$');
    if ident { s.to_owned() } else { ts_string(s) }
}

/// Errors every emitter can return. Codes continue the `IrError` table in `docs/ir.md` (D-070).
#[derive(Debug, thiserror::Error)]
pub enum EmitError {
    #[error("template: {0}")]
    Template(#[from] minijinja::Error),
    /// Detail type text is re-emitted verbatim, so an imported name cannot be renamed.
    #[error("event detail type `{name}` of `{tag}` {clash}")]
    DetailImportClash {
        tag: String,
        name: String,
        clash: String,
    },
    /// A prop would share its name with something the wrapper generates: an event's
    /// `on<base_name>` prop, Svelte `element` or a snippet, Vue `modelValue` or a method.
    #[error("prop `{prop}` of `{tag}` has the same name as {generated}")]
    NameClash {
        tag: String,
        prop: String,
        generated: String,
    },
}

impl EmitError {
    /// The `E0xx` code the CLI prints. `None` for template errors, which are holyfloe bugs
    /// rather than input a user can fix.
    pub fn code(&self) -> Option<&'static str> {
        match self {
            Self::Template(_) => None,
            Self::DetailImportClash { .. } => Some("E012"),
            Self::NameClash { .. } => Some("E013"),
        }
    }

    pub fn hint(&self) -> Option<String> {
        match self {
            Self::Template(_) => None,
            Self::DetailImportClash { .. } => {
                Some("rename the type in the library, or exclude the event's component".to_owned())
            }
            Self::NameClash { tag, .. } => Some(format!(
                "drop the prop with `overrides.{tag}.excludeProps`, or, when the other name is an event prop, rename the event with `overrides.{tag}.events`"
            )),
        }
    }
}

/// `prop` against the names the wrapper generates (`generated`: name and what it is). Read-only
/// props are not wrapper props, so they cannot clash.
pub fn check_names(c: &Component, generated: &[(String, String)]) -> Result<(), EmitError> {
    for p in c.props.iter().filter(|p| !p.readonly) {
        if let Some((_, what)) = generated.iter().find(|(name, _)| *name == p.name) {
            return Err(EmitError::NameClash {
                tag: c.tag.clone(),
                prop: p.name.clone(),
                generated: what.clone(),
            });
        }
    }
    Ok(())
}

/// Types named by event details, grouped by specifier: the one place CEM type text is re-emitted.
/// A name imported from two modules, or one the file already uses (`own`), would not compile.
pub fn detail_imports<'a>(
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

/// An identifier for a name that may not be one, such as a slot name: characters outside
/// `[A-Za-z0-9_$]` separate camelCase words (`help-text` -> `helpText`), and a leading digit gets
/// `_`. Then `js_local` rules apply (`for` -> `for_`).
pub fn camel_ident(name: &str, taken: &[String]) -> String {
    let mut out = String::with_capacity(name.len());
    let mut upper = false;
    for c in name.chars() {
        if c.is_ascii_alphanumeric() || c == '_' || c == '$' {
            out.push(if upper && !out.is_empty() {
                c.to_ascii_uppercase()
            } else {
                c
            });
            upper = false;
        } else {
            upper = true;
        }
    }
    if out.is_empty() || out.starts_with(|c: char| c.is_ascii_digit()) {
        out.insert(0, '_');
    }
    js_local(&out, taken)
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
    fn ts_keys() {
        assert_eq!(ts_key("tone"), "tone");
        assert_eq!(ts_key("onmyChange"), "onmyChange");
        assert_eq!(ts_key("$x_1"), "$x_1");
        assert_eq!(ts_key("badge-count"), "'badge-count'");
        assert_eq!(ts_key("--fx-card-padding"), "'--fx-card-padding'");
        assert_eq!(ts_key("1a"), "'1a'");
        assert_eq!(ts_key(""), "''");
    }

    #[test]
    fn camel_idents() {
        assert_eq!(camel_ident("header", &[]), "header");
        assert_eq!(camel_ident("help-text", &[]), "helpText");
        assert_eq!(camel_ident("play-icon", &["body".into()]), "playIcon");
        assert_eq!(camel_ident("for", &[]), "for_");
        assert_eq!(camel_ident("body", &["body".into()]), "body_");
        assert_eq!(camel_ident("2x", &[]), "_2x");
        assert_eq!(camel_ident("-", &[]), "_");
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
