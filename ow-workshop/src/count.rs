//! Element counting.
//!
//! Overwatch limits a mod to [`ELEMENT_LIMIT`] elements. The count runs on the flat form, as its
//! own pass, and follows the game's rules rather than the shape of the printed text.

// Consumed by the emitter and the pipeline.
#![allow(dead_code)]

use ow_workshop_data::{Lang, ValueId};

use crate::flatten::{FlatKind, FlatStmt, flatten_rule};
use crate::ir::{
    Expr, ExprKind, ModifyOp, Place, Program, Rule, SkipTarget, Stmt, StmtKind, StrExpr,
};
use crate::options::CompileOptions;

/// The most elements a mod may use.
pub(crate) const ELEMENT_LIMIT: usize = 32768;

/// Argument slots of a custom or localized string.
const STRING_SLOTS: usize = 3;

/// The counts of one rule.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct RuleCount {
    /// The rule, its conditions and its actions.
    pub(crate) total: usize,
    /// One count per condition.
    pub(crate) conditions: Vec<usize>,
    /// One count per flattened line, in the order of [`flatten_rule`].
    pub(crate) lines: Vec<usize>,
}

/// The counts of a whole program.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Counts {
    /// Sum over all rules.
    pub(crate) total: usize,
    /// One entry per rule, in output order.
    pub(crate) rules: Vec<RuleCount>,
}

impl Counts {
    /// Whether the mod is over the game's limit.
    pub(crate) fn over_limit(&self) -> bool {
        self.total > ELEMENT_LIMIT
    }
}

/// Counts every rule of `program`.
pub(crate) fn count(program: &Program, lang: Lang) -> Counts {
    let rules: Vec<RuleCount> = program
        .rules
        .iter()
        .map(|rule| count_rule(rule, &program.options, lang))
        .collect();
    Counts {
        total: rules.iter().map(|r| r.total).sum(),
        rules,
    }
}

/// Counts one rule.
pub(crate) fn count_rule(rule: &Rule, options: &CompileOptions, lang: Lang) -> RuleCount {
    let conditions: Vec<usize> = rule
        .conditions
        .iter()
        .map(|cond| condition(&cond.expr))
        .collect();
    let lines: Vec<usize> = flatten_rule(rule, options)
        .iter()
        .map(|line| line_count(line, lang))
        .collect();
    RuleCount {
        total: 1 + conditions.iter().sum::<usize>() + lines.iter().sum::<usize>(),
        conditions,
        lines,
    }
}

/// A condition: `a op b` costs `a + b - 1`; anything else costs its own count.
fn condition(expr: &Expr) -> usize {
    match expr.kind() {
        ExprKind::Compare { lhs, rhs, .. } => (side(lhs) + side(rhs)).saturating_sub(1),
        // Printed as `x == False`.
        ExprKind::Not(inner) => side(inner),
        _ => side(expr),
    }
}

/// An expression as a top-level slot: its count plus the extra for pairs of hero literals.
fn side(expr: &Expr) -> usize {
    value(expr) + heroes(expr) / 2
}

fn heroes(expr: &Expr) -> usize {
    let own = matches!(expr.kind(), ExprKind::Const(c) if c.enum_name == "Hero");
    usize::from(own) + children(expr).map(heroes).sum::<usize>()
}

/// The expressions directly inside `expr`, including player and index expressions.
fn children(expr: &Expr) -> impl Iterator<Item = &Expr> {
    expr.kind().children().into_iter()
}

fn line_count(line: &FlatStmt<'_>, lang: Lang) -> usize {
    match &line.kind {
        FlatKind::Label(_) | FlatKind::Comment(_) => 0,
        FlatKind::Else | FlatKind::End | FlatKind::DisabledAbort => 1,
        FlatKind::If(cond) | FlatKind::ElseIf(cond) | FlatKind::While(cond) => {
            action(&[side(cond)])
        }
        FlatKind::For {
            var,
            start,
            stop,
            step,
        } => {
            let mut slots = place_slots(var);
            slots.extend([side(start), side(stop), side(step)]);
            action(&slots)
        }
        FlatKind::Stmt(stmt) => statement(stmt, lang),
    }
}

/// An action: itself, plus its arguments, minus the free top-level slots.
fn action(slots: &[usize]) -> usize {
    (1 + slots.iter().sum::<usize>()).saturating_sub(slots.len())
}

/// The slots of a variable: the player (if any), the name, the index (if any).
fn place_slots(place: &Place) -> Vec<usize> {
    match place {
        Place::Global(_) => vec![1],
        Place::Player { player, .. } => vec![side(player), 1],
        Place::Index { base, index } => {
            let mut slots = place_slots(base);
            slots.push(side(index));
            slots
        }
    }
}

