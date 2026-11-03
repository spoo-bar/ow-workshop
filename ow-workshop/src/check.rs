//! Checks made while a rule is recorded.

use crate::ir::{Block, Diagnostics, Severity, StmtKind};

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
