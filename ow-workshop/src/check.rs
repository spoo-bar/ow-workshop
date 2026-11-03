//! Checks made while a rule is recorded.

use std::collections::BTreeMap;

use ow_workshop_data::ValueId;

use crate::ir::{
    Block, Diagnostics, Expr, ExprKind, LabelId, Location, Place, Rule, Severity, SkipTarget,
    StmtKind,
};

/// Reports `do_while` loops that are not the first statement of the rule body.
///
/// The workshop can only loop the whole rule, so only no-op statements may precede one.
pub(crate) fn check_do_while(body: &Block, rule: &str, diagnostics: &mut Diagnostics) {
    visit(body, true, rule, diagnostics);
}

fn visit(block: &Block, top_level: bool, rule: &str, diagnostics: &mut Diagnostics) {
    let mut leading = top_level;
    for stmt in &block.stmts {
        if matches!(stmt.kind, StmtKind::DoWhile { .. }) && !leading {
            diagnostics.push(
                Severity::Error,
                "misplaced-do-while",
                format!("rule `{rule}`: `do_while` must be the first statement of a rule or subroutine body"),
                stmt.loc,
            );
        }
        if !stmt.is_noop() {
            leading = false;
        }
        for child in stmt.blocks() {
            visit(child, false, rule, diagnostics);
        }
    }
}

/// Where a label is placed or used.
struct Site {
    rule: usize,
    order: usize,
    loc: Option<Location>,
}

#[derive(Default)]
struct LabelUse {
    placed: Vec<Site>,
    skips: Vec<Site>,
}

/// Reports label misuse: placed twice, never placed, used in another rule, or placed before
/// the skip that targets it.
pub(crate) fn check_labels(rules: &[Rule], diagnostics: &mut Diagnostics) {
    let mut labels: BTreeMap<LabelId, LabelUse> = BTreeMap::new();
    for (rule_index, rule) in rules.iter().enumerate() {
        let mut order = 0;
        collect(&rule.body, rule_index, &mut order, &mut labels);
    }
    for usage in labels.values() {
        let mut error = |code: &'static str, message: &str, loc: Option<Location>| {
            diagnostics.push(Severity::Error, code, message.to_owned(), loc);
        };
        for extra in usage.placed.iter().skip(1) {
            error(
                "label-placed-twice",
                "label is placed more than once",
                extra.loc,
            );
        }
        let Some(placed) = usage.placed.first() else {
            if let Some(skip) = usage.skips.first() {
                error("label-never-placed", "label is never placed", skip.loc);
            }
            continue;
        };
        for skip in &usage.skips {
            if skip.rule != placed.rule {
                error(
                    "label-in-other-rule",
                    "label is placed in a different rule than the skip that uses it",
                    skip.loc,
                );
            } else if placed.order < skip.order {
                error(
                    "label-before-skip",
                    "label is placed before the skip; skips only jump forward",
                    skip.loc,
                );
            }
        }
    }
}

fn collect(
    block: &Block,
    rule: usize,
    order: &mut usize,
    labels: &mut BTreeMap<LabelId, LabelUse>,
) {
    for stmt in &block.stmts {
        *order += 1;
        let site = Site {
            rule,
            order: *order,
            loc: stmt.loc,
        };
        match &stmt.kind {
            StmtKind::Label(id) => labels.entry(*id).or_default().placed.push(site),
            StmtKind::Skip(SkipTarget::Label(id)) | StmtKind::SkipIf(_, SkipTarget::Label(id)) => {
                labels.entry(*id).or_default().skips.push(site);
            }
            _ => {}
        }
        for child in stmt.blocks() {
            collect(child, rule, order, labels);
        }
    }
}

/// Reports array lambdas (`filter`, `map`, …) used inside another lambda's body: the inner one
/// would rebind `Current Array Element` and `Current Array Index`.
pub(crate) fn check_nested_lambdas(rules: &[Rule], diagnostics: &mut Diagnostics) {
    for rule in rules {
        for cond in &rule.conditions {
            walk(&cond.expr, false, diagnostics);
        }
        walk_block(&rule.body, diagnostics);
    }
}

fn walk_block(block: &Block, diagnostics: &mut Diagnostics) {
    for stmt in &block.stmts {
        for expr in stmt.exprs() {
            walk(expr, false, diagnostics);
        }
        for child in stmt.blocks() {
            walk_block(child, diagnostics);
        }
    }
}

fn is_lambda(id: ValueId) -> bool {
    matches!(
        id,
        ValueId::FilteredArray
            | ValueId::MappedArray
            | ValueId::SortedArray
            | ValueId::All
            | ValueId::Any
    )
}

fn walk(expr: &Expr, in_lambda: bool, diagnostics: &mut Diagnostics) {
    if let ExprKind::Value { id, args } = expr.kind() {
        if is_lambda(*id) {
            if in_lambda {
                diagnostics.push(
                    Severity::Error,
                    "nested-lambda",
                    "array lambdas cannot be nested: the inner one would rebind the element and index"
                        .to_owned(),
                    expr.loc(),
                );
            }
            if let [array, body] = args.as_slice() {
                walk(array, in_lambda, diagnostics);
                walk(body, true, diagnostics);
                return;
            }
        }
    }
    for child in expr.kind().children() {
        walk(child, in_lambda, diagnostics);
    }
}

/// Reports random values in the indices of two- and three-dimensional places.
///
/// Such assignments are rebuilt with slices, which evaluate each index more than once.
pub(crate) fn check_multi_dim_indices(rules: &[Rule], diagnostics: &mut Diagnostics) {
    for rule in rules {
        multi_dim_block(&rule.body, diagnostics);
    }
}

fn multi_dim_block(block: &Block, diagnostics: &mut Diagnostics) {
    for stmt in &block.stmts {
        let target = match &stmt.kind {
            StmtKind::Set { target, .. } | StmtKind::Modify { target, .. } => Some(target),
            _ => None,
        };
        if let Some(target) = target {
            if index_depth(target) >= 2 && indices(target).into_iter().any(contains_random) {
                diagnostics.push(
                    Severity::Error,
                    "random-in-multi-dim-index",
                    "indices of multi-dimensional places are evaluated more than once and must not be random"
                        .to_owned(),
                    stmt.loc,
                );
            }
        }
        for child in stmt.blocks() {
            multi_dim_block(child, diagnostics);
        }
    }
}

fn index_depth(place: &Place) -> usize {
    match place {
        Place::Index { base, .. } => 1 + index_depth(base),
        _ => 0,
    }
}

fn indices(place: &Place) -> Vec<&Expr> {
    match place {
        Place::Index { base, index } => {
            let mut all = indices(base);
            all.push(index);
            all
        }
        _ => Vec::new(),
    }
}

fn contains_random(expr: &Expr) -> bool {
    let random = matches!(
        expr.kind(),
        ExprKind::Value {
            id: ValueId::RandomRandint
                | ValueId::RandomUniform
                | ValueId::RandomChoice
                | ValueId::RandomShuffle,
            ..
        }
    );
    random || expr.kind().children().into_iter().any(contains_random)
}
