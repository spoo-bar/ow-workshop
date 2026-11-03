//! Rules the compiler adds: the inspector switch, init rules and editor page fillers.

use ow_workshop_data::ActionId;

use super::Ctx;
use crate::ir::{
    Block, Diagnostics, Event, Expr, ExprKind, Place, PlayerFilter, Program, Rule, Stmt, StmtKind,
    VarId, Vars,
};
use ow_workshop_data::ValueId;

/// Rules per editor page.
const PAGE_SIZE: usize = 100;

fn rule(name: &str, event: Event, stmts: Vec<StmtKind>) -> Rule {
    let body = Block {
        stmts: stmts
            .into_iter()
            .map(|kind| Stmt {
                kind,
                loc: None,
                comment: None,
            })
            .collect(),
        ..Block::default()
    };
    Rule {
        location: None,
        ..Rule::new(name, event, body)
    }
}

/// The inspector is disabled for the whole match.
fn inspector_rule() -> Rule {
    rule(
        "Disable inspector",
        Event::OngoingGlobal,
        vec![StmtKind::Action {
            id: ActionId::DisableInspector,
            args: vec![],
        }],
    )
}

/// Strings epic: the tags setup rule, when tags are enabled.
fn tags_rule(_: &Program, _: &Ctx<'_>) -> Option<Rule> {
    None
}

/// Translations epic: the language-detection rule, when player-variable translations are used.
fn translation_rule(_: &Program, _: &Ctx<'_>) -> Option<Rule> {
    None
}

/// The `(variable, initial value)` pairs in declaration order.
fn initial_values(vars: &Vars) -> impl Iterator<Item = (VarId, &Expr)> {
    vars.entries.iter().enumerate().filter_map(|(i, decl)| {
        let id = VarId(u32::try_from(i).ok()?);
        decl.init.as_ref().map(|init| (id, init))
    })
}

fn global_init_rule(program: &Program, ctx: &Ctx<'_>) -> Option<Rule> {
    let sets: Vec<StmtKind> = initial_values(&program.globals)
        .map(|(var, init)| StmtKind::Set {
            target: Place::Global(var),
            value: init.clone(),
        })
        .collect();
    (!sets.is_empty()).then(|| {
        rule(
            &ctx.options.global_init_rule_name,
            Event::OngoingGlobal,
            sets,
        )
    })
}

fn player_init_rule(program: &Program, ctx: &Ctx<'_>) -> Option<Rule> {
    let event_player = || {
        Expr::new(
            ExprKind::Value {
                id: ValueId::EventPlayer,
                args: vec![],
            },
            None,
        )
    };
    let sets: Vec<StmtKind> = initial_values(&program.players)
        .map(|(var, init)| StmtKind::Set {
            target: Place::Player {
                player: event_player(),
                var,
            },
            value: init.clone(),
        })
        .collect();
    (!sets.is_empty()).then(|| Rule {
        filter: Some(PlayerFilter::default()),
        ..rule(
            &ctx.options.player_init_rule_name,
            Event::OngoingEachPlayer,
            sets,
        )
    })
}

/// Empty rules that push the next rule onto a new editor page.
fn fillers(name: &str, rules_so_far: usize) -> impl Iterator<Item = Rule> {
    // A non-zero multiple of the page size is already a page boundary.
    let count = match rules_so_far % PAGE_SIZE {
        0 if rules_so_far == 0 => PAGE_SIZE,
        0 => 0,
        rest => PAGE_SIZE - rest,
    };
    let name = name.to_owned();
    (0..count).map(move |_| Rule {
        delimiter: true,
        disabled: name.is_empty(),
        ..rule(&name, Event::OngoingGlobal, Vec::new())
    })
}

/// Adds the generated rules before all user rules, then the page fillers.
pub(crate) fn add_rules(program: &mut Program, ctx: &mut Ctx<'_>) -> Result<(), Diagnostics> {
    let mut generated = Vec::new();
    if ctx.options.disable_inspector {
        generated.push(inspector_rule());
    }
    generated.extend(tags_rule(program, ctx));
    generated.extend(translation_rule(program, ctx));
    generated.extend(global_init_rule(program, ctx));
    generated.extend(player_init_rule(program, ctx));

    let user_rules = std::mem::take(&mut program.rules);
    let mut rules = generated;
    rules.reserve(user_rules.len());
    for mut user in user_rules {
        if let Some(name) = user.new_page.take() {
            let before = rules.len();
            rules.extend(fillers(&name, before));
        }
        rules.push(user);
    }
    program.rules = rules;
    Ok(())
}

#[cfg(test)]
mod tests;
