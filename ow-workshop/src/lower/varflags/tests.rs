use super::*;
use crate::ir::{Cond, Event, Rule, VarDecl};
use crate::lower::testutil::*;
use crate::options::OptLevel;

const AT: Location = Location {
    file: "mod.rs",
    line: 7,
    column: 3,
};

fn global(id: u32) -> Place {
    Place::Global(VarId(id))
}

fn chase(id: u32, dest: f64, over_time: bool) -> StmtKind {
    StmtKind::Chase {
        target: global(id),
        dest: num(dest),
        rate_or_duration: num(1.0),
        reeval: num(0.0),
        over_time,
    }
}

fn for_loop(id: u32) -> StmtKind {
    StmtKind::For {
        var: global(id),
        start: num(0.0),
        stop: num(3.0),
        step: num(1.0),
        body: block(vec![]),
    }
}

fn program_with(rules: Vec<Rule>) -> Program {
    let mut p = Program {
        rules,
        ..Program::default()
    };
    p.globals.declare(VarDecl::new("X"));
    p.globals.declare(VarDecl::new("Y"));
    p.players.declare(VarDecl::new("P"));
    p
}

fn rule(body: Vec<StmtKind>) -> Rule {
    let mut r = Rule::new("r", Event::OngoingGlobal, block(body));
    // Warnings point at the chase.
    for s in &mut r.body.stmts {
        s.loc = Some(AT);
    }
    r
}

fn flags_and_warnings(rules: Vec<Rule>) -> (Program, Diagnostics) {
    let mut p = program_with(rules);
    let w = run(collect, &mut p, &options(OptLevel::Size)).expect("collects");
    (p, w)
}

#[test]
fn chased_variable_used_in_a_for_loop() {
    let (p, w) = flags_and_warnings(vec![rule(vec![chase(0, 5.0, true), for_loop(0)])]);
    assert_eq!(codes(&w), ["chased-var-in-for"]);
    assert_eq!(
        w.0[0].message,
        "The variable X is chased and used in a for loop, so the loop will not run."
    );
    assert_eq!(w.0[0].location, Some(AT));
    let flags = p.globals.entries[0].flags;
    assert!(flags.chased && flags.for_loop && !flags.in_condition);
    assert!(!p.globals.entries[1].flags.chased);
}

#[test]
fn statement_order_does_not_matter() {
    let (_, a) = flags_and_warnings(vec![rule(vec![chase(0, 5.0, true), for_loop(0)])]);
    let (_, b) = flags_and_warnings(vec![rule(vec![for_loop(0), chase(0, 5.0, true)])]);
    assert_eq!(codes(&a), codes(&b));
    // The loop and the chase may sit in different rules, in either order.
    let (_, c) = flags_and_warnings(vec![
        rule(vec![for_loop(0)]),
        rule(vec![chase(0, 5.0, true)]),
    ]);
    assert_eq!(codes(&c), ["chased-var-in-for"]);
}

#[test]
fn a_chase_in_a_disabled_rule_still_counts() {
    let mut chased = rule(vec![chase(0, 5.0, false)]);
    chased.disabled = true;
    let (_, w) = flags_and_warnings(vec![rule(vec![for_loop(0)]), chased]);
    assert_eq!(codes(&w), ["chased-var-in-for"]);
}

#[test]
fn chased_variable_read_in_a_rule_condition() {
    let mut reader = rule(vec![]);
    reader.conditions.push(Cond {
        expr: expr(ExprKind::Compare {
            op: crate::ir::CmpOp::Gt,
            lhs: expr(ExprKind::GlobalVar(VarId(0))),
            rhs: num(1.0),
        }),
        comment: None,
    });
    let (p, w) = flags_and_warnings(vec![reader, rule(vec![chase(0, 5.0, true)])]);
    assert_eq!(codes(&w), ["ow2-rule-condition-chase"]);
    assert!(p.globals.entries[0].flags.in_condition);
}

#[test]
fn chase_towards_9999_at_a_rate() {
    let (_, w) = flags_and_warnings(vec![rule(vec![chase(1, 9999.0, false)])]);
    assert_eq!(codes(&w), ["chase-9999"]);
    // Over time the destination is not a duration problem.
    let (_, w) = flags_and_warnings(vec![rule(vec![chase(1, 9999.0, true)])]);
    assert_eq!(w.0.len(), 0);
    let (_, w) = flags_and_warnings(vec![rule(vec![chase(1, 99999.0, false)])]);
    assert_eq!(w.0.len(), 0);
}

#[test]
fn each_warning_fires_once_per_variable() {
    let (_, w) = flags_and_warnings(vec![rule(vec![
        chase(0, 9999.0, false),
        chase(0, 9999.0, false),
        for_loop(0),
        for_loop(0),
    ])]);
    assert_eq!(codes(&w), ["chased-var-in-for", "chase-9999"]);
}

#[test]
fn player_and_global_variables_are_separate() {
    let player_chase = StmtKind::Chase {
        target: Place::Player {
            player: expr(ExprKind::Null),
            var: VarId(0),
        },
        dest: num(1.0),
        rate_or_duration: num(1.0),
        reeval: num(0.0),
        over_time: true,
    };
    // Global X is looped over; player P (same id) is chased.
    let (p, w) = flags_and_warnings(vec![rule(vec![player_chase, for_loop(0)])]);
    assert_eq!(w.0.len(), 0);
    assert!(p.players.entries[0].flags.chased);
    assert!(!p.globals.entries[0].flags.chased);
}

#[test]
fn suppressed_per_rule() {
    let mut chased = rule(vec![chase(0, 5.0, true), for_loop(0)]);
    chased.suppressed.insert(Warning::ChasedVarInFor);
    let (_, w) = flags_and_warnings(vec![chased]);
    assert_eq!(w.0.len(), 0);
}
