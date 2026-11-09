use ow_workshop_data::{ActionId, StartRuleBehavior, ValueId};

use super::*;
use crate::ir::{Rule, SubDecl};
use crate::lower::testutil::*;
use crate::options::OptLevel;

fn value(id: ValueId) -> StmtKind {
    StmtKind::Action {
        id: ActionId::Teleport,
        args: vec![expr(ExprKind::Value { id, args: vec![] })],
    }
}

fn wait() -> StmtKind {
    action()
}

fn sub(id: u32, body: Vec<StmtKind>) -> Rule {
    Rule::new(
        format!("sub{id}"),
        Event::Subroutine(SubId(id)),
        block(body),
    )
}

fn caller(event: Event, body: Vec<StmtKind>) -> Rule {
    Rule::new("caller", event, block(body))
}

fn call(id: u32) -> StmtKind {
    StmtKind::CallSubroutine(SubId(id))
}

fn start(id: u32, behavior: StartRuleBehavior) -> StmtKind {
    StmtKind::StartRule(SubId(id), behavior)
}

fn check_program(subs: u32, rules: Vec<Rule>) -> Diagnostics {
    let mut p = Program {
        rules,
        ..Program::default()
    };
    for i in 0..subs {
        p.subroutines.declare(SubDecl::new(format!("sub{i}")));
    }
    locate(&mut p);
    run(check, &mut p, &options(OptLevel::Size)).expect("checks")
}

#[test]
fn player_values_need_a_player_event() {
    let w = check_program(
        1,
        vec![
            sub(0, vec![value(ValueId::EventPlayer)]),
            caller(Event::OngoingGlobal, vec![call(0)]),
        ],
    );
    assert_eq!(codes(&w), ["mismatched-subroutine-event"]);
    assert_eq!(
        w.0[0].message,
        "Calling subroutine sub0, which uses event player values, from a Ongoing - Global rule"
    );
}

#[test]
fn damage_or_healing_values() {
    let w = check_program(
        1,
        vec![
            sub(0, vec![value(ValueId::EventAbility)]),
            caller(Event::OngoingEachPlayer, vec![call(0)]),
        ],
    );
    assert!(w.0[0].message.contains("event damage or healing values"));
}

#[test]
fn damage_values() {
    // The caller has damage-or-healing (healing event) but not damage.
    let w = check_program(
        1,
        vec![
            sub(0, vec![value(ValueId::EventDamage)]),
            caller(Event::PlayerDealtHealing, vec![call(0)]),
        ],
    );
    assert_eq!(w.0.len(), 1);
    assert!(w.0[0].message.contains("event damage values"));
}

#[test]
fn healing_values() {
    let w = check_program(
        1,
        vec![
            sub(0, vec![value(ValueId::EventHealing)]),
            caller(Event::PlayerDied, vec![call(0)]),
        ],
    );
    assert!(w.0[0].message.contains("event healing values"));
}

#[test]
fn the_first_matching_check_wins() {
    let w = check_program(
        1,
        vec![
            sub(
                0,
                vec![value(ValueId::EventPlayer), value(ValueId::EventDamage)],
            ),
            caller(Event::OngoingGlobal, vec![call(0)]),
        ],
    );
    assert_eq!(w.0.len(), 1);
    assert!(w.0[0].message.contains("event player values"));
}

#[test]
fn matching_events_are_quiet() {
    let w = check_program(
        1,
        vec![
            sub(
                0,
                vec![
                    value(ValueId::EventPlayer),
                    value(ValueId::Attacker),
                    value(ValueId::EventWasCriticalHit),
                ],
            ),
            caller(Event::PlayerTookDamage, vec![call(0)]),
            caller(Event::PlayerDealtKnockback, vec![call(0)]),
        ],
    );
    assert_eq!(w.0.len(), 0);
}

#[test]
fn needs_propagate_through_calls_and_start_rule() {
    // sub0 -> sub1 -> sub2 (reads the damage), and sub0 -> start sub3 (reads the player).
    let w = check_program(
        4,
        vec![
            sub(0, vec![call(1)]),
            sub(1, vec![call(2)]),
            sub(2, vec![value(ValueId::EventDamage)]),
            sub(3, vec![value(ValueId::EventPlayer)]),
            caller(Event::OngoingEachPlayer, vec![call(0)]),
            caller(
                Event::OngoingGlobal,
                vec![start(3, StartRuleBehavior::Noop)],
            ),
        ],
    );
    assert_eq!(codes(&w), ["mismatched-subroutine-event"; 2]);
    assert!(w.0[0].message.contains("sub0") && w.0[0].message.contains("damage values"));
    assert!(w.0[1].message.contains("sub3") && w.0[1].message.contains("player values"));
}

#[test]
fn recursion_converges() {
    let w = check_program(
        2,
        vec![
            sub(0, vec![call(1), value(ValueId::EventPlayer)]),
            sub(1, vec![call(0)]),
            caller(Event::OngoingGlobal, vec![call(1)]),
        ],
    );
    assert_eq!(w.0.len(), 1);
}

#[test]
fn restarting_a_waiting_subroutine_crashes_the_server() {
    // foo -> Start Rule(nested, Restart); nested -> sub_with_wait, which waits.
    let w = check_program(
        2,
        vec![
            sub(0, vec![call(1)]),
            sub(1, vec![wait()]),
            caller(
                Event::OngoingGlobal,
                vec![start(0, StartRuleBehavior::Restart)],
            ),
        ],
    );
    assert_eq!(codes(&w), ["start-rule-crash"]);
}

#[test]
fn only_restarting_calls_crash() {
    let w = check_program(
        1,
        vec![
            sub(0, vec![wait()]),
            caller(
                Event::OngoingGlobal,
                vec![call(0), start(0, StartRuleBehavior::Noop)],
            ),
        ],
    );
    assert_eq!(w.0.len(), 0);
}

#[test]
fn calls_inside_subroutines_are_not_checked() {
    let w = check_program(
        2,
        vec![
            sub(0, vec![value(ValueId::EventPlayer)]),
            sub(1, vec![call(0)]),
        ],
    );
    assert_eq!(w.0.len(), 0);
}

#[test]
fn warnings_can_be_suppressed_per_rule() {
    let mut rule = caller(Event::OngoingGlobal, vec![call(0)]);
    rule.suppressed.insert(Warning::MismatchedSubroutineEvent);
    let w = check_program(1, vec![sub(0, vec![value(ValueId::EventPlayer)]), rule]);
    assert_eq!(w.0.len(), 0);
}
