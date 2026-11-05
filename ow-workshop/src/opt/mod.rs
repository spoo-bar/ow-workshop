//! The optimizer: IR rewrites that make mods faster or smaller.
//!
//! Settings are scoped per block (see [`Scope`]); when `enabled` is off no rewrite runs.

// The rewrite rules that use most of this land in the following changes.
#![allow(dead_code, unused_imports, clippy::unnecessary_wraps)]

mod arith;
mod build;
mod helpers;
mod math;
mod scope;
mod ty;

pub(crate) use helpers::{
    always_equal, contains_lambda_vars, contains_random, definitely_falsy, definitely_truthy,
    fold_guard, is_literal,
};
pub(crate) use scope::Scope;

use crate::ir::{Block, Diagnostics, Expr, Location, Program, Rule, WarningSet};
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
const EXPR_RULES: &[ExprRule] = &[arith::rule, math::rule];

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
    for stmt in &mut block.stmts {
        rewrite_stmt_exprs(stmt, &mut |e| optimize_expr(&e, env));
        let outer = env.scope;
        for child in blocks_mut(stmt) {
            env.scope = outer.enter(child);
            optimize_block(child, env);
        }
        env.scope = outer;
    }
}

fn optimize_rule(rule: &mut Rule, ctx: &mut Ctx<'_>) {
    let scope = Scope::new(ctx.options).enter(&rule.body);
    let mut env = Env {
        scope,
        ctx,
        suppressed: rule.suppressed.clone(),
    };
    for cond in &mut rule.conditions {
        cond.expr = optimize_expr(&cond.expr, &mut env);
    }
    optimize_block(&mut rule.body, &mut env);
}

/// The optimize pass.
pub(crate) fn run(program: &mut Program, ctx: &mut Ctx<'_>) -> Result<(), Diagnostics> {
    for rule in &mut program.rules {
        optimize_rule(rule, ctx);
    }
    Ok(())
}

#[cfg(test)]
mod tests;
#[cfg(test)]
pub(crate) mod testutil;
