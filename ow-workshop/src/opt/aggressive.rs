//! Whole-program size rewrites that trade readability for elements.
//!
//! They only run when both `size` and `aggressive_size` are on.

use ow_workshop_data::ValueId as V;

use super::Env;
use super::build::value;
use super::helpers::{always_equal, contains_lambda_vars};
use super::logic::inverse;
use crate::check::is_lambda;
use crate::ir::{CmpOp, Expr, ExprKind, SkipTarget, Stmt, StmtKind, StrExpr, StrToken};
use crate::lower::visit::map_children;

/// Longest custom string the game accepts.
const MAX_STRING: usize = 128;
/// A separator that reads as `0` when stringified (`First Of(Null)`).
const ZERO: &str = "0";
/// A separator that reads like `Left` when stringified (`First Of(Left)`).
const LEFT: &str = "(1.00, 0.00, 0.00)";
/// A separator no real text uses.
const PRIVATE: &str = "\u{EC51}";

// ---- If → Skip If -------------------------------------------------------------------------

/// `If(cond) body End` → `Skip If(!cond, L) body L:` where that saves the `End`.
///
/// A negated condition is free for `Not(c)` and for comparisons; a comparison only pays off
/// when the `If` has a single child.
pub(super) fn skip_if(stmt: Stmt, env: &mut Env<'_, '_>) -> Vec<Stmt> {
    if !env.scope.aggressive_size() || env.dynamic_skips {
        return vec![stmt];
    }
    let StmtKind::If {
        cond,
        then,
        elifs,
        else_: None,
    } = &stmt.kind
    else {
        return vec![stmt];
    };
    if !elifs.is_empty() || then.opt.is_some() || then.stmts.is_empty() {
        return vec![stmt];
    }
    let skip_when = match cond.kind() {
        ExprKind::Not(inner) => inner.clone(),
        ExprKind::Compare { op, lhs, rhs } if then.stmts.len() == 1 => Expr::new(
            ExprKind::Compare {
                op: inverse(*op),
                lhs: lhs.clone(),
                rhs: rhs.clone(),
            },
            cond.loc(),
        ),
        _ => return vec![stmt],
    };
    let label = env.ctx.fresh_label();
    let at = |kind| Stmt {
        kind,
        loc: stmt.loc,
        comment: None,
    };
    let mut out = vec![Stmt {
        kind: StmtKind::SkipIf(skip_when, SkipTarget::Label(label)),
        loc: stmt.loc,
        comment: stmt.comment.clone(),
    }];
    out.extend(then.stmts.iter().cloned());
    // The `Pass` after the label keeps the skip's landing spot a statement of its own.
    out.push(at(StmtKind::Label(label)));
    out.push(at(StmtKind::Pass));
    out
}

// ---- arrays -------------------------------------------------------------------------------

fn custom(text: &str, like: &Expr) -> Expr {
    Expr::new(
        ExprKind::Str(StrExpr::Custom {
            tokens: vec![StrToken::Text(text.to_owned())],
            args: Vec::new(),
        }),
        like.loc(),
    )
}

/// The text of a custom string without placeholders.
fn plain_text(e: &Expr) -> Option<&str> {
    match e.kind() {
        ExprKind::Str(StrExpr::Custom { tokens, .. }) => match tokens.as_slice() {
            [StrToken::Text(text)] if !text.is_empty() => Some(text),
            _ => None,
        },
        _ => None,
    }
}

/// `["a", "b", …]` as one string split at a separator that stringifies cheaply.
fn string_array(e: &Expr, items: &[Expr]) -> Option<Expr> {
    if items.len() < 2 {
        return None;
    }
    let texts: Vec<&str> = items.iter().map(plain_text).collect::<Option<_>>()?;
    let fits = |sep: &str| {
        let len = texts.iter().map(|t| t.chars().count()).sum::<usize>()
            + sep.chars().count() * (texts.len() - 1);
        len <= MAX_STRING && texts.iter().all(|t| !t.contains(sep))
    };
    let free = |sep: &str| texts.iter().all(|t| !t.contains(sep));
    let loc = e.loc();
    let null = || Expr::new(ExprKind::Null, loc);
    // Elements an array costs: itself plus one per item; the split costs 4 (3 with a string).
    let (sep, sep_value, split_cost) = if free(ZERO) {
        (ZERO, value(V::FirstOf, vec![null()], loc), 4)
    } else if free(LEFT) {
        (
            LEFT,
            value(V::FirstOf, vec![value(V::VectorLeft, Vec::new(), loc)], loc),
            4,
        )
    } else {
        (PRIVATE, custom(PRIVATE, e), 3)
    };
    if !fits(sep) || items.len() < split_cost {
        return None;
    }
    Some(value(
        V::Split,
        vec![custom(&texts.join(sep), e), sep_value],
        loc,
    ))
}

/// A rough element count of a value: itself plus its arguments; strings count 4.
fn element_cost(e: &Expr) -> usize {
    match e.kind() {
        ExprKind::Str(_) => 4,
        kind => 1 + kind.children().len(),
    }
}

/// How many identical elements make a mapped array cheaper than listing them.
fn repeat_threshold(cost: usize) -> usize {
    match cost {
        0 | 1 => 11,
        2 => 5,
        3 => 4,
        _ => 3,
    }
}

/// `[x, x, x, …]` as a map over a string split into that many empty pieces.
fn repeated_array(e: &Expr, items: &[Expr]) -> Option<Expr> {
    let first = items.first()?;
    if contains_lambda_vars(first)
        || items.len() > MAX_STRING
        || items.len() < repeat_threshold(element_cost(first))
        || !items.iter().all(|x| always_equal(x, first))
    {
        return None;
    }
    let loc = e.loc();
    let pieces = value(
        V::Split,
        vec![
            custom(&ZERO.repeat(items.len() - 1), e),
            value(V::FirstOf, vec![Expr::new(ExprKind::Null, loc)], loc),
        ],
        loc,
    );
    Some(value(V::MappedArray, vec![pieces, first.clone()], loc))
}

fn array_rule(e: &Expr, in_lambda: bool) -> Option<Expr> {
    let ExprKind::Array(items) = e.kind() else {
        return None;
    };
    string_array(e, items).or_else(|| {
        // A mapped array inside a lambda would rebind the element and index.
        (!in_lambda).then(|| repeated_array(e, items)).flatten()
    })
}

fn is_workshop_combo(id: V) -> bool {
    id == V::CreateWorkshopSettingEnum
}

fn walk(e: &Expr, in_lambda: bool) -> Expr {
    if let Some(new) = array_rule(e, in_lambda) {
        return new;
    }
    match e.kind() {
        // The options of a combo setting must stay a list of strings.
        ExprKind::Value { id, .. } if is_workshop_combo(*id) => e.clone(),
        ExprKind::Value { id, args } => {
            let lambda = is_lambda(*id);
            let args = args
                .iter()
                .enumerate()
                .map(|(i, a)| walk(a, in_lambda || (lambda && i > 0)))
                .collect();
            Expr::new(ExprKind::Value { id: *id, args }, e.loc())
        }
        kind => Expr::new(map_children(kind, &mut |c| walk(c, in_lambda)), e.loc()),
    }
}

/// Packs literal arrays: string lists and long runs of one value.
pub(super) fn arrays(e: &Expr) -> Expr {
    walk(e, false)
}

#[cfg(test)]
mod tests;
