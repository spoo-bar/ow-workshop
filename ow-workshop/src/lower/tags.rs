//! Text tags in string templates: the `<` of each tag becomes `Global.__holygrail__`.

use crate::fmt::push_text;
use crate::ir::{
    Block, Diagnostics, Expr, ExprKind, Location, Program, Severity, StrExpr, StrToken, VarId,
    WarningSet,
};
use crate::lower::Ctx;
use crate::lower::strings::unnumber;
use crate::lower::visit::{blocks_mut, map_stmt_roots, rewrite_expr_skipping};
use crate::lower::workshop_settings::is_workshop_setting;
use crate::tags::{Item, TxFix, find_tags, holygrail_var, items, tx_fix};
use crate::warning::Warning;

/// The pass.
pub(crate) fn lower(program: &mut Program, ctx: &mut Ctx<'_>) -> Result<(), Diagnostics> {
    let enabled = crate::tags::enabled(program, ctx.options);
    let grail = holygrail_var(program).filter(|_| enabled);
    if enabled && grail.is_none() {
        let mut errors = Diagnostics::default();
        errors.push(
            Severity::Error,
            "missing-holygrail",
            "text tags are enabled but `__holygrail__` was not declared".to_owned(),
            None,
        );
        return Err(errors);
    }
    for rule in &mut program.rules {
        let mut env = Env {
            ctx: &mut *ctx,
            grail,
            suppressed: rule.suppressed.clone(),
        };
        let mut node = |e: Expr| lower_node(e, &mut env);
        for cond in &mut rule.conditions {
            cond.expr = rewrite_expr_skipping(&cond.expr, &is_setting, &mut node);
        }
        lower_block(&mut rule.body, &mut node);
    }
    Ok(())
}

struct Env<'a, 'b> {
    ctx: &'a mut Ctx<'b>,
    /// `None` while tags are disabled.
    grail: Option<VarId>,
    suppressed: WarningSet,
}

fn lower_block(block: &mut Block, node: &mut impl FnMut(Expr) -> Expr) {
    for stmt in &mut block.stmts {
        map_stmt_roots(stmt, &mut |e| rewrite_expr_skipping(e, &is_setting, node));
        for child in blocks_mut(stmt) {
            lower_block(child, node);
        }
    }
}

fn lower_node(expr: Expr, env: &mut Env<'_, '_>) -> Expr {
    match (expr.kind(), env.grail) {
        (ExprKind::HolyGrail, Some(var)) => Expr::new(ExprKind::GlobalVar(var), expr.loc()),
        (ExprKind::Str(StrExpr::Custom { tokens, args }), grail) => {
            let (tokens, args) = unnumber(tokens, args);
            let Some(grail) = grail else {
                if !find_tags(&items(&tokens)).is_empty() {
                    env.ctx.warn(
                        &env.suppressed,
                        Warning::Tags,
                        "this string has a text tag, but tags are disabled; set `CompileOptions::tags`"
                            .to_owned(),
                        expr.loc(),
                    );
                }
                return expr;
            };
            let (tokens, args) = substitute(&tokens, &args, grail, expr.loc());
            let tokens = autocorrect(tokens, &args, grail, expr.loc(), env);
            Expr::new(ExprKind::Str(StrExpr::Custom { tokens, args }), expr.loc())
        }
        _ => expr,
    }
}

/// Workshop settings keep their tags as plain text.
fn is_setting(expr: &Expr) -> bool {
    matches!(expr.kind(), ExprKind::Value { id, .. } if is_workshop_setting(*id))
}

/// Replaces the `<` of every tag with an argument holding `__holygrail__`.
fn substitute(
    tokens: &[StrToken],
    args: &[Expr],
    grail: VarId,
    loc: Option<Location>,
) -> (Vec<StrToken>, Vec<Expr>) {
    let all = items(tokens);
    let starts: Vec<usize> = find_tags(&all).iter().map(|r| r.start).collect();
    let mut out = Vec::new();
    let mut out_args: Vec<Expr> = Vec::new();
    for (i, item) in all.iter().enumerate() {
        match item {
            Item::Ch(_) if starts.contains(&i) => {
                out.push(StrToken::Arg(out_args.len()));
                out_args.push(Expr::new(ExprKind::GlobalVar(grail), loc));
            }
            Item::Ch(c) => push_text(&mut out, &c.to_string()),
            Item::Arg(a) => {
                out.push(StrToken::Arg(out_args.len()));
                out_args.push(args[*a].clone());
            }
        }
    }
    (out, out_args)
}

/// Rewrites short `tx<id>>` ids right after a holygrail argument to `txC` + 14 digits.
fn autocorrect(
    mut tokens: Vec<StrToken>,
    args: &[Expr],
    grail: VarId,
    loc: Option<Location>,
    env: &mut Env<'_, '_>,
) -> Vec<StrToken> {
    for at in 1..tokens.len() {
        let (before, after) = tokens.split_at_mut(at);
        let (StrToken::Arg(a), StrToken::Text(text)) = (&before[at - 1], &mut after[0]) else {
            continue;
        };
        if !matches!(args[*a].kind(), ExprKind::GlobalVar(v) if *v == grail) {
            continue;
        }
        match tx_fix(text) {
            TxFix::Fixed(len, fixed) => text.replace_range(..len, &fixed),
            TxFix::Malformed => env.ctx.warn(
                &env.suppressed,
                Warning::MalformattedTx,
                "a texture id must be `C` followed by exactly 14 hex digits".to_owned(),
                loc,
            ),
            TxFix::None => {}
        }
    }
    tokens
}

#[cfg(test)]
mod tests;
