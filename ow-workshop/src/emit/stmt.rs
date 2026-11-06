//! Statement printing.

use core::fmt::{self, Write};

use ow_workshop_data::{ActionId, Lang, ModifyOp as DataModifyOp, ValueId};

use super::Cx;
use super::expr::{
    write_expr, write_global, write_name_and_args, write_player_var, write_var_name,
};
use crate::ir::{ModifyOp, Place, SkipTarget, Stmt, StmtKind, SubId, VarId};

/// The actions of one variable operation: plain and indexed, for globals and players.
struct Family {
    global: ActionId,
    player: ActionId,
    global_at_index: Option<ActionId>,
    player_at_index: Option<ActionId>,
}

const SET: Family = Family {
    global: ActionId::SetGlobalVariable,
    player: ActionId::SetPlayerVariable,
    global_at_index: Some(ActionId::SetGlobalVariableAtIndex),
    player_at_index: Some(ActionId::SetPlayerVariableAtIndex),
};
const MODIFY: Family = Family {
    global: ActionId::ModifyGlobalVariable,
    player: ActionId::ModifyPlayerVariable,
    global_at_index: Some(ActionId::ModifyGlobalVariableAtIndex),
    player_at_index: Some(ActionId::ModifyPlayerVariableAtIndex),
};
const CHASE_RATE: Family = Family {
    global: ActionId::ChaseGlobalVariableAtRate,
    player: ActionId::ChasePlayerVariableAtRate,
    global_at_index: None,
    player_at_index: None,
};
const CHASE_TIME: Family = Family {
    global: ActionId::ChaseGlobalVariableOverTime,
    player: ActionId::ChasePlayerVariableOverTime,
    global_at_index: None,
    player_at_index: None,
};
const STOP_CHASING: Family = Family {
    global: ActionId::StopChasingGlobalVariable,
    player: ActionId::StopChasingPlayerVariable,
    global_at_index: None,
    player_at_index: None,
};
const FOR: Family = Family {
    global: ActionId::ForGlobalVariable,
    player: ActionId::ForPlayerVariable,
    global_at_index: None,
    player_at_index: None,
};

/// What follows the variable (and index) in a variable action's argument list.
#[derive(Clone, Copy)]
enum Tail<'a> {
    /// A raw operation name, then the expressions.
    Op(&'static str, &'a [&'a crate::ir::Expr]),
    /// Just the expressions.
    Exprs(&'a [&'a crate::ir::Expr]),
}

/// Writes a statement without its `;`.
pub(super) fn write_stmt(out: &mut impl Write, cx: Cx<'_>, stmt: &Stmt) -> fmt::Result {
    let name = |id: ActionId| id.name(cx.lang);
    match &stmt.kind {
        StmtKind::Action { id, args } => {
            let args: Vec<_> = args.iter().collect();
            write_name_and_args(out, cx, name(*id), &args)
        }
        StmtKind::Set { target, value } => {
            write_variable_action(out, cx, &SET, target, Tail::Exprs(&[value]))
        }
        StmtKind::Modify { target, op, value } => write_modify(out, cx, target, *op, value),
        StmtKind::Chase {
            target,
            dest,
            rate_or_duration,
            reeval,
            over_time,
        } => {
            let family = if *over_time { &CHASE_TIME } else { &CHASE_RATE };
            let tail = Tail::Exprs(&[dest, rate_or_duration, reeval]);
            write_variable_action(out, cx, family, target, tail)
        }
        StmtKind::StopChasing(target) => {
            write_variable_action(out, cx, &STOP_CHASING, target, Tail::Exprs(&[]))
        }
        StmtKind::Skip(target) => {
            out.write_str(name(ActionId::Skip))?;
            out.write_char('(')?;
            write_distance(out, cx, target)?;
            out.write_char(')')
        }
        StmtKind::SkipIf(cond, target) => {
            out.write_str(name(ActionId::SkipIf))?;
            out.write_char('(')?;
            write_expr(out, cx, cond)?;
            out.write_str(", ")?;
            write_distance(out, cx, target)?;
            out.write_char(')')
        }
        StmtKind::Abort => out.write_str(name(ActionId::Return)),
        StmtKind::AbortIf(cond) => write_with(out, cx, ActionId::AbortIf, &[cond]),
        StmtKind::AbortIfConditionTrue => out.write_str(name(ActionId::AbortIfConditionIsTrue)),
        StmtKind::AbortIfConditionFalse => out.write_str(name(ActionId::AbortIfConditionIsFalse)),
        StmtKind::Loop => out.write_str(name(ActionId::Loop)),
        StmtKind::LoopIf(cond) => write_with(out, cx, ActionId::LoopIf, &[cond]),
        StmtKind::LoopIfConditionTrue => out.write_str(name(ActionId::LoopIfConditionIsTrue)),
        StmtKind::LoopIfConditionFalse => out.write_str(name(ActionId::LoopIfConditionIsFalse)),
        StmtKind::CallSubroutine(sub) => {
            out.write_str(name(ActionId::CallSubroutine))?;
            out.write_char('(')?;
            write_sub_name(out, cx, *sub)?;
            out.write_char(')')
        }
        StmtKind::StartRule(sub, behavior) => {
            out.write_str(name(ActionId::StartRule))?;
            out.write_char('(')?;
            write_sub_name(out, cx, *sub)?;
            write!(out, ", {})", behavior.name(cx.lang))
        }
        StmtKind::SwitchIf => {
            write!(
                out,
                "{}({})",
                name(ActionId::If),
                super::expr::truth(cx, true)
            )
        }
        // Printed by the flatten pass's own lines, or removed by lowering.
        StmtKind::If { .. }
        | StmtKind::While { .. }
        | StmtKind::For { .. }
        | StmtKind::Label(_)
        | StmtKind::Pass
        | StmtKind::Comment(_)
        | StmtKind::Switch { .. }
        | StmtKind::Break
        | StmtKind::Continue
        | StmtKind::DoWhile { .. } => Err(fmt::Error),
    }
}

