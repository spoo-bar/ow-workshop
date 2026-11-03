//! Lowers dictionaries and multi-dimensional assignment.
//!
//! The workshop only reads arrays by index and only assigns one dimension, so lookups become
//! array searches and `a[i][j] = v` rebuilds `a[i]` from slices.

use ow_workshop_data::ValueId;

use super::Ctx;
use super::visit::{fold_block, rewrite_block_exprs, walk_block};
use crate::ir::{
    CmpOp, Diagnostics, Expr, ExprKind, Location, ModifyOp, Place, Program, Severity, Stmt,
    StmtKind,
};

/// Slice count meaning "to the end of the array".
const TO_THE_END: f64 = 999_999_999_999.0;

fn node(kind: ExprKind, loc: Option<Location>) -> Expr {
    Expr::new(kind, loc)
}

fn value(id: ValueId, args: Vec<Expr>, loc: Option<Location>) -> Expr {
    node(ExprKind::Value { id, args }, loc)
}

/// `Value In Array(Array(values), [1 +] Index Of Array Value(Array(keys), key))`.
fn lookup(
    entries: &[(Expr, Expr)],
    default: Option<&Expr>,
    key: &Expr,
    loc: Option<Location>,
) -> Expr {
    let keys = node(
        ExprKind::Array(entries.iter().map(|(k, _)| k.clone()).collect()),
        loc,
    );
    let index_of = value(ValueId::Index, vec![keys, key.clone()], loc);
    let mut values: Vec<Expr> = default.into_iter().cloned().collect();
    values.extend(entries.iter().map(|(_, v)| v.clone()));
    // A missing key gives -1; shifting by one selects the default in front.
    let index = if default.is_some() {
        node(ExprKind::Add(node(ExprKind::Num(1.0), loc), index_of), loc)
    } else {
        index_of
    };
    node(
        ExprKind::Index {
            array: node(ExprKind::Array(values), loc),
            index,
        },
        loc,
    )
}

/// Rewrites `dict[key]` lookups.
fn lower_lookups(program: &mut Program) {
    for rule in &mut program.rules {
        rewrite_block_exprs(&mut rule.body, &mut |e| match e.kind() {
            ExprKind::Index { array, index } => match array.kind() {
                ExprKind::Dict { entries, default } => {
                    lookup(entries, default.as_ref(), index, e.loc())
                }
                _ => e,
            },
            _ => e,
        });
    }
}

fn find_dict(expr: &Expr) -> Option<&Expr> {
    if matches!(expr.kind(), ExprKind::Dict { .. }) {
        return Some(expr);
    }
    expr.kind().children().into_iter().find_map(find_dict)
}

/// Reports dictionaries that are still around after the lookups were lowered.
fn report_unread(program: &Program, errors: &mut Diagnostics) {
    for rule in &program.rules {
        walk_block(&rule.body, &mut |stmt: &Stmt| {
            if let Some(dict) = stmt.exprs().into_iter().find_map(find_dict) {
                errors.push(
                    Severity::Error,
                    "dict-not-indexed",
                    format!(
                        "rule `{}`: a dictionary can only be read with `get`",
                        rule.name
                    ),
                    dict.loc().or(stmt.loc),
                );
            }
        });
    }
}

fn read(place: &Place, loc: Option<Location>) -> Expr {
    match place {
        Place::Global(var) => node(ExprKind::GlobalVar(*var), loc),
        Place::Player { player, var } => node(
            ExprKind::PlayerVar {
                player: player.clone(),
                var: *var,
            },
            loc,
        ),
        Place::Index { base, index } => node(
            ExprKind::Index {
                array: read(base, loc),
                index: index.clone(),
            },
            loc,
        ),
    }
}

/// The variable at the root of a place and its indices, outermost first.
fn split(place: &Place) -> (Place, Vec<Expr>) {
    match place {
        Place::Index { base, index } => {
            let (root, mut indices) = split(base);
            indices.push(index.clone());
            (root, indices)
        }
        root => (root.clone(), Vec::new()),
    }
}

