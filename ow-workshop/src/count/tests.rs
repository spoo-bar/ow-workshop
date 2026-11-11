use ow_workshop_data::{ActionId, Lang, ValueId};

use super::{ELEMENT_LIMIT, count, count_rule};
use crate::emit::emit;
use crate::ir::{
    CmpOp, Cond, Event, Expr, ExprKind, ModifyOp, Place, Program, Rule, StmtKind, StrExpr,
    StrToken, VarDecl, VarId,
};
use crate::lower::testutil::{act, block, boolean, call, expr, if_, konst, null, num, options};
use crate::options::{CompileOptions, OptLevel};

fn set(var: u32, value: Expr) -> StmtKind {
    StmtKind::Set {
        target: Place::Global(VarId(var)),
        value,
    }
}

fn rule_with(conditions: Vec<Expr>, body: Vec<StmtKind>) -> Rule {
    let mut rule = Rule::new("r", Event::OngoingGlobal, block(body));
    rule.conditions = conditions
        .into_iter()
        .map(|expr| Cond {
            expr,
            comment: None,
        })
        .collect();
    rule
}

fn total(rule: &Rule) -> usize {
    count_rule(rule, &options(OptLevel::Speed), Lang::EnUs).total
}

fn expr_count(e: &Expr) -> usize {
    let rule = rule_with(vec![], vec![act(ActionId::Wait, vec![("time", e.clone())])]);
    // `Wait(e, Null)` is 1 + (e + 1) - 2 = e, plus the rule.
    total(&rule) - 1
}

#[test]
fn a_one_action_rule_is_three_elements() {
    let rule = rule_with(vec![], vec![set(0, num(1.0))]);
    assert_eq!(total(&rule), 3);
}

#[test]
fn conditions() {
    let alive = call(ValueId::IsAlive, vec![call(ValueId::EventPlayer, vec![])]);
    let cmp = expr(ExprKind::Compare {
        op: CmpOp::Eq,
        lhs: num(1.0),
        rhs: num(2.0),
    });
    let rule = rule_with(vec![cmp, alive.clone(), expr(ExprKind::Not(alive))], vec![]);
    let counted = count_rule(&rule, &options(OptLevel::Speed), Lang::EnUs);
    // `1 == 2` is 2 + 2 - 1; `Is Alive(Event Player)` is 2; `Not(..)` prints as `x == False`.
    assert_eq!(counted.conditions, [3, 2, 2]);
    assert_eq!(counted.total, 1 + 3 + 2 + 2);
}

#[test]
fn leaves() {
    assert_eq!(expr_count(&num(5.0)), 2);
    assert_eq!(expr_count(&boolean(true)), 1);
    assert_eq!(expr_count(&null()), 1);
    assert_eq!(expr_count(&call(ValueId::EventPlayer, vec![])), 1);
    assert_eq!(expr_count(&konst("Wait", "IGNORE_CONDITION")), 1);
}

#[test]
fn wrapped_constants_and_teams_cost_two() {
    assert_eq!(expr_count(&konst("Hero", "ana")), 2);
    assert_eq!(expr_count(&konst("Color", "WHITE")), 2);
    assert_eq!(expr_count(&konst("Button", "INTERACT")), 2);
    assert_eq!(expr_count(&konst("Map", "hanamura")), 2);
    assert_eq!(expr_count(&konst("Team", "1")), 2);
}

#[test]
fn variable_reads() {
    let global = expr(ExprKind::GlobalVar(VarId(0)));
    assert_eq!(expr_count(&global), 2);
    let player = call(ValueId::EventPlayer, vec![]);
    let read = expr(ExprKind::PlayerVar {
        player,
        var: VarId(0),
    });
    assert_eq!(expr_count(&read), 3);
}

