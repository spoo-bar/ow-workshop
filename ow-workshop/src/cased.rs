//! Cased progress-bar text: pixel-exact lowercase text.
//!
//! The game uppercases text. Overlaying several progress-bar in-world texts, each holding some
//! letters as lowercase lookalike glyphs padded to the right position, renders lowercase text.

use ow_workshop_data::{DEFAULT_WIDTH, char_width, lowercase_metrics};

use crate::block::Block;
use crate::expr::Expr;
use crate::generated::api::CreateProgressBarInWorldTextArgs;
use crate::ir::{ExprKind, StrExpr, StrToken};
use crate::lower::strings::unnumber;
use crate::tags::{Item, find_tags, items};
use crate::ty::{
    Any, Clip, Color, IntoExpr, Num, PlayerOrPosition, Players, ProgressWorldTextReeval,
    SpecVisibility, Str,
};
use crate::width::spaces_for;

/// Fewest and most overlaid texts.
const COUNTS: std::ops::RangeInclusive<usize> = 2..=6;

/// Letters whose lowercase glyph sets the initial offset (not `i`, `j`, `l`, which are narrow).
const OFFSET_LETTERS: &str = "abcdefghkmnopqrstuvwxyz";

/// Why cased text could not be built.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum CasedTextError {
    /// The number of overlaid texts is not between 2 and 6.
    Count(usize),
    /// The text is not a string template (a literal or `ws_fmt!`).
    NotATemplate,
    /// An argument of the text is random, so it would differ between the overlaid texts.
    RandomArgument,
}

impl std::fmt::Display for CasedTextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Count(n) => write!(f, "cased text needs 2 to 6 overlaid texts, not {n}"),
            Self::NotATemplate => f.write_str("cased text needs a string literal or `ws_fmt!`"),
            Self::RandomArgument => f.write_str("cased text cannot have random arguments"),
        }
    }
}

impl std::error::Error for CasedTextError {}

/// Optional arguments of [`Block::cased_progress_bar_text_with`].
#[derive(Clone, Debug)]
pub struct CasedTextArgs {
    /// Who sees the text.
    pub visible_to: Expr<Players>,
    /// Text scale.
    pub scale: Expr<Num>,
    /// Which entities clip the text.
    pub clipping: Expr<Clip>,
    /// The text color.
    pub text_color: Expr<Color>,
    /// What the text re-evaluates.
    pub reevaluation: Expr<ProgressWorldTextReeval>,
    /// Whether spectators see it.
    pub spec_visibility: Expr<SpecVisibility>,
}

impl Default for CasedTextArgs {
    fn default() -> Self {
        let d = CreateProgressBarInWorldTextArgs::default();
        Self {
            visible_to: d.visible_to,
            scale: d.scale,
            clipping: d.clipping,
            text_color: d.text_color,
            reevaluation: d.reevaluation,
            spec_visibility: d.spec_visibility,
        }
    }
}

impl Block {
    /// Creates `n` (2 to 6) progress-bar in-world texts that together render `text` in
    /// lowercase, in place of one `Create Progress Bar In-World Text`.
    ///
    /// `text` must be a string literal or `ws_fmt!` without random arguments.
    ///
    /// # Errors
    /// When `n` is out of range or `text` is not such a template.
    #[track_caller]
    pub fn cased_progress_bar_text(
        &mut self,
        n: usize,
        text: Expr<Str>,
        position: impl IntoExpr<PlayerOrPosition>,
    ) -> Result<(), CasedTextError> {
        self.cased_progress_bar_text_with(n, text, position, CasedTextArgs::default())
    }

    /// [`cased_progress_bar_text`](Self::cased_progress_bar_text) with the optional arguments.
    ///
    /// # Errors
    /// When `n` is out of range or `text` is not a template without random arguments.
    #[track_caller]
    pub fn cased_progress_bar_text_with(
        &mut self,
        n: usize,
        text: Expr<Str>,
        position: impl IntoExpr<PlayerOrPosition>,
        opts: CasedTextArgs,
    ) -> Result<(), CasedTextError> {
        let texts = cased_texts(n, text)?;
        let position = position.into_expr();
        let CasedTextArgs {
            visible_to,
            scale,
            clipping,
            text_color,
            reevaluation,
            spec_visibility,
        } = opts;
        for (k, text) in texts.into_iter().enumerate() {
            let percentage = u32::try_from(k + 1).unwrap_or(u32::MAX);
            self.create_progress_bar_in_world_text_with(
                position.clone(),
                CreateProgressBarInWorldTextArgs {
                    visible_to: visible_to.clone(),
                    value: percentage.into(),
                    text,
                    scale: scale.clone(),
                    clipping: clipping.clone(),
                    progress_bar_color: Color::White.into(),
                    text_color: text_color.clone(),
                    reevaluation: reevaluation.clone(),
                    spec_visibility: spec_visibility.clone(),
                },
            );
        }
        Ok(())
    }
}

