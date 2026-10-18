//! Pass 4: `PropKind` from the prop type text, and event detail from `CustomEvent<T>`.
//! Type text is only lexed, never rewritten or re-emitted (it can be wrong, D-055).

use crate::{Library, PropKind, TypeRef};

pub(crate) fn run(lib: &mut Library) {
    for c in &mut lib.components {
        for p in &mut c.props {
            p.kind =
                p.ty.as_ref()
                    .map_or(PropKind::Complex, |t| classify(&t.text));
        }
        for e in &mut c.events {
            e.detail = e.event_type.as_ref().and_then(|t| {
                detail(&t.text).map(|text| TypeRef {
                    text: text.to_owned(),
                    imports: t.imports.clone(),
                })
            });
        }
    }
}

pub(crate) fn classify(text: &str) -> PropKind {
    let Some(pipes) = top_level_pipes(text) else {
        return PropKind::Complex;
    };
    let mut start = 0;
    let mut members = Vec::new();
    for end in pipes.into_iter().chain([text.len()]) {
        members.push(text[start..end].trim());
        start = end + 1;
    }
    // Optional props come out as `T | undefined`; a leading `|` leaves an empty member.
    members.retain(|m| !m.is_empty() && *m != "undefined" && *m != "null");
    match members.as_slice() {
        [] => PropKind::Complex,
        ["boolean"] => PropKind::Boolean,
        ms if ms.iter().all(|m| is_primitive(m)) => PropKind::Primitive,
        _ => PropKind::Complex,
    }
}

fn is_primitive(m: &str) -> bool {
    let quoted = |q: char| m.len() >= 2 && m.starts_with(q) && m.ends_with(q);
    matches!(m, "string" | "number" | "bigint")
        || quoted('\'')
        || quoted('"')
        || (quoted('`') && !m.contains("${"))
        || (m.starts_with(|c: char| c.is_ascii_digit() || c == '-' || c == '.')
            && m.parse::<f64>().is_ok())
}

/// `T` from `CustomEvent<T>`, verbatim. Anything else (bare `CustomEvent`, unions, named
/// subclasses) has no detail; emitters render that as `unknown`.
pub(crate) fn detail(text: &str) -> Option<&str> {
    let inner = text
        .trim()
        .strip_prefix("CustomEvent<")?
        .strip_suffix('>')?;
    // Balanced means the `<` after `CustomEvent` closes at the very end.
    top_level_pipes(inner)?;
    Some(inner.trim())
}

/// Byte offsets of `|` outside brackets and string literals. `None` when brackets don't balance.
fn top_level_pipes(text: &str) -> Option<Vec<usize>> {
    let bytes = text.as_bytes();
    let mut depth = 0usize;
    let mut quote = None;
    let mut pipes = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if let Some(q) = quote {
            if b == b'\\' {
                i += 1;
            } else if b == q {
                quote = None;
            }
        } else {
            match b {
                b'\'' | b'"' | b'`' => quote = Some(b),
                b'(' | b'[' | b'{' | b'<' => depth += 1,
                b'>' if i > 0 && bytes[i - 1] == b'=' => {} // `=>`
                b')' | b']' | b'}' | b'>' => depth = depth.checked_sub(1)?,
                b'|' if depth == 0 => pipes.push(i),
                _ => {}
            }
        }
        i += 1;
    }
    (depth == 0 && quote.is_none()).then_some(pipes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use PropKind::*;

    #[test]
    fn kinds() {
        for (text, kind) in [
            ("boolean", Boolean),
            ("boolean | undefined", Boolean),
            ("string", Primitive),
            ("string | undefined", Primitive),
            ("number | null", Primitive),
            ("bigint", Primitive),
            ("'primary' | 'secondary' | 'danger'", Primitive),
            ("| \"a\" | \"b\"", Primitive),
            ("1 | 2 | -3.5", Primitive),
            ("'a|b' | 'c'", Primitive),
            ("string | boolean", Complex),
            ("ListItem[]", Complex),
            ("ListOptions", Complex),
            ("{ a: string | number }", Complex),
            ("Array<'a' | 'b'>", Complex),
            ("unknown", Complex),
            ("any", Complex),
            ("undefined", Complex),
            ("`x-${string}`", Complex),
            ("NaN", Complex),
            ("(a: string", Complex),
        ] {
            assert_eq!(classify(text), kind, "{text}");
        }
    }

    #[test]
    fn lossy_function_type_is_complex() {
        // D-055: the analyzer drops the parentheses around `(item) => string | undefined`.
        assert_eq!(classify("(item: ListItem) => string | undefined"), Complex);
    }

    #[test]
    fn details() {
        assert_eq!(
            detail("CustomEvent<{ value: string }>"),
            Some("{ value: string }")
        );
        assert_eq!(detail("CustomEvent<() => void>"), Some("() => void"));
        assert_eq!(
            detail("CustomEvent<Map<string, A | B>>"),
            Some("Map<string, A | B>")
        );
        assert_eq!(detail("CustomEvent"), None);
        assert_eq!(detail("MyEvent"), None);
        assert_eq!(detail("CustomEvent<A> | CustomEvent<B>"), None);
    }
}
