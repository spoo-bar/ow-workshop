//! Normalizing a string before it is looked up in the catalog.

// The string lowering that calls these lands in the next change.
#![allow(dead_code)]

use std::fmt::Write as _;

/// Most placeholders a translated string may have.
const MAX_PLACEHOLDERS: usize = 16;

/// Whether `c` counts as whitespace around a translated string.
fn is_blank(c: char) -> bool {
    c.is_whitespace() || c.is_control() || matches!(c, '\u{AD}' | '\u{3164}')
}

/// A string split into the part that is looked up and the whitespace around it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Stripped {
    /// The lookup key: the lines without their outer whitespace.
    pub(crate) key: String,
    lead: String,
    trail: String,
    /// Whitespace before and after each line of the key.
    lines: Vec<(String, String)>,
}

impl Stripped {
    /// `translation` with the original whitespace put back around it and its lines.
    pub(crate) fn apply(&self, translation: &str) -> String {
        let parts: Vec<&str> = translation.split('\n').collect();
        let mut out = self.lead.clone();
        if parts.len() == self.lines.len() {
            for (i, (part, (before, after))) in parts.iter().zip(&self.lines).enumerate() {
                if i > 0 {
                    out.push('\n');
                }
                out.push_str(before);
                out.push_str(part);
                out.push_str(after);
            }
        } else {
            out.push_str(translation);
        }
        out.push_str(&self.trail);
        out
    }
}

/// Strips whitespace around the string and around each of its lines.
pub(crate) fn strip_whitespace(text: &str) -> Stripped {
    let core = text.trim_start_matches(is_blank).trim_end_matches(is_blank);
    let lead = text[..text.len() - text.trim_start_matches(is_blank).len()].to_owned();
    let trail = text[text.trim_end_matches(is_blank).len()..].to_owned();
    let mut lines = Vec::new();
    let mut bodies = Vec::new();
    for line in core.split('\n') {
        let body = line.trim_start_matches(is_blank).trim_end_matches(is_blank);
        let before = line[..line.len() - line.trim_start_matches(is_blank).len()].to_owned();
        let after = line[line.trim_end_matches(is_blank).len()..].to_owned();
        bodies.push(body);
        lines.push((before, after));
    }
    Stripped {
        key: bodies.join("\n"),
        lead,
        trail,
        lines,
    }
}

/// Numbers the `{}` placeholders `{0}`, `{1}`, … in order; `{n}` ones are kept.
///
/// Fails when the string has more than 16 placeholders.
pub(crate) fn number_placeholders(text: &str) -> Result<String, usize> {
    let mut out = String::with_capacity(text.len());
    let (mut next, mut count) = (0, 0);
    let mut rest = text;
    while let Some(open) = rest.find('{') {
        out.push_str(&rest[..open]);
        let after = &rest[open..];
        let Some(close) = after.find('}') else {
            rest = after;
            break;
        };
        let inner = &after[1..close];
        if inner.is_empty() {
            let _ = write!(out, "{{{next}}}");
            next += 1;
            count += 1;
        } else if inner.bytes().all(|b| b.is_ascii_digit()) {
            out.push_str(&after[..=close]);
            count += 1;
        } else {
            out.push('{');
            rest = &after[1..];
            continue;
        }
        rest = &after[close + 1..];
    }
    out.push_str(rest);
    if count > MAX_PLACEHOLDERS {
        return Err(count);
    }
    Ok(out)
}
