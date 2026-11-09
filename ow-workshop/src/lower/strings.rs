//! Normalizes string templates: unnumbers placeholders, folds simple arguments into the text and
//! merges adjacent text. Runs before the optimizer and before splitting.

use crate::fmt::push_text;
use crate::ir::{Block, Diagnostics, Expr, ExprKind, Location, Program, StrExpr, StrToken};
use crate::lower::Ctx;
use crate::lower::visit::{blocks_mut, map_stmt_roots, rewrite_expr};
use crate::opt::Scope;
use crate::warning::Warning;

/// Largest magnitude of a number literal that is folded into the text.
const MAX_FOLDED: f64 = 10_000_000.0;

/// The pass.
pub(crate) fn normalize(program: &mut Program, ctx: &mut Ctx<'_>) -> Result<(), Diagnostics> {
    for rule in &mut program.rules {
        let scope = Scope::new(ctx.options).enter(&rule.body);
        let mut env = Env {
            ctx: &mut *ctx,
            optimize: scope.enabled(),
        };
        for cond in &mut rule.conditions {
            cond.expr = rewrite_expr(&cond.expr, &mut |e| normalize_node(e, &mut env));
        }
        normalize_block(&mut rule.body, scope, &mut env);
    }
    Ok(())
}

struct Env<'a, 'b> {
    ctx: &'a mut Ctx<'b>,
    optimize: bool,
}

fn normalize_block(block: &mut Block, scope: Scope, env: &mut Env<'_, '_>) {
    for stmt in &mut block.stmts {
        env.optimize = scope.enabled();
        map_stmt_roots(stmt, &mut |e| {
            rewrite_expr(e, &mut |n| normalize_node(n, env))
        });
        for child in blocks_mut(stmt) {
            normalize_block(child, scope.enter(child), env);
        }
    }
}

fn normalize_node(expr: Expr, env: &mut Env<'_, '_>) -> Expr {
    let ExprKind::Str(StrExpr::Custom { tokens, args }) = expr.kind() else {
        return expr;
    };
    let (tokens, args) = normalize_template(tokens, args, expr.loc(), env);
    Expr::new(ExprKind::Str(StrExpr::Custom { tokens, args }), expr.loc())
}

/// Gives every placeholder of a template its own argument, in order.
pub(crate) fn unnumber(tokens: &[StrToken], args: &[Expr]) -> (Vec<StrToken>, Vec<Expr>) {
    let mut out = Vec::new();
    let mut out_args = Vec::new();
    for token in tokens {
        match token {
            StrToken::Text(text) => push_text(&mut out, text),
            StrToken::Arg(i) => {
                if let Some(arg) = args.get(*i) {
                    out.push(StrToken::Arg(out_args.len()));
                    out_args.push(arg.clone());
                }
            }
        }
    }
    (out, out_args)
}

/// The normalized form of one template whose arguments are already normalized.
fn normalize_template(
    tokens: &[StrToken],
    args: &[Expr],
    loc: Option<Location>,
    env: &mut Env<'_, '_>,
) -> (Vec<StrToken>, Vec<Expr>) {
    let mut out = Vec::new();
    let mut out_args = Vec::new();
    for token in tokens {
        match token {
            StrToken::Text(text) => push_text(&mut out, text),
            StrToken::Arg(i) => match args.get(*i) {
                Some(arg) => push_arg(arg.clone(), loc, env, &mut out, &mut out_args),
                None => push_text(&mut out, ""),
            },
        }
    }
    (out, out_args)
}

/// Appends one argument: folded into the text when possible, else as its own placeholder.
fn push_arg(
    arg: Expr,
    loc: Option<Location>,
    env: &mut Env<'_, '_>,
    out: &mut Vec<StrToken>,
    out_args: &mut Vec<Expr>,
) {
    if env.optimize {
        match arg.kind() {
            ExprKind::Str(StrExpr::Custom { tokens, args }) => {
                let base = out_args.len();
                for token in tokens {
                    match token {
                        StrToken::Text(text) => push_text(out, text),
                        StrToken::Arg(i) => out.push(StrToken::Arg(base + i)),
                    }
                }
                out_args.extend(args.iter().cloned());
                return;
            }
            ExprKind::Null => return push_text(out, "0"),
            ExprKind::Array(items) => {
                env.ctx.warn(
                    Warning::ArrayInString,
                    "only the first element of an array shows in a string; use `array_to_string`"
                        .to_owned(),
                    arg.loc().or(loc),
                );
                return match items.first() {
                    Some(first) => push_arg(first.clone(), loc, env, out, out_args),
                    None => push_text(out, ""),
                };
            }
            _ => {
                if let Some(n) = number_literal(&arg).filter(|n| n.abs() <= MAX_FOLDED) {
                    return push_text(out, &to_fixed_2(n));
                }
            }
        }
    }
    out.push(StrToken::Arg(out_args.len()));
    out_args.push(arg);
}

fn number_literal(e: &Expr) -> Option<f64> {
    match e.kind() {
        ExprKind::Num(n) => Some(*n),
        ExprKind::Neg(inner) => match inner.kind() {
            ExprKind::Num(n) => Some(-n),
            _ => None,
        },
        _ => None,
    }
}

/// `n.toFixed(2)` without a trailing `.00`.
pub(crate) fn to_fixed_2(n: f64) -> String {
    let abs = n.abs();
    // Ties (odd multiples of 1/8) round up in JavaScript, to even in Rust.
    let eighths = abs * 8.0;
    #[allow(
        clippy::float_cmp,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    let fixed = if eighths == eighths.trunc() && (eighths as u64) % 2 == 1 {
        let cents = (abs * 100.0 + 0.5) as u64;
        format!("{}.{:02}", cents / 100, cents % 100)
    } else {
        format!("{abs:.2}")
    };
    let fixed = fixed.strip_suffix(".00").unwrap_or(&fixed);
    if n < 0.0 {
        format!("-{fixed}")
    } else {
        fixed.to_owned()
    }
}

#[cfg(test)]
mod tests;
