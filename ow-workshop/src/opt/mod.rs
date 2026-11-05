//! The optimizer: IR rewrites that make mods faster or smaller.
//!
//! Settings are scoped per block (see [`Scope`]); when `enabled` is off no rewrite runs.

// The rewrite rules that use most of this land in the following changes.
#![allow(dead_code, unused_imports, clippy::unnecessary_wraps)]

mod actions;
mod arith;
mod arrays;
mod build;
mod conditions;
mod helpers;
mod logic;
mod math;
mod scope;
mod strings;
mod ty;

pub(crate) use helpers::{
    always_equal, contains_lambda_vars, contains_random, definitely_falsy, definitely_truthy,
    fold_guard, is_literal,
};
pub(crate) use scope::Scope;

use crate::ir::{Block, Diagnostics, Expr, Location, Program, Rule, Stmt, WarningSet};
use crate::lower::Ctx;
use crate::lower::visit::{blocks_mut, rewrite_expr, rewrite_stmt_exprs};
use crate::warning::Warning;

/// What a rewrite rule can see: the settings in effect and the rule being optimized.
pub(crate) struct Env<'a, 'b> {
    /// Optimizer settings for the block being walked.
    pub(crate) scope: Scope,
    /// The shared pass context, for warnings.
    pub(crate) ctx: &'a mut Ctx<'b>,
    /// Warnings the rule being optimized suppresses.
    pub(crate) suppressed: WarningSet,
    /// Whether the rule has a skip whose distance is only known at runtime: statements of such
    /// a rule must not be removed or merged.
    pub(crate) dynamic_skips: bool,
}

impl Env<'_, '_> {
    /// Reports a warning unless the rule or the options silence it.
    pub(crate) fn warn(&mut self, warning: Warning, message: String, loc: Option<Location>) {
        self.ctx.warn(&self.suppressed, warning, message, loc);
    }
}

/// A rewrite of one expression whose children are already optimized; `None` leaves it as is.
pub(crate) type ExprRule = fn(&Expr, &mut Env<'_, '_>) -> Option<Expr>;

/// Rules that run even with the optimizer off: the workshop cannot express what they remove.
const REQUIRED_RULES: &[ExprRule] = &[arith::negative_literal];

/// The optimizing rules, tried in order on every node.
const EXPR_RULES: &[ExprRule] = &[
    arith::rule,
    math::rule,
    logic::rule,
    arrays::rule,
    strings::rule,
];

/// A rewrite of one statement whose expressions are already optimized.
pub(crate) type StmtRule = fn(Stmt, &mut Env<'_, '_>) -> Vec<Stmt>;

/// The statement rules, applied in order to every statement of an enabled block.
const STMT_RULES: &[StmtRule] = &[actions::rule];

/// Most rewrites of one node before giving up; guards against rules that undo each other.
const MAX_ROUNDS: usize = 16;

/// Optimizes `expr` bottom-up with every expression rule.
pub(crate) fn optimize_expr(expr: &Expr, env: &mut Env<'_, '_>) -> Expr {
    rewrite_expr(expr, &mut |mut node| {
        for _ in 0..MAX_ROUNDS {
            let optimizing = if env.scope.enabled() { EXPR_RULES } else { &[] };
            let found = REQUIRED_RULES
                .iter()
                .chain(optimizing)
                .find_map(|rule| rule(&node, env));
            let Some(next) = found else { break };
            node = next;
        }
        node
    })
}

fn optimize_block(block: &mut Block, env: &mut Env<'_, '_>) {
    let outer = env.scope;
    let stmts = std::mem::take(&mut block.stmts);
    for mut stmt in stmts {
        rewrite_stmt_exprs(&mut stmt, &mut |e| optimize_expr(&e, env));
        for child in blocks_mut(&mut stmt) {
            env.scope = outer.enter(child);
            optimize_block(child, env);
        }
        env.scope = outer;
        let mut out = vec![stmt];
        if outer.enabled() {
            for rule in STMT_RULES {
                out = out.into_iter().flat_map(|s| rule(s, env)).collect();
            }
        }
        block.stmts.extend(out);
    }
}

/// Optimizes `rule` in place; `false` when it can never run and should be removed.
fn optimize_rule(rule: &mut Rule, ctx: &mut Ctx<'_>) -> bool {
    let scope = Scope::new(ctx.options).enter(&rule.body);
    let mut env = Env {
        scope,
        ctx,
        suppressed: rule.suppressed.clone(),
        dynamic_skips: crate::lower::labels::has_dynamic_skip(rule),
    };
    for cond in &mut rule.conditions {
        cond.expr = optimize_expr(&cond.expr, &mut env);
    }
    optimize_block(&mut rule.body, &mut env);
    if !scope.enabled() {
        return true;
    }
    match conditions::simplify(&rule.conditions) {
        Some(conditions) => {
            rule.conditions = conditions;
            true
        }
        None => rule.delimiter,
    }
}

/// The optimize pass.
pub(crate) fn run(program: &mut Program, ctx: &mut Ctx<'_>) -> Result<(), Diagnostics> {
    program.rules.retain_mut(|rule| optimize_rule(rule, ctx));
    Ok(())
}

#[cfg(test)]
mod tests;
#[cfg(test)]
pub(crate) mod testutil;
