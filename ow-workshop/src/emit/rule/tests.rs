use ow_workshop_data::{Hero, Lang, ValueId};

use super::write_rules;
use crate::emit::Cx;
use crate::ir::{
    CmpOp, Cond, Event, Expr, ExprKind, OptOverride, PlayerFilter, PlayerSlot, Program, Rule,
    StmtKind, SubDecl, SubId, TeamFilter,
};
use crate::lower::testutil::{act, block, boolean, call, expr, if_, null, num, options};
use crate::options::{CompileOptions, OptLevel};
use ow_workshop_data::ActionId;

fn print(program: &Program, lang: Lang) -> String {
    let options = program.options.clone();
    let mut out = String::new();
    write_rules(
        &mut out,
        Cx {
            lang,
            program,
            options: &options,
        },
        None,
    )
    .unwrap();
    out
}

fn en(rules: Vec<Rule>) -> String {
    print(
        &Program {
            rules,
            ..Program::default()
        },
        Lang::EnUs,
    )
}

fn rule(event: Event, conditions: Vec<Expr>, body: Vec<StmtKind>) -> Rule {
    let mut rule = Rule::new("Name", event, block(body));
    rule.conditions = conditions
        .into_iter()
        .map(|expr| Cond {
            expr,
            comment: None,
        })
        .collect();
    rule
}

fn wait() -> StmtKind {
    act(ActionId::Wait, vec![("time", num(0.25))])
}

fn cmp(op: CmpOp, lhs: Expr, rhs: Expr) -> Expr {
    expr(ExprKind::Compare { op, lhs, rhs })
}

#[test]
fn global_rule() {
    let text = en(vec![rule(Event::OngoingGlobal, vec![], vec![])]);
    assert_eq!(
        text,
        "rule (\"Name\") {\n    event {\n        Ongoing - Global;\n    }\n}\n\n"
    );
}

#[test]
fn each_player_rule_prints_team_and_player() {
    let text = en(vec![rule(Event::OngoingEachPlayer, vec![], vec![])]);
    assert!(text.contains("        Ongoing - Each Player;\n        All;\n        All;\n"));
}

#[test]
fn filters_print_team_slot_and_hero() {
    let mut r = rule(Event::PlayerDied, vec![], vec![]);
    r.filter = Some(PlayerFilter {
        team: TeamFilter::Team2,
        player: PlayerSlot::Slot(3),
    });
    let text = en(vec![r.clone()]);
    assert!(text.contains("        Player Died;\n        Team 2;\n        Slot 3;\n"));
    r.filter = Some(PlayerFilter {
        team: TeamFilter::Team1,
        player: PlayerSlot::Hero(Hero::Ana),
    });
    let text = en(vec![r]);
    assert!(text.contains("        Team 1;\n        Ana;\n"));
}

#[test]
fn every_player_event_prints_its_name() {
    let events = [
        (Event::PlayerEarnedElimination, "Player Earned Elimination"),
        (Event::PlayerDealtFinalBlow, "Player Dealt Final Blow"),
        (Event::PlayerDealtDamage, "Player Dealt Damage"),
        (Event::PlayerTookDamage, "Player Took Damage"),
        (Event::PlayerDied, "Player Died"),
        (Event::PlayerDealtHealing, "Player Dealt Healing"),
        (Event::PlayerReceivedHealing, "Player Received Healing"),
        (Event::PlayerJoinedMatch, "Player Joined Match"),
        (Event::PlayerLeftMatch, "Player Left Match"),
        (Event::PlayerDealtKnockback, "Player Dealt Knockback"),
        (Event::PlayerReceivedKnockback, "Player Received Knockback"),
    ];
    for (event, name) in events {
        let text = en(vec![rule(event, vec![], vec![])]);
        assert!(text.contains(&format!("        {name};\n        All;\n        All;\n")));
    }
}

#[test]
fn subroutine_rule_prints_its_name() {
    let mut program = Program::default();
    let id = program.subroutines.declare(SubDecl::new("bar"));
    assert_eq!(id, SubId(0));
    program.rules = vec![rule(Event::Subroutine(id), vec![], vec![])];
    let text = print(&program, Lang::EnUs);
    assert!(text.contains("        Subroutine;\n        bar;\n"));
}

#[test]
fn disabled_rule_has_a_prefix() {
    let mut r = rule(Event::OngoingGlobal, vec![], vec![]);
    r.disabled = true;
    assert!(en(vec![r]).starts_with("disabled rule (\"Name\") {"));
}

#[test]
fn rule_names_are_escaped_and_censored() {
    let mut r = rule(Event::OngoingGlobal, vec![], vec![]);
    r.name = "Admin \"x\"".to_owned();
    assert!(en(vec![r]).starts_with("rule (\"A\u{AD}dmin \\\"x\\\"\") {"));
}

#[test]
fn rules_are_separated_by_a_blank_line() {
    let r = rule(Event::OngoingGlobal, vec![], vec![]);
    let text = en(vec![r.clone(), r]);
    assert_eq!(text.matches("}\n\n").count(), 2);
}