/// Writes the header of a `For` loop.
pub(super) fn write_for(
    out: &mut impl Write,
    cx: Cx<'_>,
    var: &Place,
    range: [&crate::ir::Expr; 3],
) -> fmt::Result {
    write_variable_action(out, cx, &FOR, var, Tail::Exprs(&range))
}

fn write_with(
    out: &mut impl Write,
    cx: Cx<'_>,
    id: ActionId,
    args: &[&crate::ir::Expr],
) -> fmt::Result {
    write_name_and_args(out, cx, id.name(cx.lang), args)
}

fn write_distance(out: &mut impl Write, cx: Cx<'_>, target: &SkipTarget) -> fmt::Result {
    match target {
        SkipTarget::Dynamic(distance) => write_expr(out, cx, distance),
        // Resolved to a distance before printing.
        SkipTarget::Label(_) => Err(fmt::Error),
    }
}

fn write_sub_name(out: &mut impl Write, cx: Cx<'_>, sub: SubId) -> fmt::Result {
    let decl = cx
        .program
        .subroutines
        .entries
        .get(sub.0 as usize)
        .ok_or(fmt::Error)?;
    out.write_str(&decl.name)
}

/// A place split into its player, variable and index.
struct Parts<'a> {
    player: Option<&'a crate::ir::Expr>,
    var: VarId,
    global: bool,
    index: Option<&'a crate::ir::Expr>,
}

fn parts(place: &Place) -> Result<Parts<'_>, fmt::Error> {
    let (base, index) = match place {
        Place::Index { base, index } => (&**base, Some(index)),
        base => (base, None),
    };
    match base {
        Place::Global(var) => Ok(Parts {
            player: None,
            var: *var,
            global: true,
            index,
        }),
        Place::Player { player, var } => Ok(Parts {
            player: Some(player),
            var: *var,
            global: false,
            index,
        }),
        // The workshop has one index level; deeper places are lowered to arrays.
        Place::Index { .. } => Err(fmt::Error),
    }
}

/// `Set Global Variable(X, v)`, `Set Player Variable At Index(P, X, i, v)`, ...
fn write_variable_action(
    out: &mut impl Write,
    cx: Cx<'_>,
    family: &Family,
    place: &Place,
    tail: Tail<'_>,
) -> fmt::Result {
    let Parts {
        player,
        var,
        global,
        index,
    } = parts(place)?;
    let id = match (global, index.is_some()) {
        (true, false) => Some(family.global),
        (false, false) => Some(family.player),
        (true, true) => family.global_at_index,
        (false, true) => family.player_at_index,
    }
    .ok_or(fmt::Error)?;
    out.write_str(id.name(cx.lang))?;
    out.write_char('(')?;
    if let Some(player) = player {
        write_expr(out, cx, player)?;
        out.write_str(", ")?;
    }
    write_var_name(out, cx, global, var)?;
    if let Some(index) = index {
        out.write_str(", ")?;
        write_expr(out, cx, index)?;
    }
    let exprs = match tail {
        Tail::Op(op, exprs) => {
            write!(out, ", {op}")?;
            exprs
        }
        Tail::Exprs(exprs) => exprs,
    };
    for expr in exprs {
        out.write_str(", ")?;
        write_expr(out, cx, expr)?;
    }
    out.write_char(')')
}

fn data_op(op: ModifyOp) -> DataModifyOp {
    match op {
        ModifyOp::Add => DataModifyOp::Add,
        ModifyOp::Subtract => DataModifyOp::Subtract,
        ModifyOp::Multiply => DataModifyOp::Multiply,
        ModifyOp::Divide => DataModifyOp::Divide,
        ModifyOp::Modulo => DataModifyOp::Modulo,
        ModifyOp::RaiseToPower => DataModifyOp::RaiseToPower,
        ModifyOp::Min => DataModifyOp::Min,
        ModifyOp::Max => DataModifyOp::Max,
        ModifyOp::AppendToArray => DataModifyOp::AppendToArray,
        ModifyOp::RemoveFromArrayByValue => DataModifyOp::RemoveFromArrayByValue,
        ModifyOp::RemoveFromArrayByIndex => DataModifyOp::RemoveFromArrayByIndex,
    }
}

fn write_modify(
    out: &mut impl Write,
    cx: Cx<'_>,
    target: &Place,
    op: ModifyOp,
    value: &crate::ir::Expr,
) -> fmt::Result {
    // In Japanese "Add" and "Append To Array" share a name, so `Modify(…, Add, v)` is ambiguous.
    if cx.lang == Lang::JaJp && op == ModifyOp::Add {
        write_place_read(out, cx, target)?;
        out.write_str(" += ")?;
        return write_expr(out, cx, value);
    }
    let tail = Tail::Op(data_op(op).name(cx.lang), &[value]);
    write_variable_action(out, cx, &MODIFY, target, tail)
}

/// The place as a value: `Global.X`, `(P).X`, `Global.X[i]`.
fn write_place_read(out: &mut impl Write, cx: Cx<'_>, place: &Place) -> fmt::Result {
    match place {
        Place::Global(var) => write_global(out, cx, *var),
        Place::Player { player, var } => write_player_var(out, cx, player, *var),
        Place::Index { base, index } => {
            write_place_read(out, cx, base)?;
            out.write_char('[')?;
            write_expr(out, cx, index)?;
            out.write_char(']')
        }
    }
}

#[cfg(test)]
mod tests;
