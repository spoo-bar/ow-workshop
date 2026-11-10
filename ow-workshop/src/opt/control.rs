//! Statement and control-flow cleanup: assignments, dead code and no-ops.
//!
//! None of this runs in a rule with a dynamic skip: removing a statement would move the
//! distance such a skip measures.

use std::collections::HashSet;

use ow_workshop_data::{ActionId, ValueId as V};

use super::Env;
use super::build::call_args;
use super::helpers::{always_equal, definitely_falsy, definitely_truthy};
use crate::ir::{
    Block, Expr, ExprKind, LabelId, ModifyOp, Place, Rule, Stmt, StmtKind, StrExpr, StrToken, SubId,
};
use crate::lower::labels::skip_labels;
use crate::lower::visit::walk_block;

/// The labels the rule's skips jump to, and whether it has any skip.
pub(super) fn skips(rule: &Rule) -> (HashSet<LabelId>, bool) {
    let mut targeted = HashSet::new();
    let mut any = false;
    walk_block(&rule.body, &mut |stmt: &Stmt| {
        if let Some(labels) = skip_labels(&stmt.kind) {
            any = true;
            targeted.extend(labels);
        }
    });
    (targeted, any)
}

/// Subroutines something calls or starts.
pub(super) fn called_subroutines(rules: &[Rule]) -> HashSet<SubId> {
    let mut called = HashSet::new();
    for rule in rules {
        walk_block(&rule.body, &mut |stmt: &Stmt| {
            if let StmtKind::CallSubroutine(sub) | StmtKind::StartRule(sub, _) = &stmt.kind {
                called.insert(*sub);
            }
        });
    }
    called
}

/// Whether a rule is worth keeping: it has an action that is not control flow or a pause.
///
/// A subroutine in `called` is kept either way.
pub(super) fn does_something(rule: &Rule, called: &HashSet<SubId>) -> bool {
    if let crate::ir::Event::Subroutine(sub) = rule.event {
        if called.contains(&sub) {
            return true;
        }
    }
    let in_subroutine = matches!(rule.event, crate::ir::Event::Subroutine(_));
    let mut meaningful = false;
    walk_block(&rule.body, &mut |stmt: &Stmt| {
        meaningful |= match &stmt.kind {
            StmtKind::If { .. }
            | StmtKind::While { .. }
            | StmtKind::Abort
            | StmtKind::AbortIf(_)
            | StmtKind::AbortIfConditionTrue
            | StmtKind::AbortIfConditionFalse
            | StmtKind::Loop
            | StmtKind::LoopIf(_)
            | StmtKind::LoopIfConditionTrue
            | StmtKind::LoopIfConditionFalse
            | StmtKind::Skip(_)
            | StmtKind::SkipIf(..)
            | StmtKind::Pass
            | StmtKind::Comment(_)
            | StmtKind::Label(_) => false,
            // Pausing matters to whoever called the subroutine.
            StmtKind::Action {
                id: ActionId::Wait, ..
            } => in_subroutine,
            _ => true,
        };
    });
    meaningful
}

// ---- assignments ------------------------------------------------------------------------

/// The expression that reads a place.
fn read(place: &Place, like: &Stmt) -> Expr {
    let loc = like.loc;
    match place {
        Place::Global(var) => Expr::new(ExprKind::GlobalVar(*var), loc),
        Place::Player { player, var } => Expr::new(
            ExprKind::PlayerVar {
                player: player.clone(),
                var: *var,
            },
            loc,
        ),
        Place::Index { base, index } => Expr::new(
            ExprKind::Index {
                array: read(base, like),
                index: index.clone(),
            },
            loc,
        ),
    }
}

/// `X = X op Y` as the operation and `Y`, when the left operand is `current`.
fn self_update(value: &Expr, current: &Expr) -> Option<(ModifyOp, Expr)> {
    let (op, lhs, rhs) = match value.kind() {
        ExprKind::Add(a, b) => (ModifyOp::Add, a, b),
        ExprKind::Sub(a, b) => (ModifyOp::Subtract, a, b),
        ExprKind::Mul(a, b) => (ModifyOp::Multiply, a, b),
        ExprKind::Div(a, b) => (ModifyOp::Divide, a, b),
        ExprKind::Mod(a, b) => (ModifyOp::Modulo, a, b),
        ExprKind::Pow(a, b) => (ModifyOp::RaiseToPower, a, b),
        ExprKind::Value { id, args } => match (*id, args.as_slice()) {
            (V::Min, [a, b]) => (ModifyOp::Min, a, b),
            (V::Max, [a, b]) => (ModifyOp::Max, a, b),
            (V::Concat, [a, b]) => (ModifyOp::AppendToArray, a, b),
            (V::Exclude, [a, b]) => (ModifyOp::RemoveFromArrayByValue, a, b),
            _ => return None,
        },
        _ => return None,
    };
    always_equal(lhs, current).then(|| (op, rhs.clone()))
}

fn assignment(mut stmt: Stmt, env: &Env<'_, '_>) -> Vec<Stmt> {
    let StmtKind::Set { target, value } = &stmt.kind else {
        return vec![stmt];
    };
    let current = read(target, &stmt);
    if always_equal(value, &current) {
        return if env.dynamic_skips {
            vec![stmt]
        } else {
            Vec::new()
        };
    }
    if let Some((op, operand)) = self_update(value, &current) {
        stmt.kind = StmtKind::Modify {
            target: target.clone(),
            op,
            value: operand,
        };
    }
    vec![stmt]
}

// ---- dead code --------------------------------------------------------------------------

fn has_target(block: &Block, targeted: &HashSet<LabelId>) -> bool {
    let mut found = false;
    walk_block(block, &mut |stmt: &Stmt| {
        found |= matches!(&stmt.kind, StmtKind::Label(l) if targeted.contains(l));
    });
    found
}

