//! Text rewrites behind `.fullwidth()`, `.big()` and `.case_sensitive()`.
//!
//! Each applies to the text tokens of a string template, never to its placeholders.

use crate::ir::ErrorKind;
use crate::ir::{Diagnostics, Expr, ExprKind, Program, StrExpr, StrModifier, StrToken};
use crate::lower::Ctx;
use crate::lower::visit::{blocks_mut, map_stmt_roots, rewrite_expr};
use crate::tags::segments;
use crate::warning::Warning;

/// Latin letters (both cases) and their Greek lookalikes in the large font.
const BIG_LETTERS: [(char, char); 28] = [
    ('\u{61}', '\u{391}'),
    ('\u{62}', '\u{392}'),
    ('\u{65}', '\u{395}'),
    ('\u{68}', '\u{397}'),
    ('\u{69}', '\u{399}'),
    ('\u{6B}', '\u{39A}'),
    ('\u{6D}', '\u{39C}'),
    ('\u{6E}', '\u{39D}'),
    ('\u{6F}', '\u{39F}'),
    ('\u{70}', '\u{3A1}'),
    ('\u{74}', '\u{3A4}'),
    ('\u{78}', '\u{3A7}'),
    ('\u{79}', '\u{3A5}'),
    ('\u{7A}', '\u{396}'),
    ('\u{41}', '\u{391}'),
    ('\u{42}', '\u{392}'),
    ('\u{45}', '\u{395}'),
    ('\u{48}', '\u{397}'),
    ('\u{49}', '\u{399}'),
    ('\u{4B}', '\u{39A}'),
    ('\u{4D}', '\u{39C}'),
    ('\u{4E}', '\u{39D}'),
    ('\u{4F}', '\u{39F}'),
    ('\u{50}', '\u{3A1}'),
    ('\u{54}', '\u{3A4}'),
    ('\u{58}', '\u{3A7}'),
    ('\u{59}', '\u{3A5}'),
    ('\u{5A}', '\u{396}'),
];

/// Characters that stand in for `.` and a space in the large font.
const BIG_PUNCTUATION: [(char, char); 2] = [('.', '\u{2024}'), (' ', '\u{2028}')];

/// Currency and other signs with a dedicated fullwidth form.
const FULLWIDTH_SIGNS: [(char, char); 8] = [
    (' ', '\u{2001}'),
    ('\u{A5}', '\u{FFE5}'),
    ('\u{20A9}', '\u{FFE6}'),
    ('\u{A2}', '\u{FFE0}'),
    ('\u{A3}', '\u{FFE1}'),
    ('\u{AF}', '\u{FFE3}'),
    ('\u{AC}', '\u{FFE2}'),
    ('\u{A6}', '\u{FFE4}'),
];

/// Characters after which `e` and `n` take a grave accent.
const ACCENT_TRIGGERS: &str = "0123456789!?/@\"&#^$*%";

/// Lowercase letters and the lookalikes the game does not uppercase.
const CASE_LETTERS: [(char, char); 26] = [
    ('\u{61}', '\u{1EA1}'),
    ('\u{62}', '\u{1E05}'),
    ('\u{63}', '\u{188}'),
    ('\u{64}', '\u{1E0D}'),
    ('\u{65}', '\u{1EB9}'),
    ('\u{66}', '\u{192}'),
    ('\u{67}', '\u{1E5}'),
    ('\u{68}', '\u{4BB}'),
    ('\u{69}', '\u{456}'),
    ('\u{6A}', '\u{458}'),
    ('\u{6B}', '\u{1E33}'),
    ('\u{6C}', '\u{49}'),
    ('\u{6D}', '\u{1E43}'),
    ('\u{6E}', '\u{1E47}'),
    ('\u{6F}', '\u{3BF}'),
    ('\u{70}', '\u{1E57}'),
    ('\u{71}', '\u{1EB}'),
    ('\u{72}', '\u{1E5B}'),
    ('\u{73}', '\u{455}'),
    ('\u{74}', '\u{1E6D}'),
    ('\u{75}', '\u{3C5}'),
    ('\u{76}', '\u{3BD}'),
    ('\u{77}', '\u{1E89}'),
    ('\u{78}', '\u{4B3}'),
    ('\u{79}', '\u{1EF5}'),
    ('\u{7A}', '\u{1E93}'),
];

/// Maps ASCII and the signs of [`FULLWIDTH_SIGNS`] to fullwidth forms; reports whether every
/// character had one.
pub(crate) fn fullwidth(text: &str) -> (String, bool) {
    let mut total = true;
    let out = text
        .chars()
        .map(|c| match c {
            '!'..='~' => char::from_u32(u32::from(c) + 0xFEE0).unwrap_or(c),
            c => FULLWIDTH_SIGNS
                .iter()
                .find(|(from, _)| *from == c)
                .map_or_else(
                    || {
                        total = false;
                        c
                    },
                    |(_, to)| *to,
                ),
        })
        .collect();
    (out, total)
}