/// The `n` strings to overlay.
#[track_caller]
fn cased_texts(n: usize, text: Expr<Str>) -> Result<Vec<Expr<Any>>, CasedTextError> {
    if !COUNTS.contains(&n) {
        return Err(CasedTextError::Count(n));
    }
    let text = text.into_ir();
    let ExprKind::Str(StrExpr::Custom { tokens, args }) = text.kind() else {
        return Err(CasedTextError::NotATemplate);
    };
    if args.iter().any(crate::opt::contains_random) {
        return Err(CasedTextError::RandomArgument);
    }
    let (tokens, args) = unnumber(tokens, args);
    let outputs = cased_template(&tokens, n);
    Ok(outputs
        .into_iter()
        .map(|pieces| {
            let mut out_tokens = Vec::new();
            let mut out_args = Vec::new();
            for piece in pieces {
                match piece {
                    Piece::Text(t) => crate::fmt::push_text(&mut out_tokens, &t),
                    Piece::Arg(i) => {
                        out_tokens.push(StrToken::Arg(out_args.len()));
                        out_args.push(args[i].clone());
                    }
                }
            }
            Expr::<Str>::from_ir(crate::ir::Expr::here(ExprKind::Str(StrExpr::Custom {
                tokens: out_tokens,
                args: out_args,
            })))
            .erase()
        })
        .collect())
}

/// Text or a placeholder of the template's arguments.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Piece {
    Text(String),
    Arg(usize),
}

/// What a line is made of.
#[derive(Clone, Debug)]
enum Unit {
    Ch(char),
    /// A `<fg…>` or `</fg>` tag: zero width, in every output.
    FgTag(String),
    /// A `<tx…>` tag or a placeholder: unknown width, in one output.
    Opaque(Vec<Piece>),
}

/// The overlaid strings for a whole template, lines joined with newlines.
pub(crate) fn cased_template(tokens: &[StrToken], n: usize) -> Vec<Vec<Piece>> {
    // Each newline gets two spaces on both sides.
    let mut all = Vec::new();
    for item in items(tokens) {
        match item {
            Item::Ch('\n') => all.extend([' ', ' ', '\n', ' ', ' '].map(Item::Ch)),
            other => all.push(other),
        }
    }
    let mut outputs: Vec<Vec<Piece>> = vec![Vec::new(); n];
    for (line, segment) in all.split(|i| *i == Item::Ch('\n')).enumerate() {
        if line > 0 {
            for output in &mut outputs {
                push_text(output, "\n");
            }
        }
        for (output, mut part) in outputs.iter_mut().zip(cased_line(&units(segment), n)) {
            output.append(&mut part);
            // Trailing spaces are trimmed otherwise.
            push_text(output, "\u{AD}");
        }
    }
    outputs
}

fn push_text(pieces: &mut Vec<Piece>, text: &str) {
    match pieces.last_mut() {
        Some(Piece::Text(prev)) => prev.push_str(text),
        _ => pieces.push(Piece::Text(text.to_owned())),
    }
}

fn units(line: &[Item]) -> Vec<Unit> {
    let tags = find_tags(line);
    let text_of = |range: std::ops::Range<usize>| -> Vec<Piece> {
        let mut pieces = Vec::new();
        for item in &line[range] {
            match item {
                Item::Ch(c) => push_text(&mut pieces, &c.to_string()),
                Item::Arg(i) => pieces.push(Piece::Arg(*i)),
            }
        }
        pieces
    };
    let mut out = Vec::new();
    let mut i = 0;
    while i < line.len() {
        if let Some(tag) = tags.iter().find(|r| r.start == i) {
            let pieces = text_of(tag.clone());
            let is_fg = matches!(line.get(i + 1), Some(Item::Ch('f' | 'F' | '/')));
            out.push(match pieces.as_slice() {
                [Piece::Text(t)] if is_fg => Unit::FgTag(t.clone()),
                _ => Unit::Opaque(pieces),
            });
            i = tag.end;
            continue;
        }
        out.push(match line[i] {
            Item::Ch(c) => Unit::Ch(c),
            Item::Arg(a) => Unit::Opaque(vec![Piece::Arg(a)]),
        });
        i += 1;
    }
    out
}

