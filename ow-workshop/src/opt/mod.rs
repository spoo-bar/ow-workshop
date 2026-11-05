//! The optimizer: IR rewrites that make mods faster or smaller.
//!
//! Settings are scoped per block (see [`Scope`]); when `enabled` is off no rewrite runs.

// The rewrite rules that use most of this land in the following changes.
#![allow(dead_code, unused_imports, clippy::unnecessary_wraps)]

mod actions;
mod aggressive;
mod arith;
mod arrays;
mod build;
mod conditions;
mod control;
mod helpers;
mod logic;
mod math;
mod scope;
mod size;
mod strings;
mod ty;
mod vectors;

pub(crate) use helpers::{
    always_equal, contains_lambda_vars, contains_random, definitely_falsy, definitely_truthy,
    fold_guard, is_literal,
};
pub(crate) use scope::Scope;

use std::collections::HashSet;

use crate::ir::{
    Block, Diagnostics, Expr, LabelId, Location, Program, Rule, Stmt, SubId, WarningSet,
};
use crate::lower::Ctx;
use crate::lower::visit::{blocks_mut, map_stmt_roots, rewrite_expr};
use crate::warning::Warning;

/// Which sweep over the program is running.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Phase {
    /// The rewrites proper.
    Rewrite,
    /// Whole-program size rewrites that trade readability for elements.
    Aggressive,
    /// Last touches that would undo folding if they ran earlier, e.g. direction constants.
    Finish,
}

/// What a rewrite rule can see: the settings in effect and the rule being optimized.
pub(crate) struct Env<'a, 'b> {
    /// The sweep in progress.
    pub(crate) phase: Phase,
    /// Optimizer settings for the block being walked.
    pub(crate) scope: Scope,
    /// The shared pass context, for warnings.
    pub(crate) ctx: &'a mut Ctx<'b>,
    /// Warnings the rule being optimized suppresses.
    pub(crate) suppressed: WarningSet,
    /// Whether the rule has a skip whose distance is only known at runtime: statements of such
    /// a rule must not be removed or merged.
    pub(crate) dynamic_skips: bool,
    /// Labels some skip of the rule jumps to.
    pub(crate) targeted: HashSet<LabelId>,
    /// Whether the rule has any skip at all.
    pub(crate) has_skips: bool,
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
    size::rule,
];

/// The rules of the [`Phase::Finish`] sweep.
const FINISH_RULES: &[ExprRule] = &[vectors::rule];

/// A rewrite of one statement whose expressions are already optimized.
pub(crate) type StmtRule = fn(Stmt, &mut Env<'_, '_>) -> Vec<Stmt>;

/// The statement rules of the [`Phase::Rewrite`] sweep.
const STMT_RULES: &[StmtRule] = &[actions::rule, control::rule, size::stmt_rule];

/// The statement rules of the [`Phase::Aggressive`] sweep.
const AGGRESSIVE_STMT_RULES: &[StmtRule] = &[aggressive::skip_if];

/// Most rewrites of one node before giving up; guards against rules that undo each other.
const MAX_ROUNDS: usize = 16;

/// Optimizes `expr` bottom-up with every expression rule.
pub(crate) fn optimize_expr(expr: &Expr, env: &mut Env<'_, '_>) -> Expr {
    if env.phase == Phase::Aggressive {
        return if env.scope.aggressive_size() {
            aggressive::arrays(expr)
        } else {
            expr.clone()
        };
    }
    rewrite_expr(expr, &mut |mut node| {
        for _ in 0..MAX_ROUNDS {
            let optimizing = match (env.scope.enabled(), env.phase) {
                (false, _) | (true, Phase::Aggressive) => &[],
                (true, Phase::Rewrite) => EXPR_RULES,
                (true, Phase::Finish) => FINISH_RULES,
            };
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
        map_stmt_roots(&mut stmt, &mut |e| optimize_expr(e, env));
        for child in blocks_mut(&mut stmt) {
            env.scope = outer.enter(child);
            optimize_block(child, env);
        }
        env.scope = outer;
        let mut out = vec![stmt];
        let stmt_rules = match env.phase {
            Phase::Rewrite => STMT_RULES,
            Phase::Aggressive => AGGRESSIVE_STMT_RULES,
            Phase::Finish => &[],
        };
        if outer.enabled() {
            for rule in stmt_rules {
                out = out.into_iter().flat_map(|s| rule(s, env)).collect();
            }
        }
        block.stmts.extend(out);
    }
    if outer.enabled() && env.phase == Phase::Rewrite && !env.dynamic_skips {
        control::trim_unreachable(block, &env.targeted);
    }
}

/// Optimizes `rule` in place; `false` when it can never run or does nothing and should go.
fn optimize_rule(rule: &mut Rule, ctx: &mut Ctx<'_>, called: &HashSet<SubId>) -> bool {
    let scope = Scope::new(ctx.options).enter(&rule.body);
    let (targeted, has_skips) = control::skips(rule);
    let mut env = Env {
        phase: Phase::Rewrite,
        scope,
        ctx,
        suppressed: rule.suppressed.clone(),
        dynamic_skips: crate::lower::labels::has_dynamic_skip(rule),
        targeted,
        has_skips,
    };
    for cond in &mut rule.conditions {
        cond.expr = optimize_expr(&cond.expr, &mut env);
    }
    optimize_block(&mut rule.body, &mut env);
    if !scope.enabled() || rule.delimiter {
        return true;
    }
    let Some(conditions) = conditions::simplify(&rule.conditions) else {
        return false;
    };
    rule.conditions = conditions;
    if !control::does_something(rule, called) {
        return false;
    }
    for phase in [Phase::Aggressive, Phase::Finish] {
        env.phase = phase;
        sweep(rule, &mut env);
    }
    true
}

/// Runs the current phase over the rule's conditions and body.
fn sweep(rule: &mut Rule, env: &mut Env<'_, '_>) {
    for cond in &mut rule.conditions {
        cond.expr = optimize_expr(&cond.expr, env);
    }
    optimize_block(&mut rule.body, env);
}

/// The optimize pass.
pub(crate) fn run(program: &mut Program, ctx: &mut Ctx<'_>) -> Result<(), Diagnostics> {
    let called = control::called_subroutines(&program.rules);
    program
        .rules
        .retain_mut(|rule| optimize_rule(rule, ctx, &called));
    Ok(())
}

#[cfg(test)]
mod tests;
#[cfg(test)]
pub(crate) mod testutil;
