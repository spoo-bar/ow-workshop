//! Traversal helpers, so passes do not hand-write recursion over the IR.

#![allow(dead_code)]

use crate::ir::{Block, Expr, ExprKind, Place, SkipTarget, Stmt, StmtKind, StrExpr, Translated};

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
        | K::RuleCondition => kind.clone(),
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
            key: t.key.clone(),
            args: t.args.iter().map(&mut *g).collect(),
        }),
    }
}

/// Rewrites the expressions inside a place (owning player, indices) bottom-up.
pub(crate) fn rewrite_place(place: &mut Place, f: &mut impl FnMut(Expr) -> Expr) {
    match place {
        Place::Global(_) => {}
        Place::Player { player, .. } => *player = rewrite_expr(player, f),
        Place::Index { base, index } => {
            rewrite_place(base, f);
            *index = rewrite_expr(index, f);
        }
    }
}

fn rw(e: &mut Expr, f: &mut impl FnMut(Expr) -> Expr) {
    *e = rewrite_expr(e, f);
}

/// Rewrites the expressions of `stmt` itself, not those of nested blocks.
pub(crate) fn rewrite_stmt_exprs(stmt: &mut Stmt, f: &mut impl FnMut(Expr) -> Expr) {
    match &mut stmt.kind {
        StmtKind::Action { args, .. } => args.iter_mut().for_each(|e| rw(e, f)),
        StmtKind::Set { target, value } | StmtKind::Modify { target, value, .. } => {
            rewrite_place(target, f);
            *value = rewrite_expr(value, f);
        }
        StmtKind::Chase {
            target,
            dest,
            rate_or_duration,
            reeval,
            ..
        } => {
            rewrite_place(target, f);
            for e in [dest, rate_or_duration, reeval] {
                *e = rewrite_expr(e, f);
            }
        }
        StmtKind::StopChasing(place) => rewrite_place(place, f),
        StmtKind::If { cond, elifs, .. } => {
            rw(cond, f);
            for (c, _) in elifs.iter_mut() {
                rw(c, f);
            }
        }
        StmtKind::While { cond, .. }
        | StmtKind::DoWhile { cond, .. }
        | StmtKind::AbortIf(cond)
        | StmtKind::LoopIf(cond) => rw(cond, f),
        StmtKind::For {
            var,
            start,
            stop,
            step,
            ..
        } => {
            rewrite_place(var, f);
            for e in [start, stop, step] {
                *e = rewrite_expr(e, f);
            }
        }
        StmtKind::Skip(SkipTarget::Dynamic(e)) => rw(e, f),
        StmtKind::SkipIf(cond, target) => {
            rw(cond, f);
            if let SkipTarget::Dynamic(e) = target {
                rw(e, f);
            }
        }
        StmtKind::Switch { value, cases, .. } => {
            rw(value, f);
            for (c, _) in cases.iter_mut() {
                rw(c, f);
            }
        }
        _ => {}
    }
}

/// Rewrites every expression in `block`, including those of nested blocks.
pub(crate) fn rewrite_block_exprs(block: &mut Block, f: &mut impl FnMut(Expr) -> Expr) {
    for stmt in &mut block.stmts {
        rewrite_stmt_exprs(stmt, f);
        for child in blocks_mut(stmt) {
            rewrite_block_exprs(child, f);
        }
    }
}