#[test]
fn values_with_arguments() {
    let e = call(ValueId::IsAlive, vec![call(ValueId::EventPlayer, vec![])]);
    assert_eq!(expr_count(&e), 2);
    assert_eq!(expr_count(&expr(ExprKind::Add(num(1.0), num(2.0)))), 5);
    assert_eq!(
        expr_count(&expr(ExprKind::Compare {
            op: CmpOp::Lt,
            lhs: num(1.0),
            rhs: num(2.0),
        })),
        6
    );
}

#[test]
fn custom_strings_always_have_three_slots() {
    let no_args = expr(ExprKind::Str(StrExpr::Custom {
        tokens: vec![StrToken::Text("x".to_owned())],
        args: vec![],
    }));
    assert_eq!(expr_count(&no_args), 2 + 3);
    let one = expr(ExprKind::Str(StrExpr::Custom {
        tokens: vec![StrToken::Arg(0)],
        args: vec![num(1.0)],
    }));
    assert_eq!(expr_count(&one), 2 + 2 + 2);
}

#[test]
fn localized_strings() {
    let e = expr(ExprKind::Str(StrExpr::Localized(
        ow_workshop_data::LocalizedString::Empty,
    )));
    assert_eq!(expr_count(&e), 6);
}

#[test]
fn arrays() {
    assert_eq!(expr_count(&expr(ExprKind::Array(vec![]))), 1);
    let array = expr(ExprKind::Array(vec![num(1.0), num(2.0)]));
    assert_eq!(expr_count(&array), 2 + 2 + 2);
}

#[test]
fn evaluate_once_costs_one_more() {
    let e = call(ValueId::EvalOnce, vec![num(1.0)]);
    assert_eq!(expr_count(&e), 1 + 1 + 2);
}

#[test]
fn hero_literals_add_one_per_pair_in_a_slot() {
    let heroes = |n: usize| {
        expr(ExprKind::Array(
            (0..n).map(|_| konst("Hero", "ana")).collect(),
        ))
    };
    // Array(h, h): 2 + 2 + 2, plus floor(2 / 2).
    assert_eq!(expr_count(&heroes(2)), 6 + 1);
    assert_eq!(expr_count(&heroes(3)), 8 + 1);
    assert_eq!(expr_count(&heroes(1)), 4);
    // Two slots with one hero each do not pair up.
    let rule = rule_with(
        vec![],
        vec![act(
            ActionId::Wait,
            vec![
                ("time", konst("Hero", "ana")),
                ("waitBehavior", konst("Hero", "ana")),
            ],
        )],
    );
    assert_eq!(total(&rule), 1 + (1 + 2 + 2 - 2));
}

#[test]
fn workshop_setting_values_are_discounted() {
    let args = || vec![num(1.0), num(2.0), num(3.0)];
    let integer = call(ValueId::CreateWorkshopSettingInt, args());
    assert_eq!(expr_count(&integer), 1 + 6 - 3);
    let real = call(ValueId::CreateWorkshopSettingFloat, args());
    assert_eq!(expr_count(&real), 1 + 6 - 3);
    let combo = call(ValueId::CreateWorkshopSettingEnum, args());
    assert_eq!(expr_count(&combo), 1 + 6 - 2);
    let toggle = call(ValueId::CreateWorkshopSettingBool, args());
    assert_eq!(expr_count(&toggle), 1 + 6);
}

#[test]
fn flat_control_flow_lines_count_one_each() {
    // If(True) = 1; Wait = 1 + (2 + 1) - 2 = 2; End = 1.
    let rule = rule_with(
        vec![],
        vec![
            if_(
                boolean(true),
                vec![act(ActionId::Wait, vec![("time", num(1.0))])],
            ),
            StmtKind::Abort,
        ],
    );
    assert_eq!(total(&rule), 1 + 1 + 2 + 1 + 1);
}

#[test]
fn labels_comments_and_passes() {
    let rule = rule_with(
        vec![],
        vec![
            StmtKind::Label(crate::ir::LabelId(0)),
            StmtKind::Comment("x".to_owned()),
            StmtKind::Pass,
        ],
    );
    // Only the rule and the `disabled Abort`.
    assert_eq!(total(&rule), 2);
}