fn statement(stmt: &Stmt, lang: Lang) -> usize {
    let one = |e: &Expr| side(e);
    let distance = |t: &SkipTarget| match t {
        SkipTarget::Dynamic(e) => side(e),
        SkipTarget::Label(_) => 1,
    };
    match &stmt.kind {
        StmtKind::Action { args, .. } => {
            let slots: Vec<usize> = args.iter().map(one).collect();
            action(&slots)
        }
        StmtKind::Set { target, value } => {
            let mut slots = place_slots(target);
            slots.push(one(value));
            action(&slots)
        }
        StmtKind::Modify { target, op, value } => {
            let mut slots = place_slots(target);
            slots.extend([1, one(value)]);
            let general = action(&slots);
            // Printed as `place += value` in Japanese, which is two elements cheaper.
            if lang == Lang::JaJp && *op == ModifyOp::Add {
                general.saturating_sub(2)
            } else {
                general
            }
        }
        StmtKind::Chase {
            target,
            dest,
            rate_or_duration,
            reeval,
            ..
        } => {
            let mut slots = place_slots(target);
            slots.extend([one(dest), one(rate_or_duration), one(reeval)]);
            action(&slots)
        }
        StmtKind::StopChasing(target) => action(&place_slots(target)),
        StmtKind::Skip(target) => action(&[distance(target)]),
        StmtKind::SkipIf(cond, target) => action(&[one(cond), distance(target)]),
        StmtKind::AbortIf(cond) | StmtKind::LoopIf(cond) => action(&[one(cond)]),
        // Every argument is a bare name or constant, so the action costs one.
        StmtKind::CallSubroutine(_)
        | StmtKind::StartRule(..)
        | StmtKind::SwitchIf
        | StmtKind::Abort
        | StmtKind::Break
        | StmtKind::AbortIfConditionTrue
        | StmtKind::AbortIfConditionFalse
        | StmtKind::Loop
        | StmtKind::LoopIfConditionTrue
        | StmtKind::LoopIfConditionFalse => 1,
        // Printed as zero-width markers, or gone before counting.
        _ => 0,
    }
}

/// Elements the game's own settings values save, relative to their arguments.
fn setting_discount(id: ValueId) -> usize {
    match id {
        ValueId::CreateWorkshopSettingInt | ValueId::CreateWorkshopSettingFloat => 3,
        ValueId::CreateWorkshopSettingEnum => 2,
        _ => 0,
    }
}

/// An expression nested inside another (not a top-level slot).
fn value(expr: &Expr) -> usize {
    use ExprKind as K;
    let all = |items: &[&Expr]| items.iter().map(|e| value(e)).sum::<usize>();
    match expr.kind() {
        K::Bool(_) | K::Null | K::ArrayElem | K::ArrayIndex => 1,
        K::Const(c) => {
            if matches!(
                c.enum_name,
                "Hero" | "Map" | "Gamemode" | "Button" | "Color" | "Team"
            ) {
                2
            } else {
                1
            }
        }
        K::Str(StrExpr::Custom { args, .. }) => {
            let missing = STRING_SLOTS.saturating_sub(args.len());
            2 + args.iter().map(value).sum::<usize>() + missing
        }
        K::Str(StrExpr::Localized(_)) => 3 + STRING_SLOTS,
        K::Vector([x, y, z]) => 1 + all(&[x, y, z]),
        K::Array(items) if items.is_empty() => 1,
        // `Array` and `Evaluate Once` take one more than the general rule.
        K::Array(items) => 2 + items.iter().map(value).sum::<usize>(),
        // `String("phrase", a, b, c)`: the phrase and unused slots count like the bare phrase.
        K::Value {
            id: ValueId::LocalizedString,
            args,
        } if matches!(
            args.first().map(Expr::kind),
            Some(K::Str(StrExpr::Localized(_)))
        ) =>
        {
            let rest = &args[1..];
            3 + rest.iter().map(value).sum::<usize>() + STRING_SLOTS.saturating_sub(rest.len())
        }
        K::Value { id, args } => {
            let extra = usize::from(*id == ValueId::EvalOnce);
            (1 + extra + args.iter().map(value).sum::<usize>())
                .saturating_sub(setting_discount(*id))
        }
        K::Num(_) | K::GlobalVar(_) => 2,
        K::PlayerVar { player, .. } => 2 + value(player),
        K::Index { array, index } => 1 + all(&[array, index]),
        // The comparison operator is an argument too.
        K::Compare { lhs, rhs, .. } => 2 + all(&[lhs, rhs]),
        K::And(a, b)
        | K::Or(a, b)
        | K::Add(a, b)
        | K::Sub(a, b)
        | K::Mul(a, b)
        | K::Div(a, b)
        | K::Mod(a, b)
        | K::Pow(a, b) => 1 + all(&[a, b]),
        K::Not(a) => 1 + value(a),
        // `Multiply(-1, x)`
        K::Neg(a) => 3 + value(a),
        K::IfThenElse { cond, then, else_ } => 1 + all(&[cond, then, else_]),
        // Removed before counting.
        K::Dict { .. }
        | K::Translated(_)
        | K::Resolve { .. }
        | K::Decompress(_)
        | K::StrModify { .. }
        | K::LocalizedCall { .. }
        | K::HolyGrail
        | K::RuleCondition
        | K::LabelDistance(_) => 0,
    }
}

#[cfg(test)]
mod tests;
