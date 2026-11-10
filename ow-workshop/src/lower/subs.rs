//! Event checks for subroutine calls.
//!
//! Event values are type-checked inside rules, but a subroutine can be called from any rule.
//! Each subroutine's needs are propagated through the call graph and compared with the event
//! of the rule that calls it.

use std::collections::BTreeMap;

use ow_workshop_data::{ActionId, StartRuleBehavior, ValueId};

use super::Ctx;
use super::visit::walk_block;
use crate::ir::{Diagnostics, Event, Expr, ExprKind, Program, Rule, Stmt, StmtKind, SubId};
use crate::warning::Warning;

/// The categories of event values a body uses, or an event provides.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(clippy::struct_excessive_bools)]
pub(crate) struct Needs {
    /// `Event Player`.
    pub(crate) player: bool,
    /// `Attacker`, `Victim`, `Event Damage`, `Event Was Environment`.
    pub(crate) damage: bool,
    /// `Healer`, `Healee`, `Event Healing`, `Event Was Health Pack`.
    pub(crate) healing: bool,
    /// `Event Was Critical Hit`, `Event Ability`, `Event Direction`.
    pub(crate) damage_or_healing: bool,
    /// Contains `Wait` or `Wait Until`.
    pub(crate) wait: bool,
}

impl Needs {
    fn merge(&mut self, other: Self) -> bool {
        let before = *self;
        self.player |= other.player;
        self.damage |= other.damage;
        self.healing |= other.healing;
        self.damage_or_healing |= other.damage_or_healing;
        self.wait |= other.wait;
        *self != before
    }
}

/// The categories of event values `event` provides.
fn provided(event: Event) -> Needs {
    let (player, damage, healing) = match event {
        Event::OngoingGlobal | Event::Subroutine(_) => (false, false, false),
        Event::OngoingEachPlayer | Event::PlayerJoinedMatch | Event::PlayerLeftMatch => {
            (true, false, false)
        }
        Event::PlayerEarnedElimination
        | Event::PlayerDealtFinalBlow
        | Event::PlayerDealtDamage
        | Event::PlayerTookDamage
        | Event::PlayerDied => (true, true, false),
        Event::PlayerDealtHealing | Event::PlayerReceivedHealing => (true, false, true),
        Event::PlayerDealtKnockback | Event::PlayerReceivedKnockback => (true, true, true),
    };
    Needs {
        player,
        damage,
        healing,
        damage_or_healing: damage || healing,
        wait: false,
    }
}

fn event_name(event: Event) -> &'static str {
    match event {
        Event::OngoingGlobal => "Ongoing - Global",
        Event::OngoingEachPlayer => "Ongoing - Each Player",
        Event::PlayerEarnedElimination => "Player Earned Elimination",
        Event::PlayerDealtFinalBlow => "Player Dealt Final Blow",
        Event::PlayerDealtDamage => "Player Dealt Damage",
        Event::PlayerTookDamage => "Player Took Damage",
        Event::PlayerDied => "Player Died",
        Event::PlayerDealtHealing => "Player Dealt Healing",
        Event::PlayerReceivedHealing => "Player Received Healing",
        Event::PlayerJoinedMatch => "Player Joined Match",
        Event::PlayerLeftMatch => "Player Left Match",
        Event::PlayerDealtKnockback => "Player Dealt Knockback",
        Event::PlayerReceivedKnockback => "Player Received Knockback",
        Event::Subroutine(_) => "Subroutine",
    }
}

fn expr_needs(expr: &Expr, needs: &mut Needs) {
    if let ExprKind::Value { id, .. } = expr.kind() {
        match id {
            ValueId::EventPlayer => needs.player = true,
            ValueId::Attacker
            | ValueId::Victim
            | ValueId::EventDamage
            | ValueId::EventWasEnvironment => needs.damage = true,
            ValueId::Healer
            | ValueId::Healee
            | ValueId::EventHealing
            | ValueId::EventWasHealthPack => needs.healing = true,
            ValueId::EventWasCriticalHit | ValueId::EventAbility | ValueId::EventDirection => {
                needs.damage_or_healing = true;
            }
            _ => {}
        }
    }
    for child in expr.kind().children() {
        expr_needs(child, needs);
    }
}