/// Replaces the first character that has a large-font lookalike; `None` when there is none.
pub(crate) fn big_letter(text: &str) -> Option<String> {
    let (at, c) = text.char_indices().find(|(_, c)| big_of(*c).is_some())?;
    let mut out = String::with_capacity(text.len());
    out.push_str(&text[..at]);
    out.push(big_of(c)?);
    out.push_str(&text[at + c.len_utf8()..]);
    Some(out)
}

fn big_of(c: char) -> Option<char> {
    BIG_LETTERS
        .iter()
        .chain(&BIG_PUNCTUATION)
        .find(|(from, _)| *from == c)
        .map(|(_, to)| *to)
}

/// The accepted characters, for the error message.
fn big_accepted() -> String {
    BIG_LETTERS
        .iter()
        .chain(&BIG_PUNCTUATION)
        .map(|(from, _)| from)
        .collect()
}

/// Lowercase text that the game keeps lowercase.
pub(crate) fn case_sensitive(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        let accent = match c {
            'e' => Some('\u{450}'),
            'n' => Some('\u{1F9}'),
            _ => None,
        };
        match (accent, chars.peek()) {
            (Some(accent), Some(next)) if ACCENT_TRIGGERS.contains(*next) => {
                out.push(accent);
                out.push(*next);
                chars.next();
            }
            _ => out.push(c),
        }
    }
    let out = out
        .replace('\u{E6}', "\u{4D5}")
        .replace("nj", "\u{1CC}")
        .replace(" a ", " \u{FF41} ");
    out.chars()
        .map(|c| {
            CASE_LETTERS
                .iter()
                .find(|(from, _)| *from == c)
                .map_or(c, |(_, to)| *to)
        })
        .collect()
}

/// The pass: applies every modifier to its template.
pub(crate) fn apply(program: &mut Program, ctx: &mut Ctx<'_>) -> Result<(), Diagnostics> {
    let mut errors = Diagnostics::default();
    let tags = crate::tags::enabled(program, ctx.options);
    for rule in &mut program.rules {
        let mut apply = |e: Expr| apply_node(e, tags, ctx, &mut errors);
        for cond in &mut rule.conditions {
            cond.expr = rewrite_expr(&cond.expr, &mut apply);
        }
        rewrite_block(&mut rule.body, &mut apply);
    }
    if errors.has_errors() {
        return Err(errors);
    }
    Ok(())
}

fn rewrite_block(block: &mut crate::ir::Block, apply: &mut impl FnMut(Expr) -> Expr) {
    for stmt in &mut block.stmts {
        map_stmt_roots(stmt, &mut |e| rewrite_expr(e, apply));
        for child in blocks_mut(stmt) {
            rewrite_block(child, apply);
        }
    }
}

fn apply_node(expr: Expr, tags: bool, ctx: &mut Ctx<'_>, errors: &mut Diagnostics) -> Expr {
    let ExprKind::StrModify { modifier, inner } = expr.kind() else {
        return expr;
    };
    let ExprKind::Str(StrExpr::Custom { tokens, args }) = inner.kind() else {
        errors.error(
            ErrorKind::StringModifierTarget,
            "string modifiers only apply to string templates (literals and `ws_fmt!`)".to_owned(),
            expr.loc(),
        );
        return inner.clone();
    };
    let mut parts = segments(tokens, tags);
    match modifier {
        StrModifier::Fullwidth => {
            let mut total = true;
            for (is_tag, part) in parts.iter_mut().flatten() {
                if !*is_tag {
                    let (mapped, ok) = fullwidth(part);
                    total &= ok;
                    *part = mapped;
                }
            }
            if !total {
                ctx.warn(
                    Warning::NotTotalFullwidth,
                    "some characters have no fullwidth form and stay as they are".to_owned(),
                    expr.loc(),
                );
            }
        }
        StrModifier::Big => {
            let done = parts
                .iter_mut()
                .flatten()
                .filter(|(is_tag, _)| !is_tag)
                .any(|(_, part)| big_letter(part).map(|t| *part = t).is_some());
            if !done {
                errors.error(
                    ErrorKind::BigLettersNoMatch,
                    format!(
                        "`.big()` needs one of these characters in the text: {}",
                        big_accepted()
                    ),
                    expr.loc(),
                );
            }
        }
        StrModifier::CaseSensitive => {
            for (is_tag, part) in parts.iter_mut().flatten() {
                if !*is_tag {
                    *part = case_sensitive(part);
                }
            }
        }
    }
    let tokens: Vec<StrToken> = tokens
        .iter()
        .zip(parts)
        .map(|(token, parts)| match token {
            StrToken::Text(_) => StrToken::Text(parts.into_iter().map(|(_, p)| p).collect()),
            StrToken::Arg(i) => StrToken::Arg(*i),
        })
        .collect();
    Expr::new(
        ExprKind::Str(StrExpr::Custom {
            tokens,
            args: args.clone(),
        }),
        expr.loc(),
    )
}

#[cfg(test)]
mod tests;
