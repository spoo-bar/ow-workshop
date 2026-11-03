//! Lowerings of specific functions: `rule_condition`, the current-map bug, HUD alignment and a
//! few argument limits.

use ow_workshop_data::signature::Signature;
use ow_workshop_data::{ActionId, ValueId};

use super::labels::has_dynamic_skip;
use super::visit::{
    fold_block, map_block_roots, rewrite_block_exprs, rewrite_expr, rewrite_expr_top, walk_block,
};
use super::{Ctx, RuleCtx};
use crate::ir::{
    Block, CmpOp, ConstRef, Diagnostics, Expr, ExprKind, Location, Program, Rule, SkipTarget, Stmt,
    StmtKind, StrExpr, StrToken,
};
use crate::options::OptLevel;

/// Maps whose `Map(X)` constant is unreliable.
const BUGGED_MAPS: [&str; 4] = ["colosseo", "esperanca", "samoa", "throneOfAnubis"];
/// The bugged maps that `Current Map` cannot tell apart from their duplicates by `==`.
const COMPARABLE_BUGGED_MAPS: [&str; 3] = ["colosseo", "esperanca", "samoa"];
/// Padding that aligns left HUD texts: 170 em spaces.
const HUD_PADDING: &str = "\u{2003}";
const HUD_PADDING_COUNT: usize = 170;

fn node(kind: ExprKind, loc: Option<Location>) -> Expr {
    Expr::new(kind, loc)
}

fn value(id: ValueId, args: Vec<Expr>, loc: Option<Location>) -> Expr {
    node(ExprKind::Value { id, args }, loc)
}

fn constant(enum_name: &'static str, key: &'static str, loc: Option<Location>) -> Expr {
    node(ExprKind::Const(ConstRef { enum_name, key }), loc)
}

pub(crate) fn is_const(expr: &Expr, enum_name: &str, key: &str) -> bool {
    matches!(expr.kind(), ExprKind::Const(c) if c.enum_name == enum_name && c.key == key)
}

pub(crate) fn is_null(expr: &Expr) -> bool {
    matches!(
        expr.kind(),
        ExprKind::Null
            | ExprKind::Value {
                id: ValueId::Null,
                ..
            }
    )
}

/// The position of the argument called `name` in a signature.
pub(crate) fn arg_index(sig: &Signature, name: &str) -> Option<usize> {
    sig.args.iter().position(|a| a.name == name)
}

/// `Custom String("{0}", inner)`: forces the client to render `inner` as text.
fn as_text(inner: Expr, loc: Option<Location>) -> Expr {
    node(
        ExprKind::Str(StrExpr::Custom {
            tokens: vec![StrToken::Arg(0)],
            args: vec![inner],
        }),
        loc,
    )
}

// ---- rule_condition ----------------------------------------------------------------------

/// The rule's conditions joined by `And`, or `True` when there are none.
fn rule_condition_expr(rule: &Rule, loc: Option<Location>) -> Expr {
    let mut conditions = rule.conditions.iter().map(|c| c.expr.clone()).rev();
    let Some(mut joined) = conditions.next() else {
        return node(ExprKind::Bool(true), loc);
    };
    for c in conditions {
        joined = node(ExprKind::And(c, joined), loc);
    }
    joined
}

/// `Some(true)` for `rule_condition()`, `Some(false)` for `!rule_condition()`.
fn rule_condition_polarity(cond: &Expr) -> Option<bool> {
    match cond.kind() {
        ExprKind::RuleCondition => Some(true),
        ExprKind::Not(inner) if matches!(inner.kind(), ExprKind::RuleCondition) => Some(false),
        _ => None,
    }
}

/// The only statement of a block, if it is the only one.
fn lone(block: &Block) -> Option<&Stmt> {
    match block.stmts.as_slice() {
        [only] => Some(only),
        _ => None,
    }
}

