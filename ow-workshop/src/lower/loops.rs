//! Lowers `break`, `continue` and `do_while`.
//!
//! The workshop `Continue` action is never used: it aborts the rule when an `If` precedes it.
//! A `continue` in a `While`/`For` skips to a label at the end of the body instead.

use super::visit::blocks_mut;
use super::{Ctx, const_truthiness};
use crate::ir::{
    Block, Diagnostics, Expr, ExprKind, LabelId, Program, Rule, Severity, SkipTarget, Stmt,
    StmtKind, WarningSet,
};
use crate::warning::Warning;

/// The innermost construct a `break` or `continue` refers to.
enum Enclosing {
    /// A `While` or `For`: `break` is the workshop action.
    Loop { continue_label: Option<LabelId> },
    /// A `do_while`: both jump with labels and the tail loop.
    DoWhile { cond: Expr, exit: Option<LabelId> },
}

struct Env<'a, 'b> {
    ctx: &'a mut Ctx<'b>,
    suppressed: &'a WarningSet,
    rule: &'a str,
    errors: Diagnostics,
}

fn mk(kind: StmtKind, like: &Stmt) -> Stmt {
    Stmt {
        kind,
        loc: like.loc,
        comment: None,
    }
}

/// The statement at the end of a `do_while` body; `None` when it never repeats.
fn loop_back(cond: &Expr) -> Option<StmtKind> {
    match const_truthiness(cond) {
        Some(true) => return Some(StmtKind::Loop),
        Some(false) => return None,
        None => {}
    }
    Some(match cond.kind() {
        ExprKind::RuleCondition => StmtKind::LoopIfConditionTrue,
        ExprKind::Not(inner) if matches!(inner.kind(), ExprKind::RuleCondition) => {
            StmtKind::LoopIfConditionFalse
        }
        _ => StmtKind::LoopIf(cond.clone()),
    })
}

fn skip_to(label: LabelId, like: &Stmt) -> Stmt {
    mk(StmtKind::Skip(SkipTarget::Label(label)), like)
}

fn outside(env: &mut Env<'_, '_>, stmt: &Stmt, warning: Warning, what: &str) -> Stmt {
    env.ctx.warn(
        env.suppressed,
        warning,
        format!(
            "rule `{}`: `{what}` outside a loop; the rule is aborted instead",
            env.rule
        ),
        stmt.loc,
    );
    mk(StmtKind::Abort, stmt)
}

fn lower_stmt(stmt: Stmt, stack: &mut Vec<Enclosing>, env: &mut Env<'_, '_>) -> Vec<Stmt> {
    match &stmt.kind {
        StmtKind::Break => match stack.last_mut() {
            None => vec![outside(env, &stmt, Warning::BreakOutsideLoop, "break")],
            Some(Enclosing::Loop { .. }) => vec![stmt],
            Some(Enclosing::DoWhile { exit, .. }) => {
                let label = *exit.get_or_insert_with(|| env.ctx.fresh_label());
                vec![skip_to(label, &stmt)]
            }
        },
        StmtKind::Continue => match stack.last_mut() {
            None => vec![outside(
                env,
                &stmt,
                Warning::ContinueOutsideLoop,
                "continue",
            )],
            Some(Enclosing::Loop { continue_label }) => {
                let label = *continue_label.get_or_insert_with(|| env.ctx.fresh_label());
                vec![skip_to(label, &stmt)]
            }
            Some(Enclosing::DoWhile { cond, exit }) => {
                // `Loop If` falls through when the condition fails, so leave the loop after it.
                let back = loop_back(cond);
                let mut out: Vec<Stmt> = back.iter().map(|k| mk(k.clone(), &stmt)).collect();
                if !matches!(back, Some(StmtKind::Loop)) {
                    let label = *exit.get_or_insert_with(|| env.ctx.fresh_label());
                    out.push(skip_to(label, &stmt));
                }
                out
            }
        },
        StmtKind::DoWhile { .. } => {
            env.errors.push(
                Severity::Error,
                "misplaced-do-while",
                format!(
                    "rule `{}`: `do_while` must be the first statement of a rule or subroutine body",
                    env.rule
                ),
                stmt.loc,
            );
            Vec::new()
        }
        StmtKind::While { .. } | StmtKind::For { .. } => {
            let mut stmt = stmt;
            stack.push(Enclosing::Loop {
                continue_label: None,
            });
            for child in blocks_mut(&mut stmt) {
                lower_block(child, stack, env);
            }
            if let Some(Enclosing::Loop {
                continue_label: Some(label),
            }) = stack.pop()
            {
                let placed = mk(StmtKind::Label(label), &stmt);
                if let Some(body) = blocks_mut(&mut stmt).into_iter().next() {
                    body.stmts.push(placed);
                }
            }
            vec![stmt]
        }
        _ => {
            let mut stmt = stmt;
            for child in blocks_mut(&mut stmt) {
                lower_block(child, stack, env);
            }
            vec![stmt]
        }
    }
}

fn lower_block(block: &mut Block, stack: &mut Vec<Enclosing>, env: &mut Env<'_, '_>) {
    let stmts = std::mem::take(&mut block.stmts);
    for stmt in stmts {
        block.stmts.extend(lower_stmt(stmt, stack, env));
    }
}

/// Lowers the `do_while` at the top of a rule body, then everything else.
fn lower_rule(rule: &mut Rule, ctx: &mut Ctx<'_>, errors: &mut Diagnostics) {
    let name = rule.name.clone();
    let suppressed = rule.suppressed.clone();
    let mut env = Env {
        ctx,
        suppressed: &suppressed,
        rule: &name,
        errors: Diagnostics::default(),
    };

    let mut leading = true;
    let stmts = std::mem::take(&mut rule.body.stmts);
    let mut out = Vec::with_capacity(stmts.len());
    for stmt in stmts {
        let StmtKind::DoWhile { .. } = &stmt.kind else {
            leading &= stmt.is_noop();
            let mut stack = Vec::new();
            out.extend(lower_stmt(stmt, &mut stack, &mut env));
            continue;
        };
        if !leading {
            env.errors.push(
                Severity::Error,
                "misplaced-do-while",
                format!(
                    "rule `{name}`: `do_while` must be the first statement of a rule or subroutine body"
                ),
                stmt.loc,
            );
            continue;
        }
        leading = false;
        let StmtKind::DoWhile { cond, body } = stmt.kind.clone() else {
            continue;
        };
        let mut body = body;
        let mut stack = vec![Enclosing::DoWhile {
            cond: cond.clone(),
            exit: None,
        }];
        lower_block(&mut body, &mut stack, &mut env);
        out.extend(body.stmts);
        out.extend(loop_back(&cond).map(|k| mk(k, &stmt)));
        if let Some(Enclosing::DoWhile {
            exit: Some(label), ..
        }) = stack.pop()
        {
            out.push(mk(StmtKind::Label(label), &stmt));
        }
    }
    rule.body.stmts = out;
    errors.0.extend(env.errors.0);
}

/// Replaces `break`, `continue` and `do_while` by plain workshop control flow.
pub(crate) fn lower(program: &mut Program, ctx: &mut Ctx<'_>) -> Result<(), Diagnostics> {
    let mut errors = Diagnostics::default();
    for rule in &mut program.rules {
        lower_rule(rule, ctx, &mut errors);
    }
    if errors.has_errors() {
        return Err(errors);
    }
    Ok(())
}

#[cfg(test)]
mod tests;
