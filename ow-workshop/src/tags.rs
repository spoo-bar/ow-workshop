//! Text tags: `<fgRRGGBBAA>…</fg>` colors and `<txC…>` textures.
//!
//! The game sanitizes a literal `<`, so the compiler reads a raw one at runtime into the global
//! `__holygrail__` and substitutes it for the `<` of each tag.

use std::ops::Range;

use ow_workshop_data::Texture;

use crate::expr::Expr;
use crate::ir::{self, ExprKind, Program, StrExpr, StrToken, VarDecl, VarFlags, VarId};
use crate::options::CompileOptions;
use crate::ty::{Any, IntoExpr, Str};

/// The global that holds the raw `<`.
pub(crate) const HOLYGRAIL: &str = "__holygrail__";

/// Hex digits of a texture id after the `C` of `<txC…>`.
const TX_DIGITS: usize = 14;

/// One character or placeholder of a template.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Item {
    /// A character of the text.
    Ch(char),
    /// A placeholder, by argument index.
    Arg(usize),
}

/// The template as a flat list of characters and placeholders.
pub(crate) fn items(tokens: &[StrToken]) -> Vec<Item> {
    tokens
        .iter()
        .flat_map(|t| match t {
            StrToken::Text(text) => text.chars().map(Item::Ch).collect::<Vec<_>>(),
            StrToken::Arg(i) => vec![Item::Arg(*i)],
        })
        .collect()
}

/// Item ranges of the tags in `items`, in order and without overlap.
pub(crate) fn find_tags(items: &[Item]) -> Vec<Range<usize>> {
    let mut found = Vec::new();
    let mut i = 0;
    while i < items.len() {
        match (items[i], tag_len(&items[i..])) {
            (Item::Ch('<'), Some(len)) => {
                found.push(i..i + len);
                i += len;
            }
            _ => i += 1,
        }
    }
    found
}

/// The length of the tag starting at `items[0]`, a `<`.
fn tag_len(items: &[Item]) -> Option<usize> {
    let lower = |k: usize| match items.get(k) {
        Some(Item::Ch(c)) => Some(c.to_ascii_lowercase()),
        _ => None,
    };
    if [lower(1), lower(2), lower(3), lower(4)] == [Some('/'), Some('f'), Some('g'), Some('>')] {
        return Some(5);
    }
    let fg = match (lower(1), lower(2)) {
        (Some('t'), Some('x')) => false,
        (Some('f'), Some('g')) => true,
        _ => return None,
    };
    let mut k = 3;
    while lower(k) == Some(' ') {
        k += 1;
    }
    if fg && lower(k) == Some('#') {
        k += 1;
    }
    let digits = k;
    while matches!(items.get(k), Some(Item::Arg(_)))
        || lower(k).is_some_and(|c| c.is_ascii_hexdigit())
    {
        k += 1;
    }
    (k > digits && lower(k) == Some('>')).then_some(k + 1)
}

/// Text split into `(is_tag, part)` pieces, one list per token.
pub(crate) fn segments(tokens: &[StrToken], enabled: bool) -> Vec<Vec<(bool, String)>> {
    let all = items(tokens);
    let tags = if enabled { find_tags(&all) } else { Vec::new() };
    let in_tag = |i: usize| tags.iter().any(|r| r.contains(&i));
    let mut at = 0;
    tokens
        .iter()
        .map(|token| match token {
            StrToken::Arg(_) => {
                at += 1;
                Vec::new()
            }
            StrToken::Text(text) => {
                let mut parts: Vec<(bool, String)> = Vec::new();
                for c in text.chars() {
                    let tag = in_tag(at);
                    at += 1;
                    match parts.last_mut() {
                        Some((last, part)) if *last == tag => part.push(c),
                        _ => parts.push((tag, c.to_string())),
                    }
                }
                parts
            }
        })
        .collect()
}

/// The result of looking at the text right after a `__holygrail__` argument.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum TxFix {
    /// Not a short `<tx…>` id.
    None,
    /// The first `len` bytes become this text.
    Fixed(usize, String),
    /// A `txC…` id without exactly 14 digits.
    Malformed,
}

/// `tx<id>>` → `txC<id padded to 14 digits>>`.
pub(crate) fn tx_fix(text: &str) -> TxFix {
    let bytes = text.as_bytes();
    if bytes.len() < 2 || !bytes[..2].eq_ignore_ascii_case(b"tx") {
        return TxFix::None;
    }
    let spaces = text[2..].bytes().take_while(|b| *b == b' ').count();
    let rest = &text[2 + spaces..];
    let run = rest.bytes().take_while(u8::is_ascii_hexdigit).count();
    if run == 0 || rest.as_bytes().get(run) != Some(&b'>') {
        return TxFix::None;
    }
    let id = rest[..run].trim_start_matches('0');
    let consumed = 2 + spaces + run + 1;
    if id.len() <= 6 {
        let id = if id.is_empty() { "0" } else { id };
        return TxFix::Fixed(consumed, format!("txC{id:0>TX_DIGITS$}>"));
    }
    match id.strip_prefix(['c', 'C']) {
        Some(digits) if digits.len() >= 6 && digits.len() != TX_DIGITS => TxFix::Malformed,
        _ => TxFix::None,
    }
}

/// Whether the program uses tags: the option, or any texture.
pub(crate) fn enabled(program: &Program, options: &CompileOptions) -> bool {
    options.tags || uses_holygrail(program)
}

fn uses_holygrail(program: &Program) -> bool {
    fn deep(e: &ir::Expr) -> bool {
        matches!(e.kind(), ExprKind::HolyGrail) || e.kind().children().into_iter().any(deep)
    }
    let mut found = program
        .rules
        .iter()
        .any(|r| r.conditions.iter().any(|c| deep(&c.expr)));
    for rule in &program.rules {
        crate::lower::visit::walk_block(&rule.body, &mut |stmt| {
            found |= stmt.exprs().into_iter().any(deep);
        });
    }
    found
}

/// The id of `__holygrail__`, once declared.
pub(crate) fn holygrail_var(program: &Program) -> Option<VarId> {
    let at = program
        .globals
        .entries
        .iter()
        .position(|d| d.name == HOLYGRAIL)?;
    Some(VarId(u32::try_from(at).ok()?))
}

/// Declares `__holygrail__` when tags are enabled.
pub(crate) fn declare(program: &mut Program, options: &CompileOptions) {
    if enabled(program, options) && holygrail_var(program).is_none() {
        program.globals.declare(VarDecl {
            name: HOLYGRAIL.to_owned(),
            index: None,
            init: None,
            internal: true,
            flags: VarFlags::default(),
            location: None,
        });
    }
}

impl From<Texture> for Expr<Str> {
    /// `Custom String("{0}<rest of the tag>", Global.__holygrail__)`.
    fn from(texture: Texture) -> Self {
        let rest = texture.tag().strip_prefix('<').unwrap_or(texture.tag());
        Self::literal(ExprKind::Str(StrExpr::Custom {
            tokens: vec![StrToken::Arg(0), StrToken::Text(rest.to_owned())],
            args: vec![ir::Expr::new(ExprKind::HolyGrail, None)],
        }))
    }
}

impl IntoExpr<Str> for Texture {
    fn into_expr(self) -> Expr<Str> {
        self.into()
    }
}

impl IntoExpr<Any> for Texture {
    fn into_expr(self) -> Expr<Any> {
        Expr::<Str>::from(self).erase()
    }
}

#[cfg(test)]
mod tests;