/// A block whose statements can be spliced into the parent: it has no settings of its own.
fn splice(block: Block) -> Option<Vec<Stmt>> {
    block.opt.is_none().then_some(block.stmts)
}

fn dead_if(stmt: Stmt, env: &Env<'_, '_>) -> Vec<Stmt> {
    let StmtKind::If {
        cond,
        then,
        elifs,
        else_,
    } = &stmt.kind
    else {
        return vec![stmt];
    };
    let loc = stmt.loc;
    let keep = |stmt: Stmt| vec![stmt];
    let skipless = !env.has_skips;
    if definitely_falsy(cond) && !has_target(then, &env.targeted) {
        if let Some((first, rest)) = elifs.split_first() {
            // Each step removes a branch, so this ends; it folds chains of dead branches.
            return dead_if(
                Stmt {
                    kind: StmtKind::If {
                        cond: first.0.clone(),
                        then: first.1.clone(),
                        elifs: rest.to_vec(),
                        else_: else_.clone(),
                    },
                    loc,
                    ..stmt
                },
                env,
            );
        }
        return match else_ {
            Some(body) => splice(body.clone()).unwrap_or_else(|| keep(stmt)),
            None => Vec::new(),
        };
    }
    if skipless && definitely_truthy(cond) {
        return splice(then.clone()).unwrap_or_else(|| keep(stmt));
    }
    if let Some(at) = elifs.iter().position(|(c, b)| {
        (definitely_falsy(c) && !has_target(b, &env.targeted)) || (skipless && definitely_truthy(c))
    }) {
        let mut next = stmt.clone();
        let StmtKind::If {
            elifs: new_elifs,
            else_: new_else,
            ..
        } = &mut next.kind
        else {
            return keep(stmt);
        };
        if definitely_falsy(&elifs[at].0) {
            new_elifs.remove(at);
        } else {
            // A branch that always holds ends the chain; it becomes the `Else`.
            *new_else = Some(elifs[at].1.clone());
            new_elifs.truncate(at);
        }
        return dead_if(next, env);
    }
    if then.stmts.is_empty() && elifs.is_empty() && else_.is_none() {
        return Vec::new();
    }
    keep(stmt)
}

fn dead_while(stmt: Stmt, env: &Env<'_, '_>) -> Vec<Stmt> {
    match &stmt.kind {
        StmtKind::While { cond, body }
            if definitely_falsy(cond) && !has_target(body, &env.targeted) =>
        {
            Vec::new()
        }
        _ => vec![stmt],
    }
}

/// Whether control never continues past the statement.
fn ends_flow(stmt: &Stmt) -> bool {
    matches!(
        stmt.kind,
        StmtKind::Abort
            | StmtKind::Loop
            | StmtKind::Action {
                id: ActionId::Return,
                ..
            }
    )
}

/// Removes what follows an unconditional exit, up to the first statement a skip can land in.
pub(super) fn trim_unreachable(block: &mut Block, targeted: &HashSet<LabelId>) {
    let landable = |stmt: &Stmt| match &stmt.kind {
        StmtKind::Label(l) => targeted.contains(l),
        _ => stmt.blocks().into_iter().any(|b| has_target(b, targeted)),
    };
    let mut at = 0;
    while at < block.stmts.len() {
        if ends_flow(&block.stmts[at]) {
            let dead = block.stmts[at + 1..]
                .iter()
                .position(landable)
                .unwrap_or(block.stmts.len() - at - 1);
            block.stmts.drain(at + 1..at + 1 + dead);
        }
        at += 1;
    }
}

// ---- workshop no-ops --------------------------------------------------------------------

/// An empty string: a custom string without text or the empty phrase.
fn is_empty_text(e: &Expr) -> bool {
    match e.kind() {
        ExprKind::Str(StrExpr::Custom { tokens, .. }) => tokens
            .iter()
            .all(|t| matches!(t, StrToken::Text(text) if text.is_empty())),
        ExprKind::Str(StrExpr::Localized(s)) => s.text(ow_workshop_data::Lang::EnUs).is_empty(),
        _ => false,
    }
}

fn is_blank(e: &Expr) -> bool {
    matches!(e.kind(), ExprKind::Null) || call_args(e, V::Null).is_some() || is_empty_text(e)
}

fn hud_text(mut stmt: Stmt, env: &Env<'_, '_>) -> Vec<Stmt> {
    let StmtKind::Action {
        id: ActionId::HudText,
        args,
    } = &mut stmt.kind
    else {
        return vec![stmt];
    };
    for arg in args.iter_mut().skip(1).take(3) {
        if is_empty_text(arg) {
            *arg = Expr::new(ExprKind::Null, arg.loc());
        }
    }
    if !env.dynamic_skips && args.iter().skip(1).take(3).all(is_blank) {
        return Vec::new();
    }
    vec![stmt]
}

/// The statement rules of this module.
pub(super) fn rule(stmt: Stmt, env: &mut Env<'_, '_>) -> Vec<Stmt> {
    if matches!(stmt.kind, StmtKind::Pass) {
        return if env.dynamic_skips {
            vec![stmt]
        } else {
            Vec::new()
        };
    }
    // Assignments only ever turn into a cheaper statement or vanish.
    match stmt.kind {
        StmtKind::Set { .. } => assignment(stmt, env),
        StmtKind::If { .. } if !env.dynamic_skips => dead_if(stmt, env),
        StmtKind::While { .. } if !env.dynamic_skips => dead_while(stmt, env),
        StmtKind::Action {
            id: ActionId::HudText,
            ..
        } => hud_text(stmt, env),
        _ => vec![stmt],
    }
}

#[cfg(test)]
mod tests;
