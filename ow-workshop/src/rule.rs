//! The rule a mod defines with `Builder::rule`.

use std::marker::PhantomData;

use ow_workshop_data::Hero;

use crate::block::Block;
use crate::event::{EventType, PlayerEvent};
use crate::expr::Expr;
use crate::ir::{
    self, Cond, Diagnostics, PlayerFilter, PlayerSlot, Severity, TeamFilter, WarningSet,
};
use crate::ty::{Bool, IntoExpr};
use crate::warning::Warning;

/// A rule being defined for event `E`.
#[derive(Debug)]
pub struct Rule<E> {
    conditions: Vec<Cond>,
    body: ir::Block,
    disabled: bool,
    delimiter: bool,
    new_page: Option<String>,
    suppressed: WarningSet,
    comment: Option<String>,
    team: Option<TeamFilter>,
    slot: Option<u8>,
    hero: Option<Hero>,
    _event: PhantomData<fn() -> E>,
}

/// A condition just added to a rule.
#[derive(Debug)]
pub struct ConditionRef<'a> {
    cond: &'a mut Cond,
}

impl ConditionRef<'_> {
    /// Prints a note above the condition.
    pub fn comment(self, text: impl Into<String>) {
        self.cond.comment = Some(text.into());
    }
}

impl<E: EventType> Rule<E> {
    /// A rule without conditions or actions.
    pub(crate) fn new() -> Self {
        Self {
            conditions: Vec::new(),
            body: ir::Block::default(),
            disabled: false,
            delimiter: false,
            new_page: None,
            suppressed: WarningSet::default(),
            comment: None,
            team: None,
            slot: None,
            hero: None,
            _event: PhantomData,
        }
    }

    /// Adds a condition; all conditions must hold.
    pub fn condition(&mut self, cond: impl IntoExpr<Bool>) -> ConditionRef<'_> {
        let cond: Expr<Bool> = cond.into_expr();
        let index = self.conditions.len();
        self.conditions.push(Cond {
            expr: cond.into_ir(),
            comment: None,
        });
        ConditionRef {
            cond: &mut self.conditions[index],
        }
    }

    /// Records actions; repeated calls append.
    pub fn actions(&mut self, build: impl FnOnce(&mut Block)) {
        let mut block = Block::new();
        build(&mut block);
        self.body.stmts.extend(block.into_ir().stmts);
    }

    /// Emits the rule disabled.
    pub fn disabled(&mut self) {
        self.disabled = true;
    }

    /// Keeps the rule as a separator even if optimization empties it.
    pub fn delimiter(&mut self) {
        self.delimiter = true;
    }

    /// Starts a new editor page before this rule.
    pub fn new_page(&mut self) {
        self.new_page = Some(String::new());
    }

    /// Starts a new editor page before this rule; the filler rules are named `name`.
    pub fn new_page_named(&mut self, name: impl Into<String>) {
        self.new_page = Some(name.into());
    }

    /// Suppresses a warning for this rule.
    pub fn suppress(&mut self, warning: Warning) {
        self.suppressed.0.insert(warning.code().to_owned());
    }

    /// Attaches a free-form note to the rule.
    pub fn comment(&mut self, text: impl Into<String>) {
        self.comment = Some(text.into());
    }

    /// Finishes the rule located at the caller; records problems in `diagnostics`.
    #[track_caller]
    pub(crate) fn finish(self, name: String, event: E, diagnostics: &mut Diagnostics) -> ir::Rule {
        let mut rule = ir::Rule::new(name, event.raw().0, self.body);
        rule.conditions = self.conditions;
        rule.disabled = self.disabled;
        rule.delimiter = self.delimiter;
        rule.new_page = self.new_page;
        rule.suppressed = self.suppressed;
        rule.comment = self.comment;
        if self.slot.is_some() && self.hero.is_some() {
            diagnostics.push(
                Severity::Error,
                "slot-and-hero",
                format!(
                    "rule `{}`: `slot` and `hero` are mutually exclusive",
                    rule.name
                ),
                rule.location,
            );
        }
        if self.team.is_some() || self.slot.is_some() || self.hero.is_some() {
            let player = match (self.slot, self.hero) {
                (Some(slot), _) => PlayerSlot::Slot(slot),
                (None, Some(hero)) => PlayerSlot::Hero(hero),
                (None, None) => PlayerSlot::All,
            };
            rule.filter = Some(PlayerFilter {
                team: self.team.unwrap_or_default(),
                player,
            });
        }
        rule
    }
}

impl<E: PlayerEvent> Rule<E> {
    /// Only fires for players of `team`.
    pub fn team(&mut self, team: TeamFilter) {
        self.team = Some(team);
    }

    /// Only fires for the player in `slot` (0–11).
    pub fn slot(&mut self, slot: u8) {
        self.slot = Some(slot);
    }