#[test]
fn japanese_add_is_cheaper() {
    let modify = |op| StmtKind::Modify {
        target: Place::Global(VarId(0)),
        op,
        value: num(1.0),
    };
    let rule = rule_with(vec![], vec![modify(ModifyOp::Add)]);
    let en = count_rule(&rule, &options(OptLevel::Speed), Lang::EnUs).total;
    let ja = count_rule(&rule, &options(OptLevel::Speed), Lang::JaJp).total;
    assert_eq!(en - ja, 2);
    let append = rule_with(vec![], vec![modify(ModifyOp::AppendToArray)]);
    let en = count_rule(&append, &options(OptLevel::Speed), Lang::EnUs).total;
    let ja = count_rule(&append, &options(OptLevel::Speed), Lang::JaJp).total;
    assert_eq!(en, ja);
}

#[test]
fn program_total_is_the_sum_of_rules() {
    let program = Program {
        rules: vec![
            rule_with(vec![], vec![set(0, num(1.0))]),
            rule_with(vec![], vec![]),
        ],
        ..Program::default()
    };
    let counted = count(&program, Lang::EnUs);
    assert_eq!(counted.total, 4);
    assert!(!counted.over_limit());
}

fn two_rule_program() -> Program {
    let mut program = Program::default();
    program.globals.declare(VarDecl::new("A"));
    program.globals.allocate().unwrap();
    let mut main = rule_with(
        vec![expr(ExprKind::Compare {
            op: CmpOp::Eq,
            lhs: num(1.0),
            rhs: num(2.0),
        })],
        vec![set(0, num(1.0))],
    );
    main.name = "big".to_owned();
    let mut small = rule_with(vec![], vec![]);
    small.name = "small".to_owned();
    main.location = None;
    small.location = None;
    program.rules = vec![small, main];
    program.options = CompileOptions {
        debug_element_count: true,
        ..CompileOptions::default()
    };
    program
}

#[test]
fn debug_output_annotates_rules_conditions_and_actions() {
    let program = two_rule_program();
    let counts = count(&program, Lang::EnUs);
    let text = emit(&program, Lang::EnUs, Some(&counts)).unwrap();
    assert!(text.contains("//1 element\nrule (\"small\")"), "{text}");
    assert!(text.contains("//6 elements\nrule (\"big\")"), "{text}");
    assert!(text.contains("        1 == 2; // 3 elements\n"), "{text}");
    assert!(
        text.contains("        Set Global Variable(A, 1); // 2 elements\n"),
        "{text}"
    );
}

#[test]
fn summary_lists_rules_over_one_element_largest_first() {
    let program = two_rule_program();
    let counts = count(&program, Lang::EnUs);
    let text = emit(&program, Lang::EnUs, Some(&counts)).unwrap();
    assert!(
        text.starts_with("/* Element count: (total 7)\n\n    6: rule \"big\""),
        "{text}"
    );
    assert!(!text.split("*/").next().unwrap().contains("small"));
    assert!(text.contains("\n\n*/\n\n"));
}

#[test]
fn summary_names_the_source_module() {
    let mut program = two_rule_program();
    program.rules[1].location = Some(crate::ir::Location {
        file: "src/effects.rs",
        line: 1,
        column: 1,
    });
    let counts = count(&program, Lang::EnUs);
    let text = emit(&program, Lang::EnUs, Some(&counts)).unwrap();
    assert!(
        text.contains("    6: rule \"big\" (src/effects)\n"),
        "{text}"
    );
}

#[test]
fn over_the_limit() {
    let sets = ELEMENT_LIMIT / 2 + 1;
    let program = Program {
        rules: vec![rule_with(
            vec![],
            (0..sets).map(|_| set(0, num(1.0))).collect(),
        )],
        ..Program::default()
    };
    assert!(count(&program, Lang::EnUs).over_limit());
}
