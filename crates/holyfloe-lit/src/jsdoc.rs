//! The JSDoc subset Lit libraries document components with, read the way the CEM analyzer reads
//! it, so both inputs give the same IR.

/// A `/** */` comment: the text before the first tag, then each tag with its text.
#[derive(Debug, Default, PartialEq)]
pub struct Doc {
    pub description: Option<String>,
    pub tags: Vec<(String, String)>,
}

impl Doc {
    /// `comment` is the text between `/**` and `*/`.
    pub fn parse(comment: &str) -> Self {
        let mut doc = Doc::default();
        let mut description: Vec<&str> = Vec::new();
        for line in comment.lines() {
            let line = line.trim_start();
            let line = line.strip_prefix('*').unwrap_or(line);
            let line = line.strip_prefix(' ').unwrap_or(line).trim_end();
            if let Some(tag) = line.strip_prefix('@') {
                let (name, text) = tag.split_once(char::is_whitespace).unwrap_or((tag, ""));
                doc.tags.push((name.to_owned(), text.trim().to_owned()));
            } else if let Some((_, text)) = doc.tags.last_mut() {
                // Tag text continues on the next lines. `@summary` keeps the line breaks;
                // `tag_body` collapses them, as the analyzer does for events, slots and parts.
                let line = line.trim();
                if !line.is_empty() {
                    if !text.is_empty() {
                        text.push('\n');
                    }
                    text.push_str(line);
                }
            } else {
                description.push(line);
            }
        }
        let description = description.join("\n").trim().to_owned();
        doc.description = (!description.is_empty()).then_some(description);
        doc
    }

    pub fn tag(&self, name: &str) -> Option<&str> {
        self.tags
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, t)| t.as_str())
    }

    pub fn has(&self, name: &str) -> bool {
        self.tag(name).is_some()
    }

    pub fn all<'s>(&'s self, names: &'s [&str]) -> impl Iterator<Item = &'s str> {
        self.tags
            .iter()
            .filter(move |(n, _)| names.contains(&n.as_str()))
            .map(|(_, t)| t.as_str())
    }

    /// `@internal`, `@private` or `@protected`: not public API (the analyzer reads the last two
    /// as privacy, for JS sources without TypeScript modifiers).
    pub fn hidden(&self) -> bool {
        ["internal", "private", "protected"]
            .iter()
            .any(|t| self.has(t))
    }

    /// `@deprecated` gives `Some("")` without a reason, like the IR.
    pub fn deprecated(&self) -> Option<&str> {
        self.tag("deprecated")
    }
}

/// A tag body in the form `[{type}] [name] [- description]`, as `@fires`, `@slot`, `@csspart`
/// and `@cssprop` write it. A `[name=default]` name gives a default (`@cssprop [--x=4px]`).
#[derive(Debug, Default, PartialEq)]
pub struct TagBody {
    pub ty: Option<String>,
    pub name: String,
    pub default: Option<String>,
    pub description: Option<String>,
}

pub fn tag_body(text: &str) -> TagBody {
    let mut body = TagBody::default();
    let mut rest = text.trim();
    if rest.starts_with('{') {
        // Balanced, so `{CustomEvent<{ open: boolean }>}` keeps its inner braces.
        let mut depth = 0;
        for (i, c) in rest.char_indices() {
            match c {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        body.ty = Some(rest[1..i].trim().to_owned());
                        rest = rest[i + 1..].trim_start();
                        break;
                    }
                }
                _ => {}
            }
        }
    }
    // A lone `-` starts the description of an unnamed slot; `--x` is a CSS property name.
    if !(rest == "-" || rest.starts_with("- ")) {
        let end = rest.find(char::is_whitespace).unwrap_or(rest.len());
        let name = &rest[..end];
        rest = rest[end..].trim_start();
        // `[name]` marks an optional name, `[name=default]` adds a default.
        match name.strip_prefix('[').and_then(|n| n.strip_suffix(']')) {
            Some(inner) => match inner.split_once('=') {
                Some((n, d)) => {
                    body.name = n.to_owned();
                    body.default = Some(d.to_owned());
                }
                None => body.name = inner.to_owned(),
            },
            None => body.name = name.to_owned(),
        }
    }
    let rest = rest.strip_prefix('-').unwrap_or(rest);
    let rest = rest.split_whitespace().collect::<Vec<_>>().join(" ");
    body.description = (!rest.is_empty()).then_some(rest);
    body
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn description_and_tags() {
        let doc = Doc::parse(
            "*\n * First line.\n * Second line.\n *\n * @summary Short.\n * @slot - Body.\n * @deprecated\n ",
        );
        assert_eq!(
            doc.description.as_deref(),
            Some("First line.\nSecond line.")
        );
        assert_eq!(doc.tag("summary"), Some("Short."));
        assert_eq!(doc.tag("slot"), Some("- Body."));
        assert_eq!(doc.deprecated(), Some(""));
    }

    #[test]
    fn tag_text_continues_on_following_lines() {
        let doc = Doc::parse("* @fires x - One\n *   two.\n * @slot a");
        assert_eq!(doc.tag("fires"), Some("x - One\ntwo."));
        assert_eq!(
            tag_body(doc.tag("fires").unwrap()).description.as_deref(),
            Some("One two.")
        );
        assert_eq!(doc.tag("slot"), Some("a"));
    }

    #[test]
    fn tag_bodies() {
        assert_eq!(
            tag_body("{CustomEvent<{ open: boolean }>} open-change - Fired."),
            TagBody {
                ty: Some("CustomEvent<{ open: boolean }>".into()),
                name: "open-change".into(),
                default: None,
                description: Some("Fired.".into()),
            }
        );
        let slot = tag_body("- The label.");
        assert_eq!(
            (slot.name.as_str(), slot.description.as_deref()),
            ("", Some("The label."))
        );
        let css = tag_body("--fx-gap - Gap.");
        assert_eq!(
            (css.name.as_str(), css.description.as_deref()),
            ("--fx-gap", Some("Gap."))
        );
        let part = tag_body("base");
        assert_eq!((part.name.as_str(), part.description), ("base", None));
        assert_eq!(tag_body("[--y] - Optional.").name, "--y");
        let prop = tag_body("[--x=4px] - Gap.");
        assert_eq!(
            (prop.name.as_str(), prop.default.as_deref()),
            ("--x", Some("4px"))
        );
    }
}
