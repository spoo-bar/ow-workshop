//! Optimize-for-size rewrites: cheaper stand-ins for literals, in size mode only.
//!
//! Number literals cost two elements while `True`, `False` and `Null` cost one.

use ow_workshop_data::signature::ReplaceFlags;
use ow_workshop_data::{ActionId, ValueId as V};

use super::Env;
use super::build::{call_args, is_num, is_zero_vec, num_of, value};
use super::helpers::definitely_falsy;
use super::replacements;
use crate::ir::Expr;
use crate::ir::{ExprKind, ModifyOp, Place, Stmt, StmtKind, StrExpr, StrToken};

fn boolean(b: bool, like: &Expr) -> Expr {
    Expr::new(ExprKind::Bool(b), like.loc())
}

fn null(like: &Expr) -> Expr {
    Expr::new(ExprKind::Null, like.loc())
}

fn is_null(e: &Expr) -> bool {
    matches!(e.kind(), ExprKind::Null) || call_args(e, V::Null).is_some()
}

/// The argument replaced by a cheaper equal one, if the signature allows it: the first match wins.
fn replacement(arg: &Expr, flags: ReplaceFlags, env: &Env<'_, '_>) -> Option<Expr> {
    if is_num(arg, 0.0) {
        if flags.zero_by_false() {
            return Some(boolean(false, arg));
        }
        if flags.zero_by_null() {
            return Some(null(arg));
        }
    }
    if is_num(arg, 1.0) && flags.one_by_true() {
        return Some(boolean(true, arg));
    }
    // Signature flags win over the replacements the mod's author opted into.
    if let Some(found) = replacements::literal(arg, env) {
        return Some(found);
    }
    if is_zero_vec(arg) {
        if flags.null_vector_by_null() {
            return Some(null(arg));
        }
        return Some(Expr::new(
            ExprKind::Add(
                value(V::VectorLeft, Vec::new(), arg.loc()),
                value(V::VectorRight, Vec::new(), arg.loc()),
            ),
            arg.loc(),
        ));
    }
    if replacements::is_empty_string(arg) {
        return Some(if flags.empty_string_by_empty_array() {
            Expr::new(ExprKind::Array(Vec::new()), arg.loc())
        } else if let Some(found) = replacements::empty_string(arg, env) {
            found
        } else {
            // The workshop cannot write an empty string: take the first character of nothing.
            value(
                V::CharAt,
                vec![
                    Expr::new(ExprKind::Array(Vec::new()), arg.loc()),
                    boolean(false, arg),
                ],
                arg.loc(),
            )
        });
    }
    None
}

/// Replaces arguments in place; `flags(i)` gives the flags of argument `i`.
fn replace_all(
    args: &[Expr],
    env: &Env<'_, '_>,
    flags: impl Fn(usize) -> ReplaceFlags,
) -> Option<Vec<Expr>> {
    let mut changed = false;
    let out = args
        .iter()
        .enumerate()
        .map(|(i, arg)| {
            replacement(arg, flags(i), env).map_or_else(
                || arg.clone(),
                |new| {
                    changed = true;
                    new
                },
            )
        })
        .collect();
    changed.then_some(out)
}

/// The flags of argument `i`; extra arguments of a variadic value share the last one.
fn flags_of(args: &'static [ow_workshop_data::signature::Arg], i: usize) -> ReplaceFlags {
    args.get(i)
        .or_else(|| args.last())
        .map_or(ReplaceFlags::NONE, |a| a.replace)
}

fn is_workshop_setting(id: V) -> bool {
    matches!(
        id,
        V::CreateWorkshopSettingBool
            | V::CreateWorkshopSettingEnum
            | V::CreateWorkshopSettingFloat
            | V::CreateWorkshopSettingHero
            | V::CreateWorkshopSettingInt
    )
}

/// The value whose signature describes the arguments of an operator node.
fn operator_id(kind: &ExprKind) -> Option<V> {
    Some(match kind {
        ExprKind::Add(..) => V::Add,
        ExprKind::Sub(..) => V::Subtract,
        ExprKind::Mul(..) => V::Multiply,
        ExprKind::Div(..) => V::Divide,
        ExprKind::Mod(..) => V::Modulo,
        ExprKind::Pow(..) => V::RaiseToPower,
        ExprKind::Index { .. } => V::ValueInArray,
        ExprKind::IfThenElse { .. } => V::IfThenElse,
        _ => return None,
    })
}

/// The components of a vector literal with `0` and `1` replaced by `False` and `True`.
///
/// Runs in the finish sweep, after direction constants had their chance.
pub(super) fn vector_parts(parts: &[Expr; 3], env: &Env<'_, '_>) -> Option<[Expr; 3]> {
    let sig = V::Vect.signature().args;
    replace_all(parts, env, |i| flags_of(sig, i))?
        .try_into()
        .ok()
}

