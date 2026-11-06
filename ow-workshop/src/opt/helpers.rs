//! Predicates the rewrite rules share.

use ow_workshop_data::{Lang, ValueId};

use crate::ir::{Expr, ExprKind, StrExpr, StrToken};
use crate::lower::visit::rewrite_expr;

/// Folds never produce a number whose magnitude exceeds this: the game loses precision.
pub(crate) const MAX_FOLD: f64 = 10_000_000.0;

/// `Some(x)` unless folding to `x` would lose precision in game.
pub(crate) fn fold_guard(x: f64) -> Option<f64> {
    (x.is_finite() && x.abs() <= MAX_FOLD).then_some(x)
}

/// Enum names whose members are literals that are always truthy.
const TRUTHY_ENUMS: [&str; 6] = ["Hero", "Map", "Gamemode", "Team", "Button", "Color"];
/// Values that build such literals.
const TRUTHY_VALUES: [ValueId; 7] = [
    ValueId::Hero,
    ValueId::Map,
    ValueId::Gamemode,
    ValueId::Team,
    ValueId::Button,
    ValueId::Color,
    ValueId::Rgb,
];

fn value_id(expr: &Expr) -> Option<ValueId> {
    match expr.kind() {
        ExprKind::Value { id, .. } => Some(*id),
        _ => None,
    }
}

/// The elements of an array expression, if it is one.
pub(super) fn array_items(expr: &Expr) -> Option<&[Expr]> {
    match expr.kind() {
        ExprKind::Array(items) => Some(items),
        ExprKind::Value {
            id: ValueId::Array,
            args,
        } => Some(args),
        ExprKind::Value {
            id: ValueId::EmptyArray,
            ..
        } => Some(&[]),
        _ => None,
    }
}

/// Whether `text` has anything besides `{n}` placeholders.
fn has_real_text(text: &str) -> bool {
    let mut rest = text;
    while !rest.is_empty() {
        if let Some(after) = rest.strip_prefix('{') {
            let digits = after.chars().take_while(char::is_ascii_digit).count();
            if digits > 0 && after[digits..].starts_with('}') {
                rest = &after[digits + 1..];
                continue;
            }
        }
        return true;
    }
    false
}

/// Whether `expr` is a value that is certainly falsy in a condition.
///
/// `Vector(0,0,0)` is deliberately not falsy: treating it so breaks some rewrites.
pub(crate) fn definitely_falsy(expr: &Expr) -> bool {
    if let Some(items) = array_items(expr) {
        return items.first().is_none_or(definitely_falsy);
    }
    match expr.kind() {
        ExprKind::Bool(false)
        | ExprKind::Null
        | ExprKind::Value {
            id: ValueId::False | ValueId::Null,
            ..
        } => true,
        ExprKind::Num(n) => *n == 0.0,
        ExprKind::Str(StrExpr::Custom { tokens, .. }) => tokens
            .iter()
            .all(|t| matches!(t, StrToken::Text(text) if text.is_empty())),
        ExprKind::Str(StrExpr::Localized(s)) => s.text(Lang::EnUs).is_empty(),
        _ => false,
    }
}

/// Whether `expr` is a value that is certainly truthy in a condition.
///
/// Not the inverse of [`definitely_falsy`]: usually neither is known.
pub(crate) fn definitely_truthy(expr: &Expr) -> bool {
    if let Some(items) = array_items(expr) {
        return items.first().is_some_and(definitely_truthy);
    }
    match expr.kind() {
        ExprKind::Bool(true) => true,
        ExprKind::Num(n) => *n != 0.0,
        ExprKind::Vector(parts) => parts.iter().any(definitely_truthy),
        ExprKind::Const(c) => TRUTHY_ENUMS.contains(&c.enum_name),
        ExprKind::Value { id, .. } => *id == ValueId::True || TRUTHY_VALUES.contains(id),
        ExprKind::Str(StrExpr::Custom { tokens, .. }) => tokens
            .iter()
            .any(|t| matches!(t, StrToken::Text(text) if has_real_text(text))),
        ExprKind::Str(StrExpr::Localized(s)) => has_real_text(s.text(Lang::EnUs)),
        _ => false,
    }
}

const RANDOM: [ValueId; 4] = [
    ValueId::RandomRandint,
    ValueId::RandomUniform,
    ValueId::RandomChoice,
    ValueId::RandomShuffle,
];

/// Whether `expr` reads a random number or shuffles randomly anywhere inside it.
pub(crate) fn contains_random(expr: &Expr) -> bool {
    value_id(expr).is_some_and(|id| RANDOM.contains(&id))
        || expr.kind().children().into_iter().any(contains_random)
}

/// Whether `expr` uses `Current Array Element` or `Current Array Index` anywhere inside it.
pub(crate) fn contains_lambda_vars(expr: &Expr) -> bool {
    matches!(expr.kind(), ExprKind::ArrayElem | ExprKind::ArrayIndex)
        || expr.kind().children().into_iter().any(contains_lambda_vars)
}

/// The expression without source locations, so equal shapes compare equal.
fn stripped(expr: &Expr) -> Expr {
    rewrite_expr(expr, &mut |e| Expr::new(e.kind().clone(), None))
}

/// Structural equality of the IR; never true for expressions that read randomness.
pub(crate) fn always_equal(a: &Expr, b: &Expr) -> bool {
    !contains_random(a) && !contains_random(b) && stripped(a) == stripped(b)
}

/// Whether `expr` is a literal: equal literals are the same value, different ones differ.
///
/// A custom string without placeholders counts only when not `strict`: some languages
/// censor words, so `"am" == "**"` can be true in Turkish.
pub(crate) fn is_literal(expr: &Expr, strict: bool) -> bool {
    match expr.kind() {
        ExprKind::Num(_) | ExprKind::Const(_) | ExprKind::Str(StrExpr::Localized(_)) => true,
        ExprKind::Str(StrExpr::Custom { tokens, .. }) => {
            !strict && tokens.iter().all(|t| matches!(t, StrToken::Text(_)))
        }
        ExprKind::Vector(parts) => parts.iter().all(|p| is_literal(p, strict)),
        ExprKind::Value { id, args } => {
            id.signature().flags.is_literal() && args.iter().all(|a| is_literal(a, strict))
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests;
