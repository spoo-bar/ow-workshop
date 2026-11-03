//! Lowers `switch` to a skip table.
//!
//! ```text
//! If(True);
//!     Skip(Value In Array(Array(d_default, d_0, d_1, …),
//!                         Add(1, Index Of Array Value(Array(c0, c1, …), v))));
//! L_0: B0   L_1: B1   L_default: D
//! End;
//! ```
//!
//! `Index Of Array Value` gives -1 for a missing value, which selects the default distance.
//! Reaching an `Else` inside a taken `If` jumps to its `End`, so `break` becomes `Else`.

use ow_workshop_data::{ActionId, ValueId};

use super::Ctx;
use super::visit::fold_block;
use crate::ir::{
    Block, Diagnostics, Expr, ExprKind, LabelId, Location, Program, SkipTarget, Stmt, StmtKind,
};

fn stmt(kind: StmtKind, loc: Option<Location>) -> Stmt {
    Stmt {
        kind,
        loc,
        comment: None,
    }
}

fn action(id: ActionId, loc: Option<Location>) -> Stmt {
    stmt(StmtKind::Action { id, args: vec![] }, loc)
}

/// Turns a `break` of the switch being lowered into a jump to its `End`.
///
/// A `break` directly in the case body becomes `Else`; one inside an `If` cannot (the `Else`
/// would belong to that `If`), so it skips to a label placed before the `End`.
fn lower_breaks(block: &mut Block, nested: bool, exit: &mut Option<LabelId>, ctx: &mut Ctx<'_>) {
    for s in &mut block.stmts {
        match &mut s.kind {
            StmtKind::Break if !nested => {
                s.kind = StmtKind::Action {
                    id: ActionId::Else,
                    args: vec![],
                }
            }
            StmtKind::Break => {
                let label = *exit.get_or_insert_with(|| ctx.fresh_label());
                s.kind = StmtKind::Skip(SkipTarget::Label(label));
            }
            // Breaks in these belong to the loop.
            StmtKind::While { .. } | StmtKind::For { .. } | StmtKind::DoWhile { .. } => {}
            _ => {
                for child in super::visit::blocks_mut(s) {
                    lower_breaks(child, true, exit, ctx);
                }
            }
        }
    }
}

fn distance(label: LabelId) -> Expr {
    Expr::new(ExprKind::LabelDistance(label), None)
}

#[allow(clippy::needless_pass_by_value)]
fn lower_switch(s: Stmt, ctx: &mut Ctx<'_>) -> Vec<Stmt> {
    let loc = s.loc;
    let StmtKind::Switch {
        value,
        cases,
        default,
        default_at,
    } = s.kind
    else {
        return vec![s];
    };

    let mut exit = None;
    let default_label = ctx.fresh_label();
    let mut table = vec![distance(default_label)];
    let mut keys = Vec::new();
    let mut bodies: Vec<(LabelId, Block)> = Vec::new();
    for (key, mut body) in cases {
        let label = ctx.fresh_label();
        table.push(distance(label));
        keys.push(key);
        lower_breaks(&mut body, false, &mut exit, ctx);
        bodies.push((label, body));
    }
    let mut default = default.unwrap_or_default();
    lower_breaks(&mut default, false, &mut exit, ctx);
    bodies.insert(default_at.min(bodies.len()), (default_label, default));

    let value_expr = |kind| Expr::new(kind, loc);
    let index_of = value_expr(ExprKind::Value {
        id: ValueId::Index,
        args: vec![value_expr(ExprKind::Array(keys)), value],
    });
    let selector = value_expr(ExprKind::Index {
        array: value_expr(ExprKind::Array(table)),
        index: value_expr(ExprKind::Add(value_expr(ExprKind::Num(1.0)), index_of)),
    });

    let mut out = vec![
        Stmt {
            comment: s.comment,
            ..stmt(StmtKind::SwitchIf, loc)
        },
        stmt(StmtKind::Skip(SkipTarget::Dynamic(selector)), loc),
    ];
    for (label, body) in bodies {
        out.push(stmt(StmtKind::Label(label), loc));
        out.extend(body.stmts);
    }
    if let Some(label) = exit {
        out.push(stmt(StmtKind::Label(label), loc));
    }
    out.push(action(ActionId::End, loc));
    out
}

/// Replaces every `Switch` by the flat `If(True)` / `Skip` table sequence.
pub(crate) fn lower(program: &mut Program, ctx: &mut Ctx<'_>) -> Result<(), Diagnostics> {
    for rule in &mut program.rules {
        fold_block(&mut rule.body, &mut |s| {
            if matches!(s.kind, StmtKind::Switch { .. }) {
                lower_switch(s, ctx)
            } else {
                vec![s]
            }
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests;
