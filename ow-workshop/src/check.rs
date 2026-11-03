//! Checks made while a rule is recorded.

use std::collections::BTreeMap;

use crate::ir::{Block, Diagnostics, LabelId, Location, Rule, Severity, SkipTarget, StmtKind};

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
