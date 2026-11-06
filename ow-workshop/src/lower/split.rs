//! Splits custom strings over 128 characters or 3 placeholders into nested custom strings.
//!
//! The last placeholder of one string holds the remainder of the text.

use crate::ir::{Diagnostics, Expr, ExprKind, Location, Program, StrExpr, StrToken};
use crate::lower::Ctx;
use crate::lower::strings::unnumber;
use crate::lower::visit::{blocks_mut, map_stmt_roots, rewrite_expr};
use crate::opt::always_equal;

/// Most characters (Unicode scalar values) in a custom string, counting `{n}` as 3.
pub(crate) const MAX_CHARS: usize = 128;
/// Most placeholders (and arguments) in a custom string.
pub(crate) const MAX_ARGS: usize = 3;
/// Length of a placeholder, `{0}`.
const PLACEHOLDER: usize = 3;

/// The pass.
#[allow(clippy::unnecessary_wraps)]
pub(crate) fn run(program: &mut Program, _: &mut Ctx<'_>) -> Result<(), Diagnostics> {
    for rule in &mut program.rules {
        for cond in &mut rule.conditions {
            cond.expr = rewrite_expr(&cond.expr, &mut split_node);
        }
        split_block(&mut rule.body);
    }
    Ok(())
}

fn split_block(block: &mut crate::ir::Block) {
    for stmt in &mut block.stmts {
        map_stmt_roots(stmt, &mut |e| rewrite_expr(e, &mut split_node));
        for child in blocks_mut(stmt) {
            split_block(child);
        }
    }
}

fn split_node(expr: Expr) -> Expr {
    let ExprKind::Str(StrExpr::Custom { tokens, args }) = expr.kind() else {
        return expr;
    };
    let (tokens, args) = unnumber(tokens, args);
    split(&Piece::from_tokens(&tokens), &args, expr.loc())
}

/// A token whose argument, if any, is the next unused one.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Piece {
    Text(String),
    Arg,
}

impl Piece {
    fn from_tokens(tokens: &[StrToken]) -> Vec<Self> {
        tokens
            .iter()
            .map(|t| match t {
                StrToken::Text(s) => Self::Text(s.clone()),
                StrToken::Arg(_) => Self::Arg,
            })
            .collect()
    }

    /// Characters it takes in the output text.
    fn width(&self) -> usize {
        match self {
            Self::Text(s) => s.chars().count(),
            Self::Arg => PLACEHOLDER,
        }
    }
}

/// The custom string for `pieces`, nesting as needed; `args` has one entry per `Arg` piece.
fn split(pieces: &[Piece], args: &[Expr], loc: Option<Location>) -> Expr {
    let mut walk = Walk::default();
    let mut arg_i = 0;
    let last = pieces.len().saturating_sub(1);
    for (i, piece) in pieces.iter().enumerate() {
        let is_last = i == last;
        match piece {
            Piece::Text(s) => {
                let reserve = if is_last { 0 } else { PLACEHOLDER };
                let n = s.chars().count();
                if walk.len + n > MAX_CHARS - reserve {
                    let moved = (MAX_CHARS - PLACEHOLDER).saturating_sub(walk.len).min(n);
                    let at = s.char_indices().nth(moved).map_or(s.len(), |(b, _)| b);
                    walk.push_text(&s[..at]);
                    let mut rest = vec![Piece::Text(s[at..].to_owned())];
                    rest.extend_from_slice(&pieces[i + 1..]);
                    return walk.finish_with(&rest, &args[arg_i..], loc);
                }
                walk.push_text(s);
            }
            Piece::Arg => {
                let arg = &args[arg_i];
                if near_end(&walk, pieces, i, is_last)
                    || too_many(&walk, &pieces[i..], &args[arg_i..])
                {
                    return walk.finish_with(&pieces[i..], &args[arg_i..], loc);
                }
                walk.push_arg(arg);
                arg_i += 1;
            }
        }
    }
    walk.finish(loc)
}

/// An argument too close to the end of the text to leave room for the next placeholder.
fn near_end(walk: &Walk, pieces: &[Piece], i: usize, is_last: bool) -> bool {
    if is_last || walk.len + 2 * PLACEHOLDER <= MAX_CHARS {
        return false;
    }
    let fits_last = i + 2 == pieces.len()
        && matches!(&pieces[i + 1], Piece::Text(s)
            if walk.len + PLACEHOLDER + s.chars().count() <= MAX_CHARS);
    !fits_last
}

/// Whether this argument would need a fourth slot, or the third slot's rest would not fit.
fn too_many(walk: &Walk, rest: &[Piece], args: &[Expr]) -> bool {
    if walk.args.len() < MAX_ARGS - 1 || walk.position(&args[0]).is_some() {
        return false;
    }
    let mut distinct: Vec<&Expr> = Vec::new();
    for arg in args {
        if walk.position(arg).is_none() && !distinct.iter().any(|d| always_equal(d, arg)) {
            distinct.push(arg);
        }
    }
    if distinct.len() >= 2 {
        return true;
    }
    walk.len + rest.iter().map(Piece::width).sum::<usize>() > MAX_CHARS
}

/// The string being built.
#[derive(Default)]
struct Walk {
    tokens: Vec<StrToken>,
    args: Vec<Expr>,
    len: usize,
}

impl Walk {
    fn push_text(&mut self, text: &str) {
        if text.is_empty() {
            return;
        }
        self.len += text.chars().count();
        match self.tokens.last_mut() {
            Some(StrToken::Text(prev)) => prev.push_str(text),
            _ => self.tokens.push(StrToken::Text(text.to_owned())),
        }
    }

    /// The index of an argument this one is always equal to.
    fn position(&self, arg: &Expr) -> Option<usize> {
        self.args.iter().position(|a| always_equal(a, arg))
    }

    fn push_arg(&mut self, arg: &Expr) {
        let index = self.position(arg).unwrap_or_else(|| {
            self.args.push(arg.clone());
            self.args.len() - 1
        });
        self.tokens.push(StrToken::Arg(index));
        self.len += PLACEHOLDER;
    }

    /// Ends the string with a placeholder for the nesting of the remainder.
    fn finish_with(mut self, rest: &[Piece], args: &[Expr], loc: Option<Location>) -> Expr {
        let nested = split(rest, args, loc);
        self.tokens.push(StrToken::Arg(self.args.len()));
        self.args.push(nested);
        self.finish(loc)
    }

    fn finish(self, loc: Option<Location>) -> Expr {
        Expr::new(
            ExprKind::Str(StrExpr::Custom {
                tokens: self.tokens,
                args: self.args,
            }),
            loc,
        )
    }
}

#[cfg(test)]
mod tests;