    /// Only fires for players playing `hero`.
    pub fn hero(&mut self, hero: Hero) {
        self.hero = Some(hero);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::Builder;
    use crate::event::{OngoingEachPlayer, OngoingGlobal, PlayerDied};
    use crate::ir::{Event, ExprKind, Program, StmtKind};
    use crate::ty::Num;
    use ow_workshop_data::ValueId;

    fn finish(ws: Builder) -> (Program, Diagnostics) {
        ws.finish(Program::default())
    }

    #[test]
    fn records_event_conditions_and_actions() {
        let mut ws = Builder::new();
        ws.rule("Score on kill", PlayerDied, |r, ev| {
            r.condition(ev.attacker().ne(ev.victim()))
                .comment("no suicides");
            r.condition(true);
            r.actions(|b| b.pass_placeholder());
            r.actions(|b| b.pass_placeholder());
        });
        let (program, diags) = finish(ws);
        assert!(!diags.has_errors());
        let rule = &program.rules[0];
        assert_eq!(rule.name, "Score on kill");
        assert_eq!(rule.event, Event::PlayerDied);
        assert_eq!(rule.conditions.len(), 2);
        assert_eq!(rule.conditions[0].comment.as_deref(), Some("no suicides"));
        assert_eq!(rule.body.stmts.len(), 2);
        assert!(rule.location.is_some_and(|l| l.file.ends_with("rule.rs")));
        assert!(rule.validate().is_ok());
    }

    #[test]
    fn rules_keep_call_order() {
        let mut ws = Builder::new();
        ws.rule("a", OngoingGlobal, |_, _| {});
        ws.rule("b", OngoingEachPlayer, |_, _| {});
        let (program, _) = finish(ws);
        let names: Vec<_> = program.rules.iter().map(|r| r.name.as_str()).collect();
        assert_eq!(names, ["a", "b"]);
        assert_eq!(program.rules[1].event, Event::OngoingEachPlayer);
    }

    #[test]
    fn flags() {
        let mut ws = Builder::new();
        ws.rule("flags", OngoingGlobal, |r, _| {
            r.disabled();
            r.delimiter();
            r.new_page_named("page");
            r.suppress(Warning::BreakOutsideLoop);
            r.comment("note");
        });
        ws.rule("plain page", OngoingGlobal, |r, _| r.new_page());
        let (program, _) = finish(ws);
        let rule = &program.rules[0];
        assert!(rule.disabled && rule.delimiter);
        assert_eq!(rule.new_page.as_deref(), Some("page"));
        assert!(rule.suppressed.0.contains("break-outside-loop"));
        assert_eq!(rule.comment.as_deref(), Some("note"));
        assert_eq!(program.rules[1].new_page.as_deref(), Some(""));
    }

    #[test]
    fn player_filters() {
        let mut ws = Builder::new();
        ws.rule("team", OngoingEachPlayer, |r, _| r.team(TeamFilter::Team2));
        ws.rule("slot", PlayerDied, |r, _| r.slot(3));
        ws.rule("hero", PlayerDied, |r, _| {
            r.team(TeamFilter::Team1);
            r.hero(Hero::Ana);
        });
        ws.rule("none", OngoingEachPlayer, |_, _| {});
        let (program, diags) = finish(ws);
        assert!(!diags.has_errors());
        let filter = |i: usize| program.rules[i].filter;
        assert_eq!(
            filter(0),
            Some(PlayerFilter {
                team: TeamFilter::Team2,
                player: PlayerSlot::All
            })
        );
        assert_eq!(
            filter(1),
            Some(PlayerFilter {
                team: TeamFilter::All,
                player: PlayerSlot::Slot(3)
            })
        );
        assert_eq!(
            filter(2),
            Some(PlayerFilter {
                team: TeamFilter::Team1,
                player: PlayerSlot::Hero(Hero::Ana)
            })
        );
        assert_eq!(filter(3), None);
    }

    #[test]
    fn slot_and_hero_conflict() {
        let mut ws = Builder::new();
        ws.rule("both", PlayerDied, |r, _| {
            r.slot(1);
            r.hero(Hero::Ana);
        });
        let (_, diags) = finish(ws);
        assert!(diags.has_errors());
        assert_eq!(diags.0[0].code, "slot-and-hero");
        assert!(diags.0[0].location.is_some());
    }

    #[test]
    fn slot_out_of_range_is_caught_by_validation() {
        let mut ws = Builder::new();
        ws.rule("far", OngoingEachPlayer, |r, _| r.slot(12));
        let (program, _) = finish(ws);
        assert!(program.validate().is_err());
    }

    #[test]
    fn event_values_record_their_ids() {
        let mut ws = Builder::new();
        ws.rule("values", PlayerDied, |_, ev| {
            let values = [
                ev.event_player().erase(),
                ev.attacker().erase(),
                ev.victim().erase(),
                ev.event_damage().erase(),
                ev.was_environment().erase(),
                ev.was_critical_hit().erase(),
                ev.event_ability().erase(),
                ev.event_direction().erase(),
            ];
            let ids: Vec<_> = values
                .iter()
                .map(|v| match v.ir().kind() {
                    ExprKind::Value { id, args } if args.is_empty() => *id,
                    other => panic!("unexpected {other:?}"),
                })
                .collect();
            assert_eq!(ids[0], ValueId::EventPlayer);
            assert_eq!(ids[1], ValueId::Attacker);
            assert_eq!(ids[3], ValueId::EventDamage);
            assert_eq!(ids[6], ValueId::EventAbility);
        });
        ws.rule("healing", crate::event::PlayerDealtHealing, |_, ev| {
            let _: Expr<Num> = ev.event_healing();
            let _ = (ev.healer(), ev.healee(), ev.was_health_pack());
        });
        ws.rule("knockback", crate::event::PlayerDealtKnockback, |_, ev| {
            let _ = (ev.attacker(), ev.healer(), ev.was_critical_hit());
        });
    }

    #[test]
    fn actions_are_recorded_in_order() {
        let mut ws = Builder::new();
        ws.rule("x", OngoingGlobal, |r, _| {
            r.actions(|b| {
                b.pass_placeholder();
            });
        });
        let (program, _) = finish(ws);
        assert!(matches!(
            program.rules[0].body.stmts[0].kind,
            StmtKind::Pass
        ));
    }
}
