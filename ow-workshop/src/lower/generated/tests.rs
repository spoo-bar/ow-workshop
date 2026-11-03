use ow_workshop_data::{ActionId, ValueId};

use super::*;
use crate::ir::{PlayerSlot, TeamFilter, VarDecl};
use crate::lower::testutil::*;
use crate::options::{CompileOptions, OptLevel};

fn user_rule(name: &str) -> Rule {
    Rule::new(name, Event::OngoingGlobal, block(vec![action()]))
}

fn with_init(name: &str, init: f64) -> VarDecl {
    VarDecl {
        init: Some(num(init)),
        ..VarDecl::new(name)
    }
}

fn names(p: &Program) -> Vec<&str> {
    p.rules.iter().map(|r| r.name.as_str()).collect()
}

fn generate(mut p: Program, options: &CompileOptions) -> Program {
    run(add_rules, &mut p, options).expect("generates");
    p
}

fn base() -> CompileOptions {
    options(OptLevel::Size)
}

#[test]
fn nothing_is_added_when_nothing_is_needed() {
    let p = generate(
        Program {
            rules: vec![user_rule("a")],
            ..Program::default()
        },
        &base(),
    );
    assert_eq!(names(&p), ["a"]);
}

#[test]
fn inspector_rule() {
    let opts = CompileOptions {
        disable_inspector: true,
        ..base()
    };
    let p = generate(
        Program {
            rules: vec![user_rule("a")],
            ..Program::default()
        },
        &opts,
    );
    assert_eq!(names(&p), ["Disable inspector", "a"]);
    let rule = &p.rules[0];
    assert_eq!(rule.event, Event::OngoingGlobal);
    assert_eq!(
        rule.body.stmts[0].kind,
        StmtKind::Action {
            id: ActionId::DisableInspector,
            args: vec![]
        }
    );
}

#[test]
fn global_init_rule_follows_declaration_order() {
    let mut p = Program::default();
    p.globals.declare(with_init("A", 1.0));
    p.globals.declare(VarDecl::new("B"));
    p.globals.declare(with_init("C", 3.0));
    p.rules.push(user_rule("a"));
    let p = generate(p, &base());
    assert_eq!(names(&p), ["Initialize global variables", "a"]);
    let rule = &p.rules[0];
    assert_eq!(rule.event, Event::OngoingGlobal);
    let sets: Vec<_> = rule.body.stmts.iter().map(|s| s.kind.clone()).collect();
    assert_eq!(
        sets,
        [
            StmtKind::Set {
                target: Place::Global(VarId(0)),
                value: num(1.0)
            },
            StmtKind::Set {
                target: Place::Global(VarId(2)),
                value: num(3.0)
            },
        ]
    );
}

#[test]
fn player_init_rule_targets_event_player_of_every_player() {
    let mut p = Program::default();
    p.players.declare(with_init("P", 5.0));
    let opts = CompileOptions {
        player_init_rule_name: "Init players".to_owned(),
        ..base()
    };
    let p = generate(p, &opts);
    assert_eq!(names(&p), ["Init players"]);
    let rule = &p.rules[0];
    assert_eq!(rule.event, Event::OngoingEachPlayer);
    let filter = rule.filter.expect("filter");
    assert_eq!(
        (filter.team, filter.player),
        (TeamFilter::All, PlayerSlot::All)
    );
    let StmtKind::Set { target, value } = &rule.body.stmts[0].kind else {
        panic!("not a set")
    };
    let Place::Player { player, var } = target else {
        panic!("not a player place")
    };
    assert_eq!(*var, VarId(0));
    assert_eq!(
        player.kind(),
        &ExprKind::Value {
            id: ValueId::EventPlayer,
            args: vec![]
        }
    );
    assert_eq!(value.kind(), &ExprKind::Num(5.0));
}

#[test]
fn internal_variables_are_initialized_too() {
    let mut p = Program::default();
    p.globals.declare(VarDecl {
        internal: true,
        ..with_init("__overpyTranslationHelper__", 0.0)
    });
    let p = generate(p, &base());
    assert_eq!(names(&p), ["Initialize global variables"]);
}

#[test]
fn generated_rules_come_in_order_before_user_rules() {
    let mut p = Program::default();
    p.globals.declare(with_init("A", 1.0));
    p.players.declare(with_init("P", 1.0));
    p.rules.push(user_rule("first"));
    p.rules.push(user_rule("second"));
    let opts = CompileOptions {
        disable_inspector: true,
        ..base()
    };
    let p = generate(p, &opts);
    assert_eq!(
        names(&p),
        [
            "Disable inspector",
            "Initialize global variables",
            "Initialize player variables",
            "first",
            "second"
        ]
    );
}

#[test]
fn a_new_page_pads_to_a_multiple_of_one_hundred() {
    let mut p = Program::default();
    for i in 0..3 {
        p.rules.push(user_rule(&format!("r{i}")));
    }
    let mut paged = user_rule("paged");
    paged.new_page = Some("-- page --".to_owned());
    p.rules.push(paged);
    let p = generate(p, &base());
    assert_eq!(p.rules.len(), 3 + 97 + 1);
    let fillers = &p.rules[3..100];
    assert!(fillers.iter().all(|r| r.delimiter
        && !r.disabled
        && r.name == "-- page --"
        && r.event == Event::OngoingGlobal
        && r.body.stmts.is_empty()));
    assert_eq!(p.rules[100].name, "paged");
    assert_eq!(p.rules[100].new_page, None);
}

#[test]
fn an_unnamed_page_break_uses_disabled_fillers() {
    let mut p = Program::default();
    let mut paged = user_rule("paged");
    paged.new_page = Some(String::new());
    p.rules.push(user_rule("a"));
    p.rules.push(paged);
    let p = generate(p, &base());
    assert_eq!(p.rules.len(), 101);
    assert!(p.rules[1..100].iter().all(|r| r.disabled && r.delimiter));
}

#[test]
fn already_on_a_page_boundary_adds_nothing() {
    let mut p = Program::default();
    for i in 0..100 {
        p.rules.push(user_rule(&format!("r{i}")));
    }
    let mut paged = user_rule("paged");
    paged.new_page = Some("page".to_owned());
    p.rules.push(paged);
    assert_eq!(generate(p, &base()).rules.len(), 101);
}

#[test]
fn generated_rules_count_towards_the_page() {
    let mut p = Program::default();
    p.globals.declare(with_init("A", 1.0));
    let mut paged = user_rule("paged");
    paged.new_page = Some("page".to_owned());
    p.rules.push(paged);
    // The init rule is one of the rules so far: 99 fillers.
    let p = generate(p, &base());
    assert_eq!(p.rules.len(), 1 + 99 + 1);
}

#[test]
fn consecutive_page_breaks_each_pad_to_the_next_boundary() {
    let mut p = Program::default();
    for name in ["x", "y"] {
        let mut r = user_rule(name);
        r.new_page = Some("p".to_owned());
        p.rules.push(r);
    }
    let p = generate(p, &base());
    // 100 fillers + x, then 99 fillers + y.
    assert_eq!(p.rules.len(), 100 + 1 + 99 + 1);
    assert_eq!(p.rules[100].name, "x");
    assert_eq!(p.rules[200].name, "y");
}