/// `if c: abort` / `loop` / `skip(n)` as a single conditional action.
fn fuse_lone_if(stmt: Stmt, optimize: bool) -> Stmt {
    let StmtKind::If {
        cond,
        then,
        elifs,
        else_: None,
    } = &stmt.kind
    else {
        return stmt;
    };
    if !elifs.is_empty() {
        return stmt;
    }
    let Some(inner) = lone(then) else { return stmt };
    let kind = match (&inner.kind, rule_condition_polarity(cond)) {
        (StmtKind::Abort, Some(true)) => StmtKind::AbortIfConditionTrue,
        (StmtKind::Abort, Some(false)) => StmtKind::AbortIfConditionFalse,
        (StmtKind::Loop, Some(true)) => StmtKind::LoopIfConditionTrue,
        (StmtKind::Loop, Some(false)) => StmtKind::LoopIfConditionFalse,
        // An optimization: fewer lines, but it moves the skip distances of a dynamic skip.
        _ if !optimize || then.opt.is_some_and(|o| !o.enabled) => return stmt,
        (StmtKind::Abort, None) => StmtKind::AbortIf(cond.clone()),
        (StmtKind::Loop, None) => StmtKind::LoopIf(cond.clone()),
        (StmtKind::Skip(target @ SkipTarget::Label(_)), None) => {
            StmtKind::SkipIf(cond.clone(), target.clone())
        }
        _ => return stmt,
    };
    Stmt { kind, ..stmt }
}

fn lower_rule_condition(rule: &mut Rule, optimize: bool) {
    let fuse_all = optimize && !has_dynamic_skip(rule);
    fold_block(&mut rule.body, &mut |stmt| {
        vec![fuse_lone_if(stmt, fuse_all)]
    });
    let loc = rule.location;
    let joined = rule_condition_expr(rule, loc);
    rewrite_block_exprs(&mut rule.body, &mut |e| {
        if matches!(e.kind(), ExprKind::RuleCondition) {
            joined.clone()
        } else {
            e
        }
    });
}

// ---- Current Map -------------------------------------------------------------------------

fn bugged_maps(program: &Program) -> Vec<ConstRef> {
    let mut found: Vec<ConstRef> = Vec::new();
    let mut note = |e: &Expr| {
        scan_consts(e, &mut |c| {
            if c.enum_name == "Map" && BUGGED_MAPS.contains(&c.key) && !found.contains(&c) {
                found.push(c);
            }
        });
    };
    for rule in &program.rules {
        for cond in &rule.conditions {
            note(&cond.expr);
        }
        walk_block(&rule.body, &mut |stmt: &Stmt| {
            stmt.exprs().into_iter().for_each(&mut note);
        });
    }
    found
}

fn scan_consts(expr: &Expr, f: &mut impl FnMut(ConstRef)) {
    if let ExprKind::Const(c) = expr.kind() {
        f(*c);
    }
    for child in expr.kind().children() {
        scan_consts(child, f);
    }
}

fn is_current_map(expr: &Expr) -> bool {
    matches!(
        expr.kind(),
        ExprKind::Value {
            id: ValueId::GetCurrentMap,
            ..
        }
    )
}

fn is_comparable_bugged_map(expr: &Expr) -> bool {
    matches!(expr.kind(), ExprKind::Const(c) if c.enum_name == "Map" && COMPARABLE_BUGGED_MAPS.contains(&c.key))
}

/// Rewrites the uses of `Current Map` that the duplicated maps break.
fn fix_current_map(e: &Expr, bugged: &[ConstRef]) -> Option<Expr> {
    let loc = e.loc();
    if let ExprKind::Compare {
        op: CmpOp::Eq,
        lhs,
        rhs,
    } = e.kind()
    {
        if (is_current_map(lhs) && is_comparable_bugged_map(rhs))
            || (is_current_map(rhs) && is_comparable_bugged_map(lhs))
        {
            return Some(node(
                ExprKind::Compare {
                    op: CmpOp::Eq,
                    lhs: as_text(lhs.clone(), loc),
                    rhs: as_text(rhs.clone(), loc),
                },
                loc,
            ));
        }
    }
    if !is_current_map(e) || bugged.is_empty() {
        return None;
    }
    // First Of(Filtered Array(Append To Array(Array(bugged maps), Current Map),
    //                         Custom String(Current Map) == String Split(Current Array Element, [])))
    let maps = node(
        ExprKind::Array(
            bugged
                .iter()
                .map(|c| node(ExprKind::Const(*c), loc))
                .collect(),
        ),
        loc,
    );
    let candidates = value(ValueId::Concat, vec![maps, e.clone()], loc);
    let same = node(
        ExprKind::Compare {
            op: CmpOp::Eq,
            lhs: as_text(e.clone(), loc),
            rhs: value(
                ValueId::Split,
                vec![
                    node(ExprKind::ArrayElem, loc),
                    value(ValueId::EmptyArray, vec![], loc),
                ],
                loc,
            ),
        },
        loc,
    );
    Some(value(
        ValueId::FirstOf,
        vec![value(ValueId::FilteredArray, vec![candidates, same], loc)],
        loc,
    ))
}

