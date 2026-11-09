//! Traversal helpers, so passes do not hand-write recursion over the IR.

#![allow(dead_code)]

use crate::ir::{
    Block, Decompress, Expr, ExprKind, Place, SkipTarget, Stmt, StmtKind, StrExpr, Translated,
};

/// Calls `f` on `stmt`, then on every statement nested inside it.
pub(crate) fn walk_stmt(stmt: &Stmt, f: &mut impl FnMut(&Stmt)) {
    f(stmt);
    for block in stmt.blocks() {
        walk_block(block, f);
    }
}

/// Calls `f` on every statement of `block`, outer before inner.
pub(crate) fn walk_block(block: &Block, f: &mut impl FnMut(&Stmt)) {
    for stmt in &block.stmts {
        walk_stmt(stmt, f);
    }
}

/// The blocks nested directly inside a statement, mutably.
pub(crate) fn blocks_mut(stmt: &mut Stmt) -> Vec<&mut Block> {
    match &mut stmt.kind {
        StmtKind::If {
            then, elifs, else_, ..
        } => std::iter::once(then)
            .chain(elifs.iter_mut().map(|(_, b)| b))
            .chain(else_)
            .collect(),
        StmtKind::While { body, .. }
        | StmtKind::For { body, .. }
        | StmtKind::DoWhile { body, .. } => vec![body],
        StmtKind::Switch { cases, default, .. } => {
            cases.iter_mut().map(|(_, b)| b).chain(default).collect()
        }
        _ => Vec::new(),
    }
}

/// Replaces statements bottom-up: nested blocks are rewritten first, then `f` maps each
/// statement to the statements that take its place.
pub(crate) fn fold_block(block: &mut Block, f: &mut impl FnMut(Stmt) -> Vec<Stmt>) {
    let stmts = std::mem::take(&mut block.stmts);
    for mut stmt in stmts {
        for child in blocks_mut(&mut stmt) {
            fold_block(child, f);
        }
        block.stmts.extend(f(stmt));
    }
}

/// Rebuilds `expr` bottom-up: children are rewritten first, then `f` maps the rebuilt node.
pub(crate) fn rewrite_expr(expr: &Expr, f: &mut impl FnMut(Expr) -> Expr) -> Expr {
    let kind = map_children(expr.kind(), &mut |child| rewrite_expr(child, f));
    f(Expr::new(kind, expr.loc()))
}

/// Like [`rewrite_expr`], but nodes for which `skip` holds are kept as they are, children and all.
pub(crate) fn rewrite_expr_skipping(
    expr: &Expr,
    skip: &impl Fn(&Expr) -> bool,
    f: &mut impl FnMut(Expr) -> Expr,
) -> Expr {
    if skip(expr) {
        return expr.clone();
    }
    let kind = map_children(expr.kind(), &mut |child| {
        rewrite_expr_skipping(child, skip, f)
    });
    f(Expr::new(kind, expr.loc()))
}

fn pair(g: &mut impl FnMut(&Expr) -> Expr, a: &Expr, b: &Expr) -> (Expr, Expr) {
    (g(a), g(b))
}

/// The same expression shape with every direct child replaced by `g(child)`.
pub(crate) fn map_children(kind: &ExprKind, g: &mut impl FnMut(&Expr) -> Expr) -> ExprKind {
    use ExprKind as K;
    match kind {
        K::Num(_)
        | K::Bool(_)
        | K::Null
        | K::Const(_)
        | K::Str(StrExpr::Localized(_))
        | K::GlobalVar(_)
        | K::ArrayElem
        | K::ArrayIndex
        | K::RuleCondition
        | K::HolyGrail
        | K::LabelDistance(_) => kind.clone(),
        K::Vector([x, y, z]) => K::Vector([g(x), g(y), g(z)]),
        K::Array(items) => K::Array(items.iter().map(&mut *g).collect()),
        K::Str(StrExpr::Custom { tokens, args }) => K::Str(StrExpr::Custom {
            tokens: tokens.clone(),
            args: args.iter().map(&mut *g).collect(),
        }),
        K::Value { id, args } => K::Value {
            id: *id,
            args: args.iter().map(&mut *g).collect(),
        },
        K::PlayerVar { player, var } => K::PlayerVar {
            player: g(player),
            var: *var,
        },
        K::Index { array, index } => {
            let (array, index) = pair(g, array, index);
            K::Index { array, index }
        }
        K::Compare { op, lhs, rhs } => {
            let (lhs, rhs) = pair(g, lhs, rhs);
            K::Compare { op: *op, lhs, rhs }
        }
        K::And(a, b) => {
            let (a, b) = pair(g, a, b);
            K::And(a, b)
        }
        K::Or(a, b) => {
            let (a, b) = pair(g, a, b);
            K::Or(a, b)
        }
        K::Add(a, b) => {
            let (a, b) = pair(g, a, b);
            K::Add(a, b)
        }
        K::Sub(a, b) => {
            let (a, b) = pair(g, a, b);
            K::Sub(a, b)
        }
        K::Mul(a, b) => {
            let (a, b) = pair(g, a, b);
            K::Mul(a, b)
        }
        K::Div(a, b) => {
            let (a, b) = pair(g, a, b);
            K::Div(a, b)
        }
        K::Mod(a, b) => {
            let (a, b) = pair(g, a, b);
            K::Mod(a, b)
        }
        K::Pow(a, b) => {
            let (a, b) = pair(g, a, b);
            K::Pow(a, b)
        }
        K::LocalizedCall { phrase, args } => K::LocalizedCall {
            phrase: phrase.clone(),
            args: args.iter().map(&mut *g).collect(),
        },
        K::StrModify { modifier, inner } => K::StrModify {
            modifier: *modifier,
            inner: g(inner),
        },
        K::Not(a) => K::Not(g(a)),
        K::Neg(a) => K::Neg(g(a)),
        K::IfThenElse { cond, then, else_ } => K::IfThenElse {
            cond: g(cond),
            then: g(then),
            else_: g(else_),
        },
        K::Dict { entries, default } => K::Dict {
            entries: entries.iter().map(|(k, v)| (g(k), g(v))).collect(),
            default: default.as_ref().map(&mut *g),
        },
        K::Translated(t) => K::Translated(Translated {
            args: t.args.iter().map(&mut *g).collect(),
            ..t.clone()
        }),
        K::Resolve { inner, spectator } => K::Resolve {
            inner: g(inner),
            spectator: *spectator,
        },
        K::Decompress(d) => K::Decompress(Box::new(Decompress {
            packed: g(&d.packed),
            ..(**d).clone()
        })),
    }
}

