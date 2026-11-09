//! Resolves `Skip(Label(l))` into counted distances.
//!
//! Runs after the optimizer (which may remove statements) and counts on the structure flatten
//! prints: every action line counts; labels, comments and `Pass` do not; an `End` counts only
//! when an action line follows it, because flatten omits the `End`s that close a rule.

use super::visit::{blocks_mut, walk_block};
use super::{Ctx, RuleCtx};
use crate::ir::ErrorKind;
use crate::ir::{
    Block, Diagnostics, Expr, ExprKind, LabelId, Program, Rule, SkipTarget, Stmt, StmtKind,
};
use crate::options::OptLevel;

/// One entry of a rule in flat order.
#[derive(Clone, Debug)]
enum Item {
    /// An action line.
    Line,
    /// A skip, in pre-order position `ordinal`, with the labels it measures to.
    Skip {
        ordinal: usize,
        labels: Vec<LabelId>,
        plain: bool,
    },
    /// An `End` line, which flatten omits at the end of a rule.
    End,
    /// A label position.
    Label(LabelId),
}

/// Whether the rule contains a skip whose distance is not known at compile time.
///
/// The optimizer must not remove or merge statements of such a rule.
pub(crate) fn has_dynamic_skip(rule: &Rule) -> bool {
    let mut found = false;
    walk_block(&rule.body, &mut |stmt: &Stmt| {
        if let StmtKind::Skip(SkipTarget::Dynamic(e))
        | StmtKind::SkipIf(_, SkipTarget::Dynamic(e)) = &stmt.kind
        {
            found |= placeholder_labels(e).is_empty();
        }
    });
    found
}

/// The labels a dynamic skip expression measures to.
fn placeholder_labels(expr: &Expr) -> Vec<LabelId> {
    let mut labels = Vec::new();
    collect_placeholders(expr, &mut labels);
    labels
}

fn collect_placeholders(expr: &Expr, labels: &mut Vec<LabelId>) {
    if let ExprKind::LabelDistance(l) = expr.kind() {
        labels.push(*l);
    }
    for child in expr.kind().children() {
        collect_placeholders(child, labels);
    }
}

pub(crate) fn skip_labels(kind: &StmtKind) -> Option<Vec<LabelId>> {
    let (StmtKind::Skip(target) | StmtKind::SkipIf(_, target)) = kind else {
        return None;
    };
    Some(match target {
        SkipTarget::Label(l) => vec![*l],
        SkipTarget::Dynamic(e) => placeholder_labels(e),
    })
}

fn line() -> Item {
    Item::Line
}

fn scan(block: &Block, items: &mut Vec<Item>, ordinal: &mut usize) {
    for stmt in &block.stmts {
        match &stmt.kind {
            StmtKind::Label(l) => items.push(Item::Label(*l)),
            StmtKind::Pass | StmtKind::Comment(_) => {}
            StmtKind::If {
                then, elifs, else_, ..
            } => {
                items.push(line());
                scan(then, items, ordinal);
                for (_, b) in elifs {
                    items.push(line());
                    scan(b, items, ordinal);
                }
                if let Some(b) = else_ {
                    items.push(line());
                    scan(b, items, ordinal);
                }
                items.push(Item::End);
            }
            StmtKind::While { body, .. } | StmtKind::For { body, .. } => {
                items.push(line());
                scan(body, items, ordinal);
                items.push(Item::End);
            }
            // Gone after lowering; counted like the lines they would become.
            StmtKind::DoWhile { body, .. } => {
                scan(body, items, ordinal);
                items.push(line());
            }
            StmtKind::Switch { .. } | StmtKind::Continue => {
                for child in stmt.blocks() {
                    scan(child, items, ordinal);
                }
            }
            kind => match skip_labels(kind) {
                Some(labels) => {
                    let plain = matches!(
                        kind,
                        StmtKind::Skip(SkipTarget::Label(_))
                            | StmtKind::SkipIf(_, SkipTarget::Label(_))
                    );
                    items.push(Item::Skip {
                        ordinal: *ordinal,
                        labels,
                        plain,
                    });
                    *ordinal += 1;
                }
                None => items.push(line()),
            },
        }
    }
}

/// What the scan of one rule found.
struct Resolution {
    /// Per skip (by ordinal), the distance to each of its labels.
    distances: Vec<Vec<usize>>,
    /// Skips whose single label is at distance 0.
    zero: Vec<bool>,
}