/// `array` with the element at `index` replaced by `elem`.
fn replace_at(array: &Expr, index: &Expr, elem: Expr, loc: Option<Location>) -> Expr {
    let slice =
        |start: Expr, count: Expr| value(ValueId::Slice, vec![array.clone(), start, count], loc);
    let head = slice(node(ExprKind::Num(0.0), loc), index.clone());
    let after = node(
        ExprKind::Add(index.clone(), node(ExprKind::Num(1.0), loc)),
        loc,
    );
    let tail = slice(after, node(ExprKind::Num(TO_THE_END), loc));
    let with_elem = value(
        ValueId::Concat,
        vec![head, node(ExprKind::Array(vec![elem]), loc)],
        loc,
    );
    value(ValueId::Concat, vec![with_elem, tail], loc)
}

/// `Set a[i] = …` that stores `leaf` at the two- or three-dimensional `target`.
fn assign_multi(target: &Place, leaf: Expr, loc: Option<Location>) -> StmtKind {
    let (root, indices) = split(target);
    let depth = indices.len();
    // `outer[l]` is the array `a[i0]…[il]`.
    let mut outer = Vec::with_capacity(depth - 1);
    let mut current = read(&root, loc);
    for index in &indices[..depth - 1] {
        current = node(
            ExprKind::Index {
                array: current,
                index: index.clone(),
            },
            loc,
        );
        outer.push(current.clone());
    }
    let mut new = replace_at(&outer[depth - 2], &indices[depth - 1], leaf, loc);
    for l in (0..depth - 2).rev() {
        new = replace_at(&outer[l], &indices[l + 1], new, loc);
    }
    StmtKind::Set {
        target: Place::Index {
            base: Box::new(root),
            index: indices[0].clone(),
        },
        value: new,
    }
}

fn depth(place: &Place) -> usize {
    match place {
        Place::Index { base, .. } => 1 + depth(base),
        _ => 0,
    }
}

/// The new value of `target` after `op` with `operand`.
fn modified(target: &Place, op: ModifyOp, operand: Expr, loc: Option<Location>) -> Expr {
    let current = read(target, loc);
    let both = |f: fn(Expr, Expr) -> ExprKind| node(f(current.clone(), operand.clone()), loc);
    let call = |id| value(id, vec![current.clone(), operand.clone()], loc);
    match op {
        ModifyOp::Add => both(ExprKind::Add),
        ModifyOp::Subtract => both(ExprKind::Sub),
        ModifyOp::Multiply => both(ExprKind::Mul),
        ModifyOp::Divide => both(ExprKind::Div),
        ModifyOp::Modulo => both(ExprKind::Mod),
        ModifyOp::RaiseToPower => both(ExprKind::Pow),
        ModifyOp::Min => call(ValueId::Min),
        ModifyOp::Max => call(ValueId::Max),
        ModifyOp::AppendToArray => call(ValueId::Concat),
        ModifyOp::RemoveFromArrayByValue => call(ValueId::Exclude),
        ModifyOp::RemoveFromArrayByIndex => value(
            ValueId::FilteredArray,
            vec![
                current,
                node(
                    ExprKind::Compare {
                        op: CmpOp::Ne,
                        lhs: node(ExprKind::ArrayIndex, loc),
                        rhs: operand,
                    },
                    loc,
                ),
            ],
            loc,
        ),
    }
}

fn lower_multi(stmt: Stmt) -> Vec<Stmt> {
    let loc = stmt.loc;
    let kind = match &stmt.kind {
        StmtKind::Set { target, value } if depth(target) >= 2 => {
            assign_multi(target, value.clone(), loc)
        }
        StmtKind::Modify { target, op, value } if depth(target) >= 2 => {
            assign_multi(target, modified(target, *op, value.clone(), loc), loc)
        }
        _ => return vec![stmt],
    };
    vec![Stmt { kind, ..stmt }]
}

/// Lowers dictionary reads and two- and three-dimensional assignments, appends and removals.
pub(crate) fn lower(program: &mut Program, _: &mut Ctx<'_>) -> Result<(), Diagnostics> {
    lower_lookups(program);
    let mut errors = Diagnostics::default();
    report_unread(program, &mut errors);
    for rule in &mut program.rules {
        fold_block(&mut rule.body, &mut lower_multi);
    }
    if errors.has_errors() {
        return Err(errors);
    }
    Ok(())
}

#[cfg(test)]
mod tests;
