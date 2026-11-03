//! Tests for the pass framework and the visitor helpers.

use ow_workshop_data::{ActionId, Lang};

use super::visit::{fold_block, rewrite_block_exprs, rewrite_expr, walk_block};
use super::*;
use crate::ir::{Block, Expr, ExprKind, ModifyOp, Place, Rule, Stmt, VarId};

fn num(n: f64) -> Expr {
    Expr::new(ExprKind::Num(n), None)
}

fn block(kinds: Vec<StmtKind>) -> Block {
    Block {
        stmts: kinds.into_iter().map(Stmt::new).collect(),
        ..Block::default()
    }
}

fn add(a: Expr, b: Expr) -> Expr {
    Expr::new(ExprKind::Add(a, b), None)
}

fn sample_program() -> Program {
    let body = block(vec![
        StmtKind::Modify {
            target: Place::Global(VarId(0)),
            op: ModifyOp::Add,
            value: add(num(1.0), num(2.0)),
        },
        StmtKind::If {
            cond: Expr::new(ExprKind::Bool(true), None),
            then: block(vec![StmtKind::Skip(crate::ir::SkipTarget::Label(LabelId(
                4,
            )))]),
            elifs: vec![],
            else_: None,
        },
    ]);
    Program {
        rules: vec![Rule::new("r", Event::OngoingGlobal, body)],
        ..Program::default()
    }
}

#[test]
fn pipeline_of_noops_leaves_the_program_alone() {
    let mut program = sample_program();
    let before = program.clone();
    let options = CompileOptions::default();
    let mut diags = Diagnostics::default();
    let mut ctx = Ctx::new(&options, Lang::EnUs, &mut diags, &program);
    run_passes(PIPELINE, &mut program, &mut ctx).expect("noop passes succeed");
    assert_eq!(program, before);
    assert_eq!(diags.0.len(), 0);
}

#[test]
fn a_failing_pass_stops_the_pipeline() {
    fn fail(_: &mut Program, _: &mut Ctx<'_>) -> Result<(), Diagnostics> {
        Err(Diagnostics::default())
    }
    #[allow(clippy::unnecessary_wraps)]
    fn mark(program: &mut Program, _: &mut Ctx<'_>) -> Result<(), Diagnostics> {
        program.rules.clear();
        Ok(())
    }
    let mut program = sample_program();
    let options = CompileOptions::default();
    let mut diags = Diagnostics::default();
    let mut ctx = Ctx::new(&options, Lang::EnUs, &mut diags, &program);
    let passes: &[(&str, Pass)] = &[("fail", fail), ("mark", mark)];
    assert!(run_passes(passes, &mut program, &mut ctx).is_err());
    assert_eq!(program.rules.len(), 1);
}

#[test]
fn fresh_labels_skip_the_ones_in_use() {
    let program = sample_program();
    let options = CompileOptions::default();
    let mut diags = Diagnostics::default();
    let mut ctx = Ctx::new(&options, Lang::EnUs, &mut diags, &program);
    assert_eq!(ctx.fresh_label(), LabelId(5));
    assert_eq!(ctx.fresh_label(), LabelId(6));
}

#[test]
fn rule_ctx_applies_block_overrides() {
    use crate::ir::OptOverride;
    let rule = Rule::new("r", Event::OngoingEachPlayer, Block::default());
    let options = CompileOptions::default();
    let top = RuleCtx::new(&rule, &options);
    assert_eq!(
        (top.event, top.opt),
        (Event::OngoingEachPlayer, OptLevel::Speed)
    );
    let off = Block {
        opt: Some(OptOverride {
            enabled: false,
            size: false,
            strict: None,
        }),
        ..Block::default()
    };
    assert_eq!(top.enter(&off).opt, OptLevel::Off);
    let size = Block {
        opt: Some(OptOverride {
            enabled: true,
            size: true,
            strict: None,
        }),
        ..Block::default()
    };
    assert_eq!(top.enter(&size).opt, OptLevel::Size);
    assert_eq!(top.enter(&Block::default()).opt, OptLevel::Speed);
}

#[test]
fn walk_visits_nested_statements_outer_first() {
    let program = sample_program();
    let mut seen = Vec::new();
    walk_block(&program.rules[0].body, &mut |s: &Stmt| {
        seen.push(match s.kind {
            StmtKind::Modify { .. } => "modify",
            StmtKind::If { .. } => "if",
            StmtKind::Skip(_) => "skip",
            _ => "other",
        });
    });
    assert_eq!(seen, ["modify", "if", "skip"]);
}

#[test]
fn rewrite_expr_is_bottom_up() {
    // 1 + (2 + 3): children are folded before their parent sees them.
    let e = add(num(1.0), add(num(2.0), num(3.0)));
    let folded = rewrite_expr(&e, &mut |e| match e.kind() {
        ExprKind::Add(a, b) => match (a.kind(), b.kind()) {
            (ExprKind::Num(a), ExprKind::Num(b)) => num(a + b),
            _ => e,
        },
        _ => e,
    });
    assert_eq!(folded.kind(), &ExprKind::Num(6.0));
}

#[test]
fn rewrite_block_exprs_reaches_nested_blocks() {
    let mut body = block(vec![StmtKind::If {
        cond: Expr::new(ExprKind::Bool(true), None),
        then: block(vec![StmtKind::Action {
            id: ActionId::Wait,
            args: vec![num(1.0)],
        }]),
        elifs: vec![],
        else_: None,
    }]);
    rewrite_block_exprs(&mut body, &mut |e| match e.kind() {
        ExprKind::Num(_) => num(9.0),
        _ => e,
    });
    let StmtKind::If { then, .. } = &body.stmts[0].kind else {
        panic!("not an if")
    };
    let StmtKind::Action { args, .. } = &then.stmts[0].kind else {
        panic!("not an action")
    };
    assert_eq!(args[0].kind(), &ExprKind::Num(9.0));
}

#[test]
fn fold_block_replaces_statements_inside_out() {
    let mut body = block(vec![
        StmtKind::Break,
        StmtKind::While {
            cond: Expr::new(ExprKind::Bool(true), None),
            body: block(vec![StmtKind::Break]),
        },
    ]);
    // Each `Break` becomes two `Pass`es.
    fold_block(&mut body, &mut |stmt| {
        if matches!(stmt.kind, StmtKind::Break) {
            vec![Stmt::new(StmtKind::Pass), Stmt::new(StmtKind::Pass)]
        } else {
            vec![stmt]
        }
    });
    assert_eq!(body.stmts.len(), 3);
    let StmtKind::While { body: inner, .. } = &body.stmts[2].kind else {
        panic!("not a while")
    };
    assert_eq!(inner.stmts.len(), 2);
}