/// What a subroutine body uses directly, and whom it calls.
fn scan(rule: &Rule) -> (Needs, Vec<SubId>) {
    let mut needs = Needs::default();
    let mut calls = Vec::new();
    walk_block(&rule.body, &mut |stmt: &Stmt| {
        for e in stmt.exprs() {
            expr_needs(e, &mut needs);
        }
        match &stmt.kind {
            StmtKind::Action {
                id: ActionId::Wait | ActionId::WaitUntil,
                ..
            } => needs.wait = true,
            StmtKind::CallSubroutine(id) | StmtKind::StartRule(id, _) => calls.push(*id),
            _ => {}
        }
    });
    (needs, calls)
}

/// What each subroutine needs, including what the subroutines it calls need.
pub(crate) fn propagate(program: &Program) -> BTreeMap<SubId, Needs> {
    let mut needs = BTreeMap::new();
    let mut calls = BTreeMap::new();
    for rule in &program.rules {
        let Event::Subroutine(id) = rule.event else {
            continue;
        };
        let (direct, called) = scan(rule);
        needs.insert(id, direct);
        calls.insert(id, called);
    }
    // Fixpoint over the call graph; recursion simply converges.
    loop {
        let mut changed = false;
        for (id, called) in &calls {
            for callee in called {
                let inherited = needs.get(callee).copied().unwrap_or_default();
                if let Some(mine) = needs.get_mut(id) {
                    changed |= mine.merge(inherited);
                }
            }
        }
        if !changed {
            break;
        }
    }
    needs
}

fn missing(sub: Needs, caller: Needs) -> Option<&'static str> {
    if sub.player && !caller.player {
        Some("event player values")
    } else if sub.damage_or_healing && !caller.damage_or_healing {
        Some("event damage or healing values")
    } else if sub.damage && !caller.damage {
        Some("event damage values")
    } else if sub.healing && !caller.healing {
        Some("event healing values")
    } else {
        None
    }
}

/// Warns about calls from rules whose event lacks values the subroutine reads, and about
/// restarting a waiting subroutine.
pub(crate) fn check(program: &mut Program, ctx: &mut Ctx<'_>) -> Result<(), Diagnostics> {
    let subs = propagate(program);
    for rule in &program.rules {
        // A subroutine inherits its caller's event values, so only the restart check applies.
        let in_subroutine = matches!(rule.event, Event::Subroutine(_));
        let caller = provided(rule.event);
        walk_block(&rule.body, &mut |stmt: &Stmt| {
            let (id, restart) = match &stmt.kind {
                StmtKind::CallSubroutine(id) => (*id, false),
                StmtKind::StartRule(id, behavior) => (*id, *behavior == StartRuleBehavior::Restart),
                _ => return,
            };
            let sub = subs.get(&id).copied().unwrap_or_default();
            let name = program
                .subroutines
                .entries
                .get(id.0 as usize)
                .map_or("?", |d| d.name.as_str());
            if let Some(what) = missing(sub, caller).filter(|_| !in_subroutine) {
                ctx.warn(
                    Warning::MismatchedSubroutineEvent,
                    format!(
                        "Calling subroutine {name}, which uses {what}, from a {} rule",
                        event_name(rule.event)
                    ),
                    stmt.loc,
                );
            } else if sub.wait && restart {
                ctx.warn(
                    Warning::StartRuleCrash,
                    format!(
                        "Restarting subroutine {name}, which waits, accumulates server state; the server crashes after about 465 restarts"
                    ),
                    stmt.loc,
                );
            }
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests;
