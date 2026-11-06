//! Built-in localized strings: `String("phrase", a, b, c)`.
//!
//! Legacy: custom strings are almost always better. The game translates the phrase.

use ow_workshop_data::LocalizedString;

use crate::expr::Expr;
use crate::ir::{self, ExprKind};
use crate::ty::{Any, IntoExpr, Str};

/// A localized string with up to three placeholder arguments.
#[track_caller]
#[must_use]
pub fn localized<A: IntoExpr<Any>>(
    phrase: LocalizedString,
    args: impl IntoIterator<Item = A>,
) -> Expr<Str> {
    call(Ok(phrase), args)
}

/// A localized string looked up by its English text, e.g. `localized_en("{0} - {1}", [a, b])`.
///
/// Text that is not a built-in phrase is the error `unknown-localized-string`.
#[track_caller]
#[must_use]
pub fn localized_en<A: IntoExpr<Any>>(text: &str, args: impl IntoIterator<Item = A>) -> Expr<Str> {
    call(
        LocalizedString::from_en(text).ok_or_else(|| text.to_owned()),
        args,
    )
}

#[track_caller]
fn call<A: IntoExpr<Any>>(
    phrase: Result<LocalizedString, String>,
    args: impl IntoIterator<Item = A>,
) -> Expr<Str> {
    let args = args.into_iter().map(|a| a.into_expr().into_ir()).collect();
    Expr::from_ir(ir::Expr::here(ExprKind::LocalizedCall { phrase, args }))
}
