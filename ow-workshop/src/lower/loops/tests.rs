use super::*;
use crate::ir::{Event, Rule};
use crate::lower::testutil::*;
use crate::options::{CompileOptions, OptLevel};

fn while_(body: Vec<StmtKind>) -> StmtKind {
    StmtKind::While {
        cond: boolean(true),
        body: block(body),
    }
}

fn do_while(cond: Expr, body: Vec<StmtKind>) -> StmtKind {
    StmtKind::DoWhile {
        cond,
        body: block(body),
    }
}

fn lowered(body: Vec<StmtKind>) -> (Program, Diagnostics) {
    let mut p = program(body);
    let warnings = run(lower, &mut p, &options(OptLevel::Size)).expect("lowers");
    (p, warnings)
}

fn body_of(kind: &StmtKind) -> Vec<&StmtKind> {
    let StmtKind::While { body, .. } = kind else {
        panic!("not a while")
    };
    body.stmts.iter().map(|s| &s.kind).collect()
}

fn label_of(kind: &StmtKind) -> LabelId {
    match kind {
        StmtKind::Skip(SkipTarget::Label(l)) | StmtKind::Label(l) => *l,
        other => panic!("no label in {other:?}"),
    }
}

#[test]
fn break_in_a_while_stays_a_workshop_break() {
    let (p, w) = lowered(vec![while_(vec![if_(
        boolean(true),
        vec![StmtKind::Break],
    )])]);
    assert_eq!(w.0.len(), 0);
    let StmtKind::While { body, .. } = kinds(&p)[0] else {
        panic!()
    };
    let StmtKind::If { then, .. } = &body.stmts[0].kind else {
        panic!()
    };
    assert_eq!(then.stmts[0].kind, StmtKind::Break);
}

#[test]
fn continue_skips_to_a_label_at_the_end_of_the_body() {
    let (p, _) = lowered(vec![while_(vec![
        action(),
        if_(boolean(true), vec![StmtKind::Continue]),
        action(),
    ])]);
    let StmtKind::While { body, .. } = kinds(&p)[0] else {
        panic!()
    };
    let StmtKind::If { then, .. } = &body.stmts[1].kind else {
        panic!()
    };
    let target = label_of(&then.stmts[0].kind);
    assert_eq!(body.stmts.len(), 4);
    assert_eq!(label_of(&body.stmts[3].kind), target);
}

#[test]
fn one_continue_label_serves_the_whole_loop() {
    let (p, _) = lowered(vec![while_(vec![StmtKind::Continue, StmtKind::Continue])]);
    let k = body_of(kinds(&p)[0]);
    assert_eq!(k.len(), 3);
    assert_eq!(label_of(k[0]), label_of(k[1]));
}

#[test]
fn break_in_a_do_while_skips_past_the_final_loop_if() {
    let cond = expr(ExprKind::GlobalVar(crate::ir::VarId(0)));
    let (p, _) = lowered(vec![do_while(
        cond.clone(),
        vec![action(), StmtKind::Break],
    )]);
    let k = kinds(&p);
    assert_eq!(k.len(), 4);
    assert_eq!(k[2], &StmtKind::LoopIf(cond));
    assert_eq!(label_of(k[1]), label_of(k[3]));
}

#[test]
fn continue_in_a_do_while_rechecks_the_condition() {
    let cond = expr(ExprKind::GlobalVar(crate::ir::VarId(0)));
    let (p, _) = lowered(vec![do_while(
        cond.clone(),
        vec![StmtKind::Continue, action()],
    )]);
    let k = kinds(&p);
    // Loop If(cond); Skip(exit) -- when the condition fails the loop is left, not continued.
    assert_eq!(k[0], &StmtKind::LoopIf(cond.clone()));
    assert!(matches!(k[1], StmtKind::Skip(SkipTarget::Label(_))));
    assert_eq!(k[3], &StmtKind::LoopIf(cond));
    assert_eq!(label_of(k[1]), label_of(k[4]));
}