/// One output being built.
#[derive(Clone, Default)]
struct Output {
    pieces: Vec<Piece>,
    /// Where the pen is, in font units.
    width: i64,
}

impl Output {
    fn pad_to(&mut self, target: i64) {
        if let Ok(missing) = u32::try_from(target - self.width) {
            if missing > 0 {
                push_text(&mut self.pieces, &spaces_for(missing));
                self.width = target;
            }
        }
    }
}

/// The `n` strings for one line whose lowercase letters are drawn as lookalike glyphs.
fn cased_line(line: &[Unit], n: usize) -> Vec<Vec<Piece>> {
    let has_letter = line
        .iter()
        .any(|u| matches!(u, Unit::Ch(c) if lowercase_metrics(*c).is_some()));
    if !has_letter {
        let plain: Vec<Piece> = line.iter().flat_map(unit_pieces).collect();
        return vec![plain; n];
    }
    let mut text_width = initial_offset(line);
    let mut outs = vec![Output::default(); n];
    let mut k = n - 1;
    let mut after_letter_or_space = false;
    for unit in line {
        match unit {
            Unit::FgTag(tag) => {
                for out in &mut outs {
                    push_text(&mut out.pieces, tag);
                }
                k = (k + 1) % n;
                continue;
            }
            Unit::Opaque(pieces) => {
                if outs[k].pieces.is_empty() || after_letter_or_space {
                    k = (k + 1) % n;
                    outs[k].pad_to(text_width);
                }
                outs[k].pieces.extend(pieces.iter().cloned());
                outs[k].width += i64::from(DEFAULT_WIDTH);
                after_letter_or_space = false;
                text_width += i64::from(DEFAULT_WIDTH);
                continue;
            }
            Unit::Ch(' ') => after_letter_or_space = true,
            Unit::Ch(c) => {
                if let Some(glyph) = lowercase_metrics(*c) {
                    k = (k + 1) % n;
                    let at = text_width - i64::from(glyph.lower_xmin) + i64::from(glyph.xmin);
                    let target = outs[k].width.max(at);
                    outs[k].pad_to(target);
                    push_text(&mut outs[k].pieces, &glyph.lower.to_string());
                    outs[k].width += i64::from(glyph.lower_width);
                    after_letter_or_space = true;
                } else {
                    if outs[k].pieces.is_empty() || after_letter_or_space {
                        k = (k + 1) % n;
                        outs[k].pad_to(text_width);
                    }
                    push_text(&mut outs[k].pieces, &c.to_string());
                    outs[k].width += i64::from(char_width(*c));
                    after_letter_or_space = false;
                }
            }
        }
        if let Unit::Ch(c) = unit {
            text_width += i64::from(char_width(*c));
        }
    }
    let widest = outs.iter().map(|o| o.width).max().unwrap_or(0);
    outs.into_iter()
        .map(|mut o| {
            o.pad_to(widest);
            o.pieces
        })
        .collect()
}

fn unit_pieces(unit: &Unit) -> Vec<Piece> {
    match unit {
        Unit::Ch(c) => vec![Piece::Text(c.to_string())],
        Unit::FgTag(t) => vec![Piece::Text(t.clone())],
        Unit::Opaque(p) => p.clone(),
    }
}

/// Room the first lowercase glyph needs before the text starts.
fn initial_offset(line: &[Unit]) -> i64 {
    let mut before = 0_i64;
    for unit in line {
        match unit {
            Unit::Ch(c) if OFFSET_LETTERS.contains(*c) => {
                let bearing = lowercase_metrics(*c).map_or(0, |g| i64::from(g.lower_xmin));
                return (bearing - before).max(0);
            }
            Unit::Ch(c) => before += i64::from(char_width(*c)),
            Unit::Opaque(_) => before += i64::from(DEFAULT_WIDTH),
            Unit::FgTag(_) => {}
        }
    }
    0
}

#[cfg(test)]
mod tests;
