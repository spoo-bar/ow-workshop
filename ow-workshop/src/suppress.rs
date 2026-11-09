//! Which warnings are suppressed where: globally, per rule or per block.

use std::collections::HashMap;

use crate::ir::{Diagnostic, Expr, Kind, Location, Rule, WarningSet};
use crate::lower::visit;
use crate::options::CompileOptions;
use crate::warning::Warning;

/// The suppression levels of a program, looked up by source location.
///
/// Built once from the recorded program, before any pass rewrites it; rewritten nodes keep the
/// location of the node they came from, so passes need no suppression state of their own.
#[derive(Clone, Debug, Default)]
pub(crate) struct Suppression {
    /// `CompileOptions::suppress`.
    global: WarningSet,
    /// Warnings suppressed at a location by its rule and the blocks around it.
    scopes: HashMap<Location, WarningSet>,
}

impl Suppression {
    /// The index for `rules` compiled with `options`.
    pub(crate) fn new(options: &CompileOptions, rules: &[Rule]) -> Self {
        let mut scopes = HashMap::new();
        for rule in rules {
            let mut add = |loc: Option<Location>, set: WarningSet| {
                if let (Some(loc), false) = (loc, set.is_empty()) {
                    let entry: &mut WarningSet = scopes.entry(loc).or_default();
                    *entry = entry.union(set);
                }
            };
            let by_rule = rule.suppressed;
            add(rule.location, by_rule);
            for cond in &rule.conditions {
                add_expr(&cond.expr, by_rule, &mut add);
            }
            visit::walk_block(&rule.body, &mut |stmt| {
                let set = by_rule.union(stmt.suppress);
                add(stmt.loc, set);
                for expr in stmt.exprs() {
                    add_expr(expr, set, &mut add);
                }
            });
        }
        Self {
            global: options.suppress,
            scopes,
        }
    }

    /// Whether `warning` raised at `loc` is dropped. A warning without a location can only be
    /// suppressed globally.
    pub(crate) fn is_suppressed(&self, warning: Warning, loc: Option<Location>) -> bool {
        self.global.contains(warning)
            || loc.is_some_and(|loc| self.scopes.get(&loc).is_some_and(|s| s.contains(warning)))
    }

    /// Whether `diagnostic` is a warning that is suppressed.
    pub(crate) fn drops(&self, diagnostic: &Diagnostic) -> bool {
        match diagnostic.kind {
            Kind::Warning(warning) => self.is_suppressed(warning, diagnostic.location),
            Kind::Error(_) => false,
        }
    }
}

fn add_expr(expr: &Expr, set: WarningSet, add: &mut impl FnMut(Option<Location>, WarningSet)) {
    add(expr.loc(), set);
    for child in expr.kind().children() {
        add_expr(child, set, add);
    }
}
