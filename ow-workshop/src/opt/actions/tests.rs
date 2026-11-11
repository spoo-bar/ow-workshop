use ow_workshop_data::ActionId;

use crate::ir::{Program, StmtKind};
use crate::lower::testutil::*;
use crate::options::OptLevel;

fn optimized(body: Vec<StmtKind>, level: OptLevel) -> Program {
    let mut p = program(body);
    run(crate::opt::run, &mut p, &options(level)).expect("optimizes");
    p
}

fn arg(p: &Program, at: usize, i: usize) -> &crate::ir::Expr {
    let StmtKind::Action { args, .. } = &p.rules[0].body.stmts[at].kind else {
        panic!("not an action")
    };
    &args[i]
}

#[test]
fn zero_score_and_zero_duration_status_are_removed() {
    let p = optimized(
        vec![
            act(ActionId::AddToScore, vec![("score", num(0.0))]),
            act(ActionId::SetStatusEffect, vec![("duration", num(0.0))]),
            act(ActionId::AddToScore, vec![("score", num(2.0))]),
            act(ActionId::SetStatusEffect, vec![("duration", num(1.0))]),
        ],
        OptLevel::Speed,
    );
    assert_eq!(kinds(&p).len(), 2);
}

#[test]
fn nothing_is_removed_next_to_a_dynamic_skip() {
    let dynamic = StmtKind::Skip(crate::ir::SkipTarget::Dynamic(num(2.0)));
    let p = optimized(
        vec![
            dynamic,
            act(ActionId::AddToScore, vec![("score", num(0.0))]),
        ],
        OptLevel::Speed,
    );
    assert_eq!(kinds(&p).len(), 2);
}

#[test]
fn a_hidden_outline_loses_its_color() {
    let color = konst("Color", "RED");
    let p = optimized(
        vec![act(
            ActionId::StartForcingOutlineFor,
            vec![("visible", boolean(false)), ("color", color.clone())],
        )],
        OptLevel::Speed,
    );
    assert!(matches!(arg(&p, 0, 3).kind(), crate::ir::ExprKind::Null));
    let shown = optimized(
        vec![act(
            ActionId::StartForcingOutlineFor,
            vec![("visible", boolean(true)), ("color", color.clone())],
        )],
        OptLevel::Speed,
    );
    assert_eq!(arg(&shown, 0, 3), &color);
}
