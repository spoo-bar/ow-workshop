//! Rewrites of array indexing, searching, building and lambdas.

// Operands are named after the maths they mirror.
#![allow(clippy::many_single_char_names)]

use ow_workshop_data::ValueId as V;

use super::Env;
use super::build::{call_args, is_num, num, num_of, value};
use super::helpers::{
    always_equal, array_items, contains_lambda_vars, definitely_falsy, definitely_truthy,
    is_literal,
};
use crate::ir::{CmpOp, Expr, ExprKind};
use crate::warning::Warning;

fn array(items: Vec<Expr>, like: &Expr) -> Expr {
    Expr::new(ExprKind::Array(items), like.loc())
}

fn null(like: &Expr) -> Expr {
    Expr::new(ExprKind::Null, like.loc())
}

fn boolean(b: bool, like: &Expr) -> Expr {
    Expr::new(ExprKind::Bool(b), like.loc())
}

fn is_elem(e: &Expr) -> bool {
    matches!(e.kind(), ExprKind::ArrayElem)
}

/// The array rewrites.
pub(super) fn rule(e: &Expr, env: &mut Env<'_, '_>) -> Option<Expr> {
    let strict = env.scope.strict();
    match e.kind() {
        ExprKind::Index { array, index } => value_in_array(e, array, index, strict, env),
        ExprKind::Value { id, args } => match (*id, args.as_slice()) {
            (V::Index, [a, v]) => index_of(e, a, v, strict),
            (V::ArrayContains, [a, v]) => contains(e, a, v, strict),
            (V::Concat, [a, v]) => concat(e, a, v, strict),
            (V::Exclude, [a, v]) => exclude(e, a, v, strict),
            (V::Slice, [a, start, count]) => slice(e, a, start, count),
            (V::Last, [a]) => array_items(a)?.last().cloned(),
            (V::Len, [a]) => count_of(e, a),
            (V::FilteredArray, [a, cond]) => filtered(e, a, cond, env.scope.size()),
            (V::MappedArray, [a, f]) => is_elem(f).then(|| a.clone()),
            (V::All, [a, cond]) => all(e, a, cond),
            (V::Any, [a, cond]) => any(e, a, cond),
            (V::SortedArray, [a, key]) => sorted(e, a, key),
            _ => None,
        },
        _ => None,
    }
}

// ---- indexing and searching ---------------------------------------------------------------

fn value_in_array(
    e: &Expr,
    array_expr: &Expr,
    index: &Expr,
    strict: bool,
    env: &mut Env<'_, '_>,
) -> Option<Expr> {
    if let Some(n) = num_of(index) {
        let i = n.round();
        let items = array_items(array_expr);
        // `[][0]` is a vector-ish quirk: in strict mode an empty array keeps its index.
        let keep_empty = strict && items.is_some_and(<[Expr]>::is_empty);
        if i < 0.0 {
            return (!keep_empty).then(|| null(e));
        }
        if let Some(items) = items {
            // `i` is a rounded non-negative number; the cast saturates for huge indices.
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let at = i as usize;
            return match items.get(at) {
                Some(item) => Some(item.clone()),
                None => (!keep_empty).then(|| null(e)),
            };
        }
        if i == 0.0 {
            return Some(value(V::FirstOf, vec![array_expr.clone()], e.loc()));
        }
    }
    if matches!(array_expr.kind(), ExprKind::Num(_)) {
        env.warn(
            Warning::ArrayIndexNumber,
            "indexing a number always gives 0".to_owned(),
            e.loc(),
        );
    }
    None
}

fn index_of(e: &Expr, a: &Expr, v: &Expr, strict: bool) -> Option<Expr> {
    let items = array_items(a)?;
    if items.is_empty() {
        return Some(num(-1.0, e.loc()));
    }
    let found = items.iter().position(|x| always_equal(x, v));
    let examined = found.unwrap_or(items.len());
    let decided = is_literal(v, strict) && items[..examined].iter().all(|x| is_literal(x, strict));
    if decided {
        #[allow(clippy::cast_precision_loss)] // array lengths are tiny
        return Some(num(found.map_or(-1.0, |p| p as f64), e.loc()));
    }
    let p = found?;
    (p + 1 < items.len()).then(|| {
        value(
            V::Index,
            vec![array(items[..=p].to_vec(), a), v.clone()],
            e.loc(),
        )
    })
}

