//! Passes that turn the recorded IR into plain workshop IR.
//!
//! Every pass is a [`Pass`]: it reads and rewrites the [`Program`] and reports through the
//! [`Ctx`]; there is no global state. Lowering is kept apart from optimization: lowering is
//! correct on its own and the optimizer only cleans up after it.

// The passes that use the context land in the following changes.
#![allow(dead_code)]
// Every pass shares one signature, even when it cannot fail.
#![allow(clippy::unnecessary_wraps)]

pub(crate) mod labels;
pub(crate) mod switch;
pub(crate) mod visit;

use ow_workshop_data::Lang;

use crate::ir::{Diagnostics, Event, LabelId, Program, Rule, Stmt, StmtKind};
use crate::options::{CompileOptions, OptLevel};

/// State shared by the passes of one compilation.
pub(crate) struct Ctx<'a> {
    /// The compile options.
    pub(crate) options: &'a CompileOptions,
    /// The language to compile for.
    pub(crate) lang: Lang,
    /// Warnings (and errors that do not stop the pass) found so far.
    pub(crate) diags: &'a mut Diagnostics,
    next_label: u32,
}

impl<'a> Ctx<'a> {
    /// A context for `program`; fresh labels start above the ones it already uses.
    pub(crate) fn new(
        options: &'a CompileOptions,
        lang: Lang,
        diags: &'a mut Diagnostics,
        program: &Program,
    ) -> Self {
        let mut next_label = 0;
        for rule in &program.rules {
            visit::walk_block(&rule.body, &mut |stmt: &Stmt| match &stmt.kind {
                StmtKind::Label(LabelId(id)) => next_label = next_label.max(id + 1),
                StmtKind::Skip(crate::ir::SkipTarget::Label(LabelId(id)))
                | StmtKind::SkipIf(_, crate::ir::SkipTarget::Label(LabelId(id))) => {
                    next_label = next_label.max(id + 1);
                }
                _ => {}
            });
        }
        Self {
            options,
            lang,
            diags,
            next_label,
        }
    }

    /// A label no statement uses yet.
    pub(crate) fn fresh_label(&mut self) -> LabelId {
        let id = LabelId(self.next_label);
        self.next_label += 1;
        id
    }
}

/// What a pass knows about the rule it is walking.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct RuleCtx {
    /// The rule's event.
    pub(crate) event: Event,
    /// The optimizer level in effect for the block being walked.
    pub(crate) opt: OptLevel,
}

impl RuleCtx {
    /// The context at the top of `rule`'s body.
    pub(crate) fn new(rule: &Rule, options: &CompileOptions) -> Self {
        Self {
            event: rule.event,
            opt: effective_opt(options.optimize, rule.body.opt),
        }
    }

    /// The context inside `block`, which may override the optimizer settings.
    pub(crate) fn enter(self, block: &crate::ir::Block) -> Self {
        Self {
            opt: effective_opt(self.opt, block.opt),
            ..self
        }
    }
}

/// The level after applying a block's override to `outer`.
fn effective_opt(outer: OptLevel, over: Option<crate::ir::OptOverride>) -> OptLevel {
    match over {
        None => outer,
        Some(o) if !o.enabled => OptLevel::Off,
        Some(o) if o.size => OptLevel::Size,
        Some(_) => OptLevel::Speed,
    }
}

/// A compile pass; an `Err` stops the compilation.
pub(crate) type Pass = fn(&mut Program, &mut Ctx<'_>) -> Result<(), Diagnostics>;

/// A pass that does nothing, standing in until the epic that owns it lands.
#[allow(clippy::unnecessary_wraps)]
pub(crate) fn noop(_: &mut Program, _: &mut Ctx<'_>) -> Result<(), Diagnostics> {
    Ok(())
}

/// The passes run before emitting, in order.
pub(crate) const PIPELINE: &[(&str, Pass)] = &[
    ("generated rules", noop),
    ("variable usage", noop),
    ("subroutine checks", noop),
    ("lower switch", switch::lower),
    ("lower loops", noop),
    ("lower dictionaries", noop),
    ("lower functions", noop),
    ("optimize", noop),
    ("resolve labels", labels::resolve_labels),
    ("flatten", noop),
];

/// Runs `passes` in order, stopping at the first error.
pub(crate) fn run_passes(
    passes: &[(&str, Pass)],
    program: &mut Program,
    ctx: &mut Ctx<'_>,
) -> Result<(), Diagnostics> {
    for (_, pass) in passes {
        pass(program, ctx)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests;
#[cfg(test)]
pub(crate) mod testutil;