/// Rewrites `expr` top-down: `f` may replace a node (its children are then left alone);
/// otherwise the children are rewritten.
pub(crate) fn rewrite_expr_top(expr: &Expr, f: &mut impl FnMut(&Expr) -> Option<Expr>) -> Expr {
    if let Some(replacement) = f(expr) {
        return replacement;
    }
    let kind = map_children(expr.kind(), &mut |child| rewrite_expr_top(child, f));
    Expr::new(kind, expr.loc())
}

/// Replaces the root expressions of a place (owning player, indices) by `g(root)`.
pub(crate) fn map_place(place: &mut Place, g: &mut impl FnMut(&Expr) -> Expr) {
    match place {
        Place::Global(_) => {}
        Place::Player { player, .. } => *player = g(player),
        Place::Index { base, index } => {
            map_place(base, g);
            *index = g(index);
        }
    }
}

/// Replaces the root expressions of `stmt` itself, not those of nested blocks, by `g(root)`.
pub(crate) fn map_stmt_roots(stmt: &mut Stmt, g: &mut impl FnMut(&Expr) -> Expr) {
    let mut r = |e: &mut Expr| *e = g(e);
    match &mut stmt.kind {
        StmtKind::Action { args, .. } => args.iter_mut().for_each(&mut r),
        StmtKind::Set { target, value } | StmtKind::Modify { target, value, .. } => {
            map_place(target, g);
            *value = g(value);
        }
        StmtKind::Chase {
            target,
            dest,
            rate_or_duration,
            reeval,
            ..
        } => {
            map_place(target, g);
            for e in [dest, rate_or_duration, reeval] {
                *e = g(e);
            }
        }
        StmtKind::StopChasing(place) => map_place(place, g),
        StmtKind::If { cond, elifs, .. } => {
            *cond = g(cond);
            for (c, _) in elifs.iter_mut() {
                *c = g(c);
            }
        }
        StmtKind::While { cond, .. }
        | StmtKind::DoWhile { cond, .. }
        | StmtKind::AbortIf(cond)
        | StmtKind::LoopIf(cond) => *cond = g(cond),
        StmtKind::For {
            var,
            start,
            stop,
            step,
            ..
        } => {
            map_place(var, g);
            for e in [start, stop, step] {
                *e = g(e);
            }
        }
        StmtKind::Skip(SkipTarget::Dynamic(e)) => *e = g(e),
        StmtKind::SkipIf(cond, target) => {
            *cond = g(cond);
            if let SkipTarget::Dynamic(e) = target {
                *e = g(e);
            }
        }
        StmtKind::Switch { value, cases, .. } => {
            *value = g(value);
            for (c, _) in cases.iter_mut() {
                *c = g(c);
            }
        }
        _ => {}
    }
}

/// Rewrites the expressions of `stmt` itself bottom-up, not those of nested blocks.
pub(crate) fn rewrite_stmt_exprs(stmt: &mut Stmt, f: &mut impl FnMut(Expr) -> Expr) {
    map_stmt_roots(stmt, &mut |e| rewrite_expr(e, f));
}

/// Rewrites every expression in `block`, including those of nested blocks, bottom-up.
pub(crate) fn rewrite_block_exprs(block: &mut Block, f: &mut impl FnMut(Expr) -> Expr) {
    map_block_roots(block, &mut |e| rewrite_expr(e, f));
}

/// Replaces every root expression in `block`, including those of nested blocks, by `g(root)`.
pub(crate) fn map_block_roots(block: &mut Block, g: &mut impl FnMut(&Expr) -> Expr) {
    for stmt in &mut block.stmts {
        map_stmt_roots(stmt, g);
        for child in blocks_mut(stmt) {
            map_block_roots(child, g);
        }
    }
}