fn contains(e: &Expr, a: &Expr, v: &Expr, strict: bool) -> Option<Expr> {
    let items = array_items(a)?;
    if items.iter().any(|x| always_equal(x, v)) {
        return Some(boolean(true, e));
    }
    let literal = is_literal(v, strict);
    let kept: Vec<&Expr> = items
        .iter()
        .filter(|x| !(literal && is_literal(x, strict)))
        .collect();
    match kept.as_slice() {
        [] => Some(boolean(false, e)),
        // `v` first: `Vector(0, 0, 0) == [][0]` is true but not the reverse.
        [only] => Some(Expr::new(
            ExprKind::Compare {
                op: CmpOp::Eq,
                lhs: v.clone(),
                rhs: (*only).clone(),
            },
            e.loc(),
        )),
        _ if kept.len() < items.len() => Some(value(
            V::ArrayContains,
            vec![array(kept.into_iter().cloned().collect(), a), v.clone()],
            e.loc(),
        )),
        _ => None,
    }
}

// ---- building arrays -----------------------------------------------------------------------

fn concat(e: &Expr, a: &Expr, v: &Expr, strict: bool) -> Option<Expr> {
    let mut items = array_items(a)?.to_vec();
    match array_items(v) {
        Some(more) => items.extend_from_slice(more),
        // Only a value known not to be an array is appended as one element.
        None if is_literal(v, strict) => items.push(v.clone()),
        None => return None,
    }
    Some(array(items, e))
}

fn exclude(e: &Expr, a: &Expr, v: &Expr, strict: bool) -> Option<Expr> {
    let items = array_items(a)?;
    let removed: Vec<&Expr> = match array_items(v) {
        Some(members) => members.iter().collect(),
        None if is_literal(v, strict) => vec![v],
        None => return None,
    };
    let kept: Vec<Expr> = items
        .iter()
        .filter(|x| !removed.iter().any(|r| always_equal(x, r)))
        .cloned()
        .collect();
    let all_literal = items
        .iter()
        .chain(removed.iter().copied())
        .all(|x| is_literal(x, strict));
    if all_literal {
        return Some(array(kept, e));
    }
    (kept.len() < items.len()).then(|| value(V::Exclude, vec![array(kept, a), v.clone()], e.loc()))
}

fn slice(e: &Expr, a: &Expr, start: &Expr, count: &Expr) -> Option<Expr> {
    let items = array_items(a)?;
    let (mut start, mut count) = (num_of(start)?.round(), num_of(count)?.round());
    if start < 0.0 {
        count += start;
        start = 0.0;
    }
    if count <= 0.0 {
        return Some(array(Vec::new(), e));
    }
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let (start, end) = (start as usize, (start + count) as usize);
    let end = end.min(items.len());
    Some(array(items.get(start..end).unwrap_or_default().to_vec(), e))
}

fn count_of(e: &Expr, a: &Expr) -> Option<Expr> {
    if let Some(items) = array_items(a) {
        #[allow(clippy::cast_precision_loss)] // array lengths are tiny
        return Some(num(items.len() as f64, e.loc()));
    }
    let ExprKind::Value { id, args } = a.kind() else {
        return None;
    };
    let counted = match id {
        V::GetPlayers => V::GetNumberOfPlayers,
        V::GetLivingPlayers => V::GetNumberOfLivingPlayers,
        V::GetDeadPlayers => V::GetNumberOfDeadPlayers,
        V::GetPlayersOnObjective => V::GetNumberOfPlayersOnObjective,
        V::GetPlayersOnHero => V::GetNumberOfHeroes,
        _ => return None,
    };
    Some(value(counted, args.clone(), e.loc()))
}

// ---- lambdas -------------------------------------------------------------------------------