fn resolve_rule(rule: &Rule, diags: &mut Diagnostics) -> Resolution {
    let mut items = Vec::new();
    let mut count = 0;
    scan(&rule.body, &mut items, &mut count);

    // Whether each item is printed: an `End` only when a line follows it.
    let mut counted = vec![false; items.len()];
    let mut line_follows = false;
    for (i, item) in items.iter().enumerate().rev() {
        match item {
            Item::Line | Item::Skip { .. } => {
                counted[i] = true;
                line_follows = true;
            }
            Item::End => counted[i] = line_follows,
            Item::Label(_) => {}
        }
    }
    // `before[i]` is the number of printed lines in `items[..i]`.
    let mut before = Vec::with_capacity(items.len() + 1);
    let mut total = 0;
    for &c in &counted {
        before.push(total);
        total += usize::from(c);
    }
    before.push(total);

    let mut at = std::collections::BTreeMap::new();
    for (i, item) in items.iter().enumerate() {
        let Item::Label(l) = item else { continue };
        if at.insert(*l, i).is_some() {
            diags.error(
                ErrorKind::DuplicateLabel,
                format!("rule `{}`: a label is placed twice", rule.name),
                rule.location,
            );
        }
    }

    let mut distances = vec![Vec::new(); count];
    let mut zero = vec![false; count];
    for (i, item) in items.iter().enumerate() {
        let Item::Skip {
            ordinal,
            labels,
            plain,
        } = item
        else {
            continue;
        };
        for label in labels {
            if let Some(&j) = at.get(label).filter(|&&j| j > i) {
                distances[*ordinal].push(before[j] - before[i + 1]);
            } else {
                distances[*ordinal].push(0);
                diags.error(
                    ErrorKind::LabelNotFound,
                    format!(
                        "rule `{}`: a skip targets a label that is not placed after it",
                        rule.name
                    ),
                    rule.location,
                );
            }
        }
        zero[*ordinal] = *plain && distances[*ordinal] == [0];
    }
    Resolution { distances, zero }
}

fn distance_expr(n: usize, like: &Expr) -> Expr {
    // Distances stay far below 2^53.
    #[allow(clippy::cast_precision_loss)]
    Expr::new(ExprKind::Num(n as f64), like.loc())
}

/// Replaces `LabelDistance` placeholders in `expr`, taking the `next` distance for each.
fn fill(expr: &Expr, distances: &[usize], next: &mut usize) -> Expr {
    if let ExprKind::LabelDistance(_) = expr.kind() {
        let n = distances.get(*next).copied().unwrap_or(0);
        *next += 1;
        return distance_expr(n, expr);
    }
    let kind = super::visit::map_children(expr.kind(), &mut |c| fill(c, distances, next));
    Expr::new(kind, expr.loc())
}

/// Removes zero-distance skips (when `remove`) or rewrites skips to their distances.
///
/// Returns whether anything was removed.
fn apply(
    block: &mut Block,
    rc: RuleCtx,
    ordinal: &mut usize,
    plan: &Resolution,
    remove: bool,
) -> bool {
    let mut removed = false;
    let mut kept = Vec::with_capacity(block.stmts.len());
    for mut stmt in std::mem::take(&mut block.stmts) {
        let mine = skip_labels(&stmt.kind).map(|_| {
            *ordinal += 1;
            *ordinal - 1
        });
        for child in blocks_mut(&mut stmt) {
            let crc = rc.enter(child);
            removed |= apply(child, crc, ordinal, plan, remove);
        }
        if let Some(o) = mine {
            if remove {
                if plan.zero[o] && rc.opt != OptLevel::Off {
                    removed = true;
                    continue;
                }
            } else {
                rewrite_skip(&mut stmt, &plan.distances[o]);
            }
        }
        kept.push(stmt);
    }
    block.stmts = kept;
    removed
}

fn rewrite_skip(stmt: &mut Stmt, distances: &[usize]) {
    let loc = stmt.loc;
    let (StmtKind::Skip(target) | StmtKind::SkipIf(_, target)) = &mut stmt.kind else {
        return;
    };
    match target {
        SkipTarget::Label(_) => {
            let n = distances.first().copied().unwrap_or(0);
            *target = SkipTarget::Dynamic(distance_expr(n, &Expr::new(ExprKind::Null, loc)));
        }
        SkipTarget::Dynamic(e) => {
            if !distances.is_empty() {
                *e = fill(e, distances, &mut 0);
            }
        }
    }
}

/// Resolves every label skip of the program to a distance.
///
/// With optimization on, a skip over nothing is removed; that shifts later distances, so the
/// distances are recomputed until nothing more is removed. Rules with a dynamic skip keep
/// every statement.
pub(crate) fn resolve_labels(program: &mut Program, ctx: &mut Ctx<'_>) -> Result<(), Diagnostics> {
    let mut errors = Diagnostics::default();
    for rule in &mut program.rules {
        let rc = RuleCtx::new(rule, ctx.options);
        let may_remove = !has_dynamic_skip(rule);
        loop {
            let mut found = Diagnostics::default();
            let plan = resolve_rule(rule, &mut found);
            if found.has_errors() {
                errors.0.extend(found.0);
                break;
            }
            let removed = may_remove && apply(&mut rule.body, rc, &mut 0, &plan, true);
            if !removed {
                apply(&mut rule.body, rc, &mut 0, &plan, false);
                break;
            }
        }
    }
    if errors.has_errors() {
        return Err(errors);
    }
    Ok(())
}

#[cfg(test)]
mod tests;