#[test]
fn conditions_of_each_shape() {
    let alive = call(ValueId::IsAlive, vec![call(ValueId::EventPlayer, vec![])]);
    let conditions = vec![
        cmp(CmpOp::Ge, num(1.0), num(2.0)),
        cmp(CmpOp::Ne, num(1.0), num(2.0)),
        expr(ExprKind::Not(alive.clone())),
        alive,
        boolean(false),
        num(5.0),
    ];
    let text = en(vec![rule(Event::OngoingGlobal, conditions, vec![])]);
    let expected = "    conditions {\n\
        \x20       1 >= 2;\n\
        \x20       1 != 2;\n\
        \x20       Is Alive(Event Player) == False;\n\
        \x20       Is Alive(Event Player) == True;\n\
        \x20       False == True;\n\
        \x20       5 != False;\n\
        \x20   }\n";
    assert!(text.contains(expected), "{text}");
}

#[test]
fn condition_comments_print_above_the_condition() {
    let mut r = rule(
        Event::OngoingGlobal,
        vec![cmp(CmpOp::Eq, num(1.0), num(1.0))],
        vec![],
    );
    r.conditions[0].comment = Some("why \"so\"".to_owned());
    let text = en(vec![r]);
    assert!(text.contains("        \"why \\\"so\\\"\"\n        1 == 1;\n"));
}

#[test]
fn actions_comments_and_nesting() {
    let mut r = rule(
        Event::OngoingGlobal,
        vec![],
        vec![wait(), if_(boolean(true), vec![wait()]), wait()],
    );
    r.body.stmts[0].comment = Some("first".to_owned());
    let text = en(vec![r]);
    let expected = "    actions {\n\
        \x20       \"first\"\n\
        \x20       Wait(0.25, Null);\n\
        \x20       If(True);\n\
        \x20           Wait(0.25, Null);\n\
        \x20       End;\n\
        \x20       Wait(0.25, Null);\n\
        \x20   }\n";
    assert!(text.contains(expected), "{text}");
}

#[test]
fn disabled_action_and_labels_and_comments() {
    let r = rule(
        Event::OngoingGlobal,
        vec![],
        vec![
            StmtKind::Pass,
            StmtKind::Label(crate::ir::LabelId(2)),
            StmtKind::Comment("note".to_owned()),
        ],
    );
    let text = en(vec![r]);
    let expected = "        \"pass\"\n\
        \x20       disabled Abort;\n\
        \x20       //label_2:\n\
        \x20       \"note\"\n";
    assert!(text.contains(expected), "{text}");
}

#[test]
fn keywords_are_localized() {
    let r = rule(Event::OngoingEachPlayer, vec![], vec![wait()]);
    let program = Program {
        rules: vec![r],
        ..Program::default()
    };
    let text = print(&program, Lang::DeDe);
    assert!(text.contains("regel (\"Name\")"), "{text}");
    assert!(text.contains("    aktionen {\n"), "{text}");
}

#[test]
fn optimizer_markers_print_only_deviations_from_the_defaults() {
    let off = Some(OptOverride {
        enabled: false,
        size: false,
        strict: None,
    });
    let mut a = rule(Event::OngoingGlobal, vec![], vec![]);
    a.body.opt = off;
    let b = rule(Event::OngoingGlobal, vec![], vec![]);
    let c = rule(Event::OngoingGlobal, vec![], vec![]);
    let text = en(vec![a, b, c]);
    assert_eq!(text.matches("//Optimizations disabled\n").count(), 1);
    assert_eq!(text.matches("//Optimizations enabled\n").count(), 1);
    let first = text.find("//Optimizations disabled").unwrap();
    let rule_at = text.find("rule (").unwrap();
    assert!(first < rule_at);
}

#[test]
fn non_default_options_print_before_the_first_rule() {
    let program = Program {
        rules: vec![rule(Event::OngoingGlobal, vec![], vec![])],
        options: CompileOptions {
            strict: true,
            ..options(OptLevel::Size)
        },
        ..Program::default()
    };
    let text = print(&program, Lang::EnUs);
    assert!(text.starts_with("//Optimize for size enabled\n//Strict optimizations enabled\nrule"));
}

#[test]
fn block_overrides_print_markers_at_the_block_boundary() {
    let mut inner = block(vec![wait()]);
    inner.opt = Some(OptOverride {
        enabled: false,
        size: false,
        strict: None,
    });
    let r = rule(
        Event::OngoingGlobal,
        vec![],
        vec![
            wait(),
            StmtKind::If {
                cond: boolean(true),
                then: inner,
                elifs: vec![],
                else_: None,
            },
            wait(),
        ],
    );
    let text = en(vec![r]);
    let expected = "        If(True);\n\
        \x20           //Optimizations disabled\n\
        \x20           Wait(0.25, Null);\n\
        \x20       //Optimizations enabled\n\
        \x20       End;\n";
    assert!(text.contains(expected), "{text}");
}
