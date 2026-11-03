//! Variable usage flags and the warnings about chased variables.
//!
//! Known workshop bugs: a variable that is ever chased (even in a disabled rule) makes any `For`
//! loop using it as the loop variable not run, and in Overwatch 2 a rule condition reading a
//! chased variable does not re-trigger properly while the chase runs.

use std::collections::BTreeMap;

use super::Ctx;
use super::visit::walk_block;
use crate::ir::{Diagnostics, Expr, ExprKind, Location, Place, Program, Stmt, StmtKind, VarId};
use crate::warning::Warning;

/// Which table a variable lives in, and its id.
type Key = (Table, VarId);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Table {
    Global,
    Player,
}

/// Where a chase was found: the rule and the statement.
#[derive(Clone, Copy, Debug)]
struct Site {
    rule: usize,
    loc: Option<Location>,
}

#[derive(Default)]
struct Usage {
    chased: Option<Site>,
    chase_9999: Option<Site>,
    for_loop: bool,
    in_condition: bool,
}

fn root(place: &Place) -> Key {
    match place {
        Place::Global(var) => (Table::Global, *var),
        Place::Player { var, .. } => (Table::Player, *var),
        Place::Index { base, .. } => root(base),
    }
}

fn reads(expr: &Expr, found: &mut Vec<Key>) {
    match expr.kind() {
        ExprKind::GlobalVar(var) => found.push((Table::Global, *var)),
        ExprKind::PlayerVar { var, .. } => found.push((Table::Player, *var)),
        _ => {}
    }
    for child in expr.kind().children() {
        reads(child, found);
    }
}

fn collect_usage(program: &Program) -> BTreeMap<Key, Usage> {
    let mut usage: BTreeMap<Key, Usage> = BTreeMap::new();
    for (rule_index, rule) in program.rules.iter().enumerate() {
        let mut in_conditions = Vec::new();
        for cond in &rule.conditions {
            reads(&cond.expr, &mut in_conditions);
        }
        for key in in_conditions {
            usage.entry(key).or_default().in_condition = true;
        }
        walk_block(&rule.body, &mut |stmt: &Stmt| match &stmt.kind {
            StmtKind::Chase {
                target,
                dest,
                over_time,
                ..
            } => {
                let site = Site {
                    rule: rule_index,
                    loc: stmt.loc,
                };
                let entry = usage.entry(root(target)).or_default();
                entry.chased.get_or_insert(site);
                if !over_time && dest.kind() == &ExprKind::Num(9999.0) {
                    entry.chase_9999.get_or_insert(site);
                }
            }
            StmtKind::For { var, .. } => usage.entry(root(var)).or_default().for_loop = true,
            _ => {}
        });
    }
    usage
}

/// Fills in [`crate::ir::VarFlags`] for every variable and warns about chased variables.
pub(crate) fn collect(program: &mut Program, ctx: &mut Ctx<'_>) -> Result<(), Diagnostics> {
    let usage = collect_usage(program);
    for (&(table, id), u) in &usage {
        let vars = match table {
            Table::Global => &mut program.globals,
            Table::Player => &mut program.players,
        };
        let Some(decl) = vars.entries.get_mut(id.0 as usize) else {
            continue;
        };
        decl.flags.chased = u.chased.is_some();
        decl.flags.for_loop = u.for_loop;
        decl.flags.in_condition = u.in_condition;
    }

    for (&(table, id), u) in &usage {
        let vars = match table {
            Table::Global => &program.globals,
            Table::Player => &program.players,
        };
        let name = vars
            .entries
            .get(id.0 as usize)
            .map_or("?", |d| d.name.as_str());
        let Some(chase) = u.chased else { continue };
        let mut warn = |warning, message: String, site: Site| {
            ctx.warn(
                &program.rules[site.rule].suppressed,
                warning,
                message,
                site.loc,
            );
        };
        if u.for_loop {
            warn(
                Warning::ChasedVarInFor,
                format!(
                    "The variable {name} is chased and used in a for loop, so the loop will not run."
                ),
                chase,
            );
        }
        if u.in_condition {
            warn(
                Warning::Ow2RuleConditionChase,
                format!(
                    "The variable {name} is chased and read in a rule condition, which does not re-trigger properly in Overwatch 2 while the chase runs."
                ),
                chase,
            );
        }
        if let Some(site) = u.chase_9999 {
            warn(
                Warning::Chase9999,
                format!(
                    "Chasing {name} towards 9999: a custom game can last up to 16200 seconds; use infinity or 99999 instead."
                ),
                site,
            );
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