/// Replaces literals in the arguments of a node.
pub(super) fn rule(e: &Expr, env: &mut Env<'_, '_>) -> Option<Expr> {
    if !env.scope.size() {
        return None;
    }
    let loc = e.loc();
    match e.kind() {
        ExprKind::Value { id, args } if !is_workshop_setting(*id) => {
            let sig = id.signature().args;
            let args = replace_all(args, env, |i| flags_of(sig, i))?;
            Some(value(*id, args, loc))
        }
        ExprKind::Array(items) => {
            let sig = V::Array.signature().args;
            Some(Expr::new(
                ExprKind::Array(replace_all(items, env, |i| flags_of(sig, i))?),
                loc,
            ))
        }
        ExprKind::Str(StrExpr::Custom { tokens, args }) => {
            let sig = V::CustomString.signature().args;
            let args = replace_all(args, env, |i| flags_of(sig, i + 1))?;
            Some(Expr::new(
                ExprKind::Str(StrExpr::Custom {
                    tokens: tokens.clone(),
                    args,
                }),
                loc,
            ))
        }
        ExprKind::Compare { op, lhs, rhs } => {
            let side = |x: &Expr| {
                if is_num(x, 0.0) {
                    null(x)
                } else {
                    replacements::one(x, env)
                        .or_else(|| replacements::team_one(x, env))
                        .unwrap_or_else(|| x.clone())
                }
            };
            let (new_lhs, new_rhs) = (side(lhs), side(rhs));
            (new_lhs != *lhs || new_rhs != *rhs).then(|| {
                Expr::new(
                    ExprKind::Compare {
                        op: *op,
                        lhs: new_lhs,
                        rhs: new_rhs,
                    },
                    loc,
                )
            })
        }
        kind => {
            let sig = operator_id(kind)?.signature().args;
            let children: Vec<Expr> = kind.children().into_iter().cloned().collect();
            let replaced = replace_all(&children, env, |i| flags_of(sig, i))?;
            let mut it = replaced.into_iter();
            let mut next = || it.next();
            Some(Expr::new(rebuild(kind, &mut next)?, loc))
        }
    }
}

/// `kind` with its direct children taken in order from `next`.
fn rebuild(kind: &ExprKind, next: &mut impl FnMut() -> Option<Expr>) -> Option<ExprKind> {
    Some(match kind {
        ExprKind::Add(..) => ExprKind::Add(next()?, next()?),
        ExprKind::Sub(..) => ExprKind::Sub(next()?, next()?),
        ExprKind::Mul(..) => ExprKind::Mul(next()?, next()?),
        ExprKind::Div(..) => ExprKind::Div(next()?, next()?),
        ExprKind::Mod(..) => ExprKind::Mod(next()?, next()?),
        ExprKind::Pow(..) => ExprKind::Pow(next()?, next()?),
        ExprKind::Index { .. } => ExprKind::Index {
            array: next()?,
            index: next()?,
        },
        ExprKind::IfThenElse { .. } => ExprKind::IfThenElse {
            cond: next()?,
            then: next()?,
            else_: next()?,
        },
        _ => return None,
    })
}

// ---- statements ---------------------------------------------------------------------------

/// `0` or `1` as the cheaper `False` or `True`.
#[allow(clippy::float_cmp)] // only the exact literals 0 and 1 qualify
fn truth_value(e: &Expr) -> Option<Expr> {
    let n = num_of(e)?;
    if n == 0.0 {
        Some(boolean(false, e))
    } else if n == 1.0 {
        Some(boolean(true, e))
    } else {
        None
    }
}

/// The color arguments the game ignores when what they color is empty: (what, color) indices.
fn ignored_colors(id: ActionId) -> &'static [(usize, usize)] {
    match id {
        ActionId::ProgressBarHud => &[(1, 5), (2, 6)],
        ActionId::CreateProgressBarInWorldText => &[(1, 6), (2, 7)],
        ActionId::CreateInWorldText => &[(1, 6)],
        _ => &[],
    }
}

fn action(id: ActionId, args: &mut Vec<Expr>, env: &Env<'_, '_>) {
    if id == ActionId::Wait {
        // Waits shorter than a frame are cast to 0.016 anyway.
        if args.first().and_then(num_of).is_some_and(|d| d <= 0.016) {
            args[0] = Expr::new(ExprKind::Num(0.0), args[0].loc());
        }
    }
    let sig = id.signature().args;
    if let Some(new) = replace_all(args, env, |i| flags_of(sig, i)) {
        *args = new;
    }
    let colors: &[(usize, usize)] = if id == ActionId::HudText {
        &[(1, 6), (2, 7), (3, 8)]
    } else {
        ignored_colors(id)
    };
    for &(what, color) in colors {
        let empty = if id == ActionId::HudText {
            args.get(what).is_some_and(is_null)
        } else {
            args.get(what).is_some_and(definitely_falsy)
        };
        if empty && args.get(color).is_some_and(|c| !is_null(c)) {
            args[color] = null(&args[color]);
        }
    }
    if id == ActionId::CreateBeam {
        let grapple = matches!(
            args.get(1).map(Expr::kind),
            Some(ExprKind::Const(c)) if c.enum_name == "Beam" && c.key == "GRAPPLE"
        );
        if grapple && args.get(4).is_some_and(|c| !is_null(c)) {
            args[4] = null(&args[4]);
        }
    }
}

/// Cheaper literals in assignments, modifications and actions.
pub(super) fn stmt_rule(mut stmt: Stmt, env: &mut Env<'_, '_>) -> Vec<Stmt> {
    if !env.scope.size() {
        return vec![stmt];
    }
    match &mut stmt.kind {
        StmtKind::Action { id, args } => action(*id, args, env),
        StmtKind::Set { target, value } => {
            if is_num(value, 0.0) {
                *value = null(value);
            } else if let Some(new) = replacement(value, ReplaceFlags::NONE, env) {
                *value = new;
            }
            if let Place::Index { index, .. } = target {
                if let Some(new) = truth_value(index) {
                    *index = new;
                }
            }
        }
        StmtKind::Modify { op, value, .. } => match op {
            ModifyOp::Add
            | ModifyOp::Subtract
            | ModifyOp::Modulo
            | ModifyOp::Max
            | ModifyOp::Min
            | ModifyOp::RemoveFromArrayByIndex => {
                if let Some(new) = truth_value(value) {
                    *value = new;
                }
            }
            ModifyOp::AppendToArray | ModifyOp::RemoveFromArrayByValue if is_num(value, 0.0) => {
                *value = null(value);
            }
            _ => {}
        },
        _ => {}
    }
    vec![stmt]
}

#[cfg(test)]
mod tests;
