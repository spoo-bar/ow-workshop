//! Lowers `localized(..)` calls to `String("phrase", a, b, c)`, padding the arguments with `Null`.

use crate::ir::ErrorKind;
use ow_workshop_data::ValueId;

use crate::ir::{Diagnostics, Expr, ExprKind, Program, StrExpr};
use crate::lower::Ctx;
use crate::lower::visit::{blocks_mut, map_stmt_roots, rewrite_expr};

/// Placeholders a localized string has.
const SLOTS: usize = 3;

/// The pass.
#[allow(clippy::unnecessary_wraps)]
pub(crate) fn lower(program: &mut Program, _: &mut Ctx<'_>) -> Result<(), Diagnostics> {
    let mut errors = Diagnostics::default();
    let mut node = |e: Expr| lower_node(e, &mut errors);
    for rule in &mut program.rules {
        for cond in &mut rule.conditions {
            cond.expr = rewrite_expr(&cond.expr, &mut node);
        }
        lower_block(&mut rule.body, &mut node);
    }
    if errors.has_errors() {
        return Err(errors);
    }
    Ok(())
}

fn lower_block(block: &mut crate::ir::Block, node: &mut impl FnMut(Expr) -> Expr) {
    for stmt in &mut block.stmts {
        map_stmt_roots(stmt, &mut |e| rewrite_expr(e, node));
        for child in blocks_mut(stmt) {
            lower_block(child, node);
        }
    }
}

fn lower_node(expr: Expr, errors: &mut Diagnostics) -> Expr {
    let ExprKind::LocalizedCall { phrase, args } = expr.kind() else {
        return expr;
    };
    let phrase = match phrase {
        Ok(phrase) => *phrase,
        Err(text) => {
            errors.error(
                ErrorKind::UnknownLocalizedString,
                format!("{text:?} is not a built-in localized phrase"),
                expr.loc(),
            );
            return expr;
        }
    };
    if args.len() > SLOTS {
        errors.error(
            ErrorKind::TooManyLocalizedArgs,
            format!(
                "a localized string takes at most {SLOTS} arguments, not {}",
                args.len()
            ),
            expr.loc(),
        );
        return expr;
    }
    let mut all = vec![Expr::new(
        ExprKind::Str(StrExpr::Localized(phrase)),
        expr.loc(),
    )];
    all.extend(args.iter().cloned());
    all.resize_with(1 + SLOTS, || Expr::new(ExprKind::Null, expr.loc()));
    Expr::new(
        ExprKind::Value {
            id: ValueId::LocalizedString,
            args: all,
        },
        expr.loc(),
    )
}

#[cfg(test)]
mod tests;