// ---- Actions -----------------------------------------------------------------------------

/// `Custom String("{0}{1}", text, 170 em spaces)`.
fn pad(text: Expr, loc: Option<Location>) -> Expr {
    let spaces = node(
        ExprKind::Str(StrExpr::Custom {
            tokens: vec![StrToken::Text(HUD_PADDING.repeat(HUD_PADDING_COUNT))],
            args: vec![],
        }),
        loc,
    );
    node(
        ExprKind::Str(StrExpr::Custom {
            tokens: vec![StrToken::Arg(0), StrToken::Arg(1)],
            args: vec![text, spaces],
        }),
        loc,
    )
}

fn only_spaces(loc: Option<Location>) -> Expr {
    node(
        ExprKind::Str(StrExpr::Custom {
            tokens: vec![StrToken::Text(HUD_PADDING.repeat(HUD_PADDING_COUNT))],
            args: vec![],
        }),
        loc,
    )
}

/// Left HUD texts are centered relative to each other; padding them aligns them.
fn fix_hud_alignment(id: ActionId, args: &mut [Expr]) {
    let sig = id.signature();
    let (Some(position), Some(text)) = (arg_index(sig, "location"), arg_index(sig, "text")) else {
        return;
    };
    if !args
        .get(position)
        .is_some_and(|p| is_const(p, "HudPosition", "ACTUALLY_LEFT"))
    {
        return;
    }
    let loc = args[position].loc();
    let padded = |args: &[Expr], i: usize| pad(args[i].clone(), loc);
    match (id, arg_index(sig, "subheader")) {
        (ActionId::HudText, Some(sub)) if !is_null(&args[sub]) => args[sub] = padded(args, sub),
        _ if !is_null(&args[text]) => args[text] = padded(args, text),
        _ => args[text] = only_spaces(loc),
    }
    args[position] = constant("HudPosition", "LEFT", loc);
}

fn fix_action(id: ActionId, args: &mut [Expr]) {
    match id {
        ActionId::HudText | ActionId::ProgressBarHud => fix_hud_alignment(id, args),
        // The editor limits number literals here to integers.
        ActionId::SetUltCharge => {
            let Some(charge) = args.get_mut(1) else {
                return;
            };
            if matches!(charge.kind(), ExprKind::Num(n) if n.fract() != 0.0) {
                *charge = value(ValueId::Abs, vec![charge.clone()], charge.loc());
            }
        }
        _ => {}
    }
}

/// `Min(a, b, c)` → `Min(Min(a, b), c)`.
fn binary_min_max(e: Expr) -> Expr {
    let ExprKind::Value { id, args } = e.kind() else {
        return e;
    };
    if !matches!(id, ValueId::Min | ValueId::Max) || args.len() <= 2 {
        return e;
    }
    let mut args = args.iter().cloned();
    let first = args.next().into_iter().collect::<Vec<_>>();
    first
        .into_iter()
        .chain(args)
        .reduce(|a, b| value(*id, vec![a, b], e.loc()))
        .unwrap_or(e)
}

fn lower_actions(block: &mut Block) {
    fold_block(block, &mut |mut stmt| {
        if let StmtKind::Action { id, args } = &mut stmt.kind {
            fix_action(*id, args);
        }
        vec![stmt]
    });
}

/// Validates, warns and rewrites the function-specific constructs.
pub(crate) fn lower(program: &mut Program, ctx: &mut Ctx<'_>) -> Result<(), Diagnostics> {
    let bugged = bugged_maps(program);
    for rule in &mut program.rules {
        let optimize = RuleCtx::new(rule, ctx.options).opt != OptLevel::Off;
        lower_rule_condition(rule, optimize);
        let mut current_map =
            |e: &Expr| rewrite_expr_top(e, &mut |node| fix_current_map(node, &bugged));
        map_block_roots(&mut rule.body, &mut current_map);
        for cond in &mut rule.conditions {
            cond.expr = current_map(&cond.expr);
            cond.expr = rewrite_expr(&cond.expr, &mut binary_min_max);
        }
        rewrite_block_exprs(&mut rule.body, &mut binary_min_max);
        lower_actions(&mut rule.body);
    }
    Ok(())
}

#[cfg(test)]
mod tests;