#[test]
fn do_while_tail_depends_on_the_condition() {
    let tail = |cond: Expr| {
        let (p, _) = lowered(vec![do_while(cond, vec![action()])]);
        kinds(&p).into_iter().skip(1).cloned().collect::<Vec<_>>()
    };
    assert_eq!(tail(boolean(true)), [StmtKind::Loop]);
    assert_eq!(tail(num(2.0)), [StmtKind::Loop]);
    assert_eq!(tail(boolean(false)), []);
    assert_eq!(tail(num(0.0)), []);
    assert_eq!(
        tail(expr(ExprKind::RuleCondition)),
        [StmtKind::LoopIfConditionTrue]
    );
    assert_eq!(
        tail(expr(ExprKind::Not(expr(ExprKind::RuleCondition)))),
        [StmtKind::LoopIfConditionFalse]
    );
    let c = expr(ExprKind::GlobalVar(crate::ir::VarId(1)));
    assert_eq!(tail(c.clone()), [StmtKind::LoopIf(c)]);
}

#[test]
fn continue_in_a_do_while_that_always_loops_is_just_loop() {
    let (p, _) = lowered(vec![do_while(boolean(true), vec![StmtKind::Continue])]);
    assert_eq!(kinds(&p), [&StmtKind::Loop, &StmtKind::Loop]);
}

#[test]
fn continue_in_a_do_while_that_never_loops_leaves_it() {
    let (p, _) = lowered(vec![do_while(
        boolean(false),
        vec![StmtKind::Continue, action()],
    )]);
    let k = kinds(&p);
    assert_eq!(k.len(), 3);
    assert_eq!(label_of(k[0]), label_of(k[2]));
}

#[test]
fn break_and_continue_outside_a_loop_abort_with_a_warning() {
    let (p, w) = lowered(vec![StmtKind::Break, StmtKind::Continue]);
    assert_eq!(kinds(&p), [&StmtKind::Abort, &StmtKind::Abort]);
    assert_eq!(codes(&w), ["break-outside-loop", "continue-outside-loop"]);
}

#[test]
fn suppressed_warnings_stay_quiet() {
    let mut p = program(vec![StmtKind::Break]);
    p.rules[0].suppressed.insert(Warning::BreakOutsideLoop);
    locate(&mut p);
    let w = run(lower, &mut p, &options(OptLevel::Size)).expect("lowers");
    assert_eq!(kinds(&p), [&StmtKind::Abort]);
    assert_eq!(w.0.len(), 0);

    let mut p = program(vec![StmtKind::Continue]);
    let mut opts = CompileOptions::default();
    opts.suppress.insert(Warning::ContinueOutsideLoop);
    let w = run(lower, &mut p, &opts).expect("lowers");
    assert_eq!(w.0.len(), 0);
}

#[test]
fn a_do_while_must_lead_the_rule() {
    let mut p = program(vec![action(), do_while(boolean(true), vec![])]);
    let err = run(lower, &mut p, &options(OptLevel::Size)).expect_err("misplaced");
    assert_eq!(codes(&err), ["misplaced-do-while"]);

    let mut p = program(vec![while_(vec![do_while(boolean(true), vec![])])]);
    let err = run(lower, &mut p, &options(OptLevel::Size)).expect_err("nested");
    assert_eq!(codes(&err), ["misplaced-do-while"]);

    // Comments and `Pass` may precede it.
    let (p, _) = lowered(vec![
        StmtKind::Pass,
        StmtKind::Comment("c".into()),
        do_while(boolean(true), vec![action()]),
    ]);
    assert_eq!(p.rules[0].body.stmts.len(), 4);
}

#[test]
fn each_rule_is_lowered_with_its_own_loop_stack() {
    let mut p = program(vec![while_(vec![StmtKind::Break])]);
    p.rules.push(Rule::new(
        "second",
        Event::OngoingGlobal,
        block(vec![StmtKind::Break]),
    ));
    let w = run(lower, &mut p, &options(OptLevel::Size)).expect("lowers");
    assert_eq!(codes(&w), ["break-outside-loop"]);
    assert_eq!(p.rules[1].body.stmts[0].kind, StmtKind::Abort);
}
