use ow_workshop_data::{ActionId, Lang, StartRuleBehavior};

use crate::emit::testutil::{global, stmt, stmt_in};
use crate::ir::{ModifyOp, Place, SkipTarget, StmtKind, SubId, VarId};
use crate::lower::testutil::{act, boolean, call, konst, num};
use ow_workshop_data::ValueId;

fn player() -> crate::ir::Expr {
    call(ValueId::EventPlayer, vec![])
}

fn g(n: u32) -> Place {
    Place::Global(VarId(n))
}

fn p(n: u32) -> Place {
    Place::Player {
        player: player(),
        var: VarId(n),
    }
}

fn at(base: Place, index: f64) -> Place {
    Place::Index {
        base: Box::new(base),
        index: num(index),
    }
}

#[test]
fn plain_action() {
    let kind = act(
        ActionId::Wait,
        vec![
            ("time", num(0.25)),
            ("waitBehavior", konst("Wait", "IGNORE_CONDITION")),
        ],
    );
    assert_eq!(stmt(kind), "Wait(0.25, Ignore Condition)");
}

#[test]
fn set_forms() {
    let set = |target, value| StmtKind::Set { target, value };
    assert_eq!(stmt(set(g(0), num(1.0))), "Set Global Variable(A, 1)");
    assert_eq!(
        stmt(set(p(0), num(1.0))),
        "Set Player Variable(Event Player, score, 1)"
    );
    assert_eq!(
        stmt(set(at(g(1), 2.0), num(3.0))),
        "Set Global Variable At Index(foo, 2, 3)"
    );
    assert_eq!(
        stmt(set(at(p(0), 2.0), num(3.0))),
        "Set Player Variable At Index(Event Player, score, 2, 3)"
    );
}

#[test]
fn modify_forms() {
    let modify = |target, op| StmtKind::Modify {
        target,
        op,
        value: num(1.0),
    };
    assert_eq!(
        stmt(modify(g(0), ModifyOp::Add)),
        "Modify Global Variable(A, Add, 1)"
    );
    assert_eq!(
        stmt(modify(p(0), ModifyOp::Subtract)),
        "Modify Player Variable(Event Player, score, Subtract, 1)"
    );
    assert_eq!(
        stmt(modify(at(g(0), 4.0), ModifyOp::AppendToArray)),
        "Modify Global Variable At Index(A, 4, Append To Array, 1)"
    );
    assert_eq!(
        stmt(modify(at(p(0), 4.0), ModifyOp::RaiseToPower)),
        "Modify Player Variable At Index(Event Player, score, 4, Raise To Power, 1)"
    );
}

#[test]
fn japanese_add_uses_plus_equals() {
    let modify = |target, op| StmtKind::Modify {
        target,
        op,
        value: num(2.0),
    };
    assert_eq!(
        stmt_in(Lang::JaJp, modify(g(0), ModifyOp::Add)),
        "グローバル.A += 2"
    );
    assert_eq!(
        stmt_in(Lang::JaJp, modify(p(0), ModifyOp::Add)),
        "(イベント・プレイヤー).score += 2"
    );
    assert!(stmt_in(Lang::JaJp, modify(at(g(0), 1.0), ModifyOp::Add)).ends_with("[1] += 2"));
    // Other operations, including appending, keep the call form.
    assert!(stmt_in(Lang::JaJp, modify(g(0), ModifyOp::AppendToArray)).contains('('));
    // Other languages are unaffected.
    assert_eq!(
        stmt_in(Lang::EnUs, modify(g(0), ModifyOp::Add)),
        "Modify Global Variable(A, Add, 2)"
    );
}

#[test]
fn chase_forms() {
    let chase = |target, over_time| StmtKind::Chase {
        target,
        dest: num(10.0),
        rate_or_duration: num(2.0),
        reeval: konst("ChaseRateReeval", "DESTINATION_AND_RATE"),
        over_time,
    };
    assert!(stmt(chase(g(0), false)).starts_with("Chase Global Variable At Rate(A, 10, 2, "));
    assert!(stmt(chase(g(0), true)).starts_with("Chase Global Variable Over Time(A, 10, 2, "));
    assert!(
        stmt(chase(p(0), false))
            .starts_with("Chase Player Variable At Rate(Event Player, score, 10, 2, ")
    );
    assert!(
        stmt(chase(p(0), true))
            .starts_with("Chase Player Variable Over Time(Event Player, score, 10, 2, ")
    );
}

#[test]
fn stop_chasing_forms() {
    assert_eq!(
        stmt(StmtKind::StopChasing(g(1))),
        "Stop Chasing Global Variable(foo)"
    );
    assert_eq!(
        stmt(StmtKind::StopChasing(p(0))),
        "Stop Chasing Player Variable(Event Player, score)"
    );
}

#[test]
fn skips() {
    assert_eq!(
        stmt(StmtKind::Skip(SkipTarget::Dynamic(num(3.0)))),
        "Skip(3)"
    );
    assert_eq!(
        stmt(StmtKind::SkipIf(
            boolean(true),
            SkipTarget::Dynamic(num(2.0))
        )),
        "Skip If(True, 2)"
    );
}

#[test]
fn control_flow_actions() {
    assert_eq!(stmt(StmtKind::Abort), "Abort");
    assert_eq!(stmt(StmtKind::AbortIf(boolean(true))), "Abort If(True)");
    assert_eq!(
        stmt(StmtKind::AbortIfConditionTrue),
        "Abort If Condition Is True"
    );
    assert_eq!(
        stmt(StmtKind::AbortIfConditionFalse),
        "Abort If Condition Is False"
    );
    assert_eq!(stmt(StmtKind::Loop), "Loop");
    assert_eq!(stmt(StmtKind::LoopIf(boolean(false))), "Loop If(False)");
    assert_eq!(
        stmt(StmtKind::LoopIfConditionTrue),
        "Loop If Condition Is True"
    );
    assert_eq!(
        stmt(StmtKind::LoopIfConditionFalse),
        "Loop If Condition Is False"
    );
    assert_eq!(stmt(StmtKind::SwitchIf), "If(True)");
}

#[test]
fn subroutine_calls_print_raw_names() {
    assert_eq!(
        stmt(StmtKind::CallSubroutine(SubId(1))),
        "Call Subroutine(bar)"
    );
    assert_eq!(
        stmt(StmtKind::StartRule(SubId(0), StartRuleBehavior::Restart)),
        "Start Rule(Sub0, Restart Rule)"
    );
}

#[test]
fn ray_cast_values_use_the_general_form() {
    let e = call(
        ValueId::RaycastHitPosition,
        vec![
            global(0),
            global(1),
            crate::lower::testutil::null(),
            crate::lower::testutil::null(),
            boolean(true),
        ],
    );
    assert!(
        crate::emit::testutil::text(&e)
            .starts_with("Ray Cast Hit Position(Global.A, Global.foo, Null")
    );
}

#[test]
fn for_headers() {
    let print = |place: Place| {
        let program = crate::emit::testutil::program();
        let options = crate::options::CompileOptions::default();
        let cx = crate::emit::Cx {
            lang: Lang::EnUs,
            program: &program,
            options: &options,
        };
        let mut out = String::new();
        super::write_for(&mut out, cx, &place, [&num(0.0), &num(10.0), &num(1.0)]).unwrap();
        out
    };
    assert_eq!(print(g(0)), "For Global Variable(A, 0, 10, 1)");
    assert_eq!(
        print(p(0)),
        "For Player Variable(Event Player, score, 0, 10, 1)"
    );
}