fn filtered(e: &Expr, a: &Expr, cond: &Expr, size: bool) -> Option<Expr> {
    if let Some([team]) = call_args(a, V::GetPlayers) {
        if let Some(found) = players_filter(e, team, cond) {
            return Some(found);
        }
    }
    if let ExprKind::Compare {
        op: CmpOp::Ne,
        lhs,
        rhs,
    } = cond.kind()
    {
        let other = match (is_elem(lhs), is_elem(rhs)) {
            (true, false) => Some(rhs),
            (false, true) => Some(lhs),
            _ => None,
        };
        if let Some(x) = other.filter(|x| !contains_lambda_vars(x)) {
            return Some(value(V::Exclude, vec![a.clone(), x.clone()], e.loc()));
        }
    }
    (!size && !contains_lambda_vars(cond)).then(|| {
        Expr::new(
            ExprKind::IfThenElse {
                cond: cond.clone(),
                then: a.clone(),
                else_: array(Vec::new(), e),
            },
            e.loc(),
        )
    })
}

/// `Filtered Array(All Players(team), cond)` as the matching player list.
fn players_filter(e: &Expr, team: &Expr, cond: &Expr) -> Option<Expr> {
    let of_team = |id| Some(value(id, vec![team.clone()], e.loc()));
    let on_elem = |c: &Expr, id| matches!(call_args(c, id), Some([p]) if is_elem(p));
    match cond.kind() {
        _ if on_elem(cond, V::IsOnObjective) => of_team(V::GetPlayersOnObjective),
        _ if on_elem(cond, V::IsAlive) => of_team(V::GetLivingPlayers),
        _ if on_elem(cond, V::IsDead) => of_team(V::GetDeadPlayers),
        ExprKind::Not(inner) if on_elem(inner, V::IsOnObjective) => {
            of_team(V::GetPlayersNotOnObjective)
        }
        ExprKind::Compare {
            op: CmpOp::Eq,
            lhs,
            rhs,
        } => {
            let hero = [(lhs, rhs), (rhs, lhs)].into_iter().find_map(|(l, r)| {
                (matches!(call_args(l, V::GetHero), Some([p]) if is_elem(p))
                    && !contains_lambda_vars(r))
                .then_some(r)
            })?;
            Some(value(
                V::GetPlayersOnHero,
                vec![hero.clone(), team.clone()],
                e.loc(),
            ))
        }
        _ => None,
    }
}

fn all(e: &Expr, a: &Expr, cond: &Expr) -> Option<Expr> {
    if array_items(a).is_some_and(<[Expr]>::is_empty) {
        return Some(boolean(true, e));
    }
    if contains_lambda_vars(cond) {
        return None;
    }
    if definitely_truthy(cond) {
        return Some(boolean(true, e));
    }
    definitely_falsy(cond).then(|| {
        let count = value(V::Len, vec![a.clone()], e.loc());
        Expr::new(ExprKind::Not(count), e.loc())
    })
}

fn any(e: &Expr, a: &Expr, cond: &Expr) -> Option<Expr> {
    if array_items(a).is_some_and(<[Expr]>::is_empty) {
        return Some(boolean(false, e));
    }
    if contains_lambda_vars(cond) {
        return None;
    }
    if definitely_falsy(cond) {
        return Some(boolean(false, e));
    }
    definitely_truthy(cond).then(|| {
        Expr::new(
            ExprKind::Compare {
                op: CmpOp::Gt,
                lhs: value(V::Len, vec![a.clone()], e.loc()),
                rhs: num(0.0, e.loc()),
            },
            e.loc(),
        )
    })
}

/// Sorting by the negated index reverses a literal array.
#[allow(clippy::float_cmp)] // the key is exactly `-1`
fn sorted(e: &Expr, a: &Expr, key: &Expr) -> Option<Expr> {
    let ExprKind::Mul(x, y) = key.kind() else {
        return None;
    };
    let negated_index = matches!(
        (x.kind(), y.kind()),
        (ExprKind::ArrayIndex, ExprKind::Num(n)) | (ExprKind::Num(n), ExprKind::ArrayIndex)
            if *n == -1.0
    );
    let mut items = array_items(a)?.to_vec();
    negated_index.then(|| {
        items.reverse();
        array(items, e)
    })
}

#[cfg(test)]
mod tests;
