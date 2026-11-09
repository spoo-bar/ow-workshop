//! Translated strings: `t!`, `t_ctx!` and the functions that resolve a stored one.

use crate::expr::Expr;
use crate::ir::{self, ExprKind, Translated};
use crate::ty::{Any, Array, IntoExpr, Str};

/// A string translated into each language of [`Translations`](crate::Translations):
/// `t!("You have {} money", money)`.
///
/// Where the viewer's language is known (a reevaluated argument of a display action) it shows
/// the viewer's language. Assigned with `set` it becomes an array; show it with [`tr`].
#[macro_export]
macro_rules! t {
    ($text:literal $(, $arg:expr)* $(,)?) => {
        $crate::__private::translated(
            $text,
            ::core::option::Option::None,
            ::std::vec![$($crate::IntoExpr::<$crate::Any>::into_expr($arg)),*],
        )
    };
}

/// Like [`t!`], with a context that tells apart identical texts: `t_ctx!("direction", "Left")`.
#[macro_export]
macro_rules! t_ctx {
    ($context:literal, $text:literal $(, $arg:expr)* $(,)?) => {
        $crate::__private::translated(
            $text,
            ::core::option::Option::Some($context),
            ::std::vec![$($crate::IntoExpr::<$crate::Any>::into_expr($arg)),*],
        )
    };
}

/// Builds the node for `t!` and `t_ctx!`.
#[doc(hidden)]
#[track_caller]
#[must_use]
pub fn translated(text: &str, context: Option<&str>, args: Vec<Expr<Any>>) -> Expr<Str> {
    Expr::from_ir(ir::Expr::here(ExprKind::Translated(Translated {
        key: text.to_owned(),
        context: context.map(str::to_owned),
        args: args.into_iter().map(Expr::into_ir).collect(),
        raw: false,
        spectator: false,
    })))
}

/// Resolves a stored translated string to the viewer's language, for a reevaluated argument.
#[track_caller]
#[must_use]
pub fn tr(value: impl IntoExpr<Any>) -> Expr<Str> {
    resolve(value.into_expr(), false)
}

/// Like [`tr`], but for spectators: it ignores the language stored in the player variable.
#[track_caller]
#[must_use]
pub fn tr_spectator(value: impl IntoExpr<Any>) -> Expr<Str> {
    resolve(value.into_expr(), true)
}

/// The array of every translation of `value`, even in a display action; resolve it later with [`tr`].
#[track_caller]
#[must_use]
pub fn tr_raw(value: Expr<Str>) -> Expr<Array<Str>> {
    let ir = value.into_ir();
    let kind = match ir.kind() {
        ExprKind::Translated(t) => ExprKind::Translated(Translated {
            raw: true,
            ..t.clone()
        }),
        other => other.clone(),
    };
    Expr::from_ir(ir::Expr::new(kind, ir.loc()))
}

#[track_caller]
fn resolve(value: Expr<Any>, spectator: bool) -> Expr<Str> {
    let ir = value.into_ir();
    let kind = match ir.kind() {
        // A string written in place is shown as is.
        ExprKind::Translated(t) => ExprKind::Translated(Translated {
            raw: false,
            spectator,
            ..t.clone()
        }),
        _ => ExprKind::Resolve {
            inner: ir.clone(),
            spectator,
        },
    };
    Expr::from_ir(ir::Expr::new(
        kind,
        ir.loc().or_else(|| Some(ir::Location::caller())),
    ))
}

#[cfg(test)]
mod tests;
