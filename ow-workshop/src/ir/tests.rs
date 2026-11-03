//! Hand-built programs exercising the IR.

use ow_workshop_data::{ActionId, StartRuleBehavior, ValueId};

use super::*;

fn num(n: f64) -> Expr {
    Expr::new(ExprKind::Num(n), None)
}

fn block(kinds: Vec<StmtKind>) -> Block {
    Block {
        stmts: kinds.into_iter().map(Stmt::new).collect(),
        ..Block::default()
    }
}

/// `Global.A += 1; If (Global.A > 3) { Teleport(Event Player, Vector(0,0,0)) } Else { Skip(label) }`
fn sample() -> Block {
    let a = VarId(0);
    let global_a = Expr::new(ExprKind::GlobalVar(a), None);
    let event_player = Expr::new(
        ExprKind::Value {
            id: ValueId::EventPlayer,
            args: vec![],
        },
        None,
    );
    let origin = Expr::new(ExprKind::Vector([num(0.0), num(0.0), num(0.0)]), None);
    let cond = Expr::new(
        ExprKind::Compare {
            op: CmpOp::Gt,
            lhs: global_a,
            rhs: num(3.0),
        },
        None,
    );
    let label = LabelId(0);
    block(vec![
        StmtKind::Modify {
            target: Place::Global(a),
            op: ModifyOp::Add,
            value: num(1.0),
        },
        StmtKind::If {
            cond,
            then: block(vec![StmtKind::Action {
                id: ActionId::Teleport,
                args: vec![event_player, origin],
            }]),
            elifs: vec![],
            else_: Some(block(vec![StmtKind::Skip(SkipTarget::Label(label))])),
        },
        StmtKind::Label(label),
        StmtKind::StartRule(SubId(0), StartRuleBehavior::Restart),
    ])
}

#[test]
fn hand_built_program_clones_and_compares() {
    let program = sample();
    assert_eq!(program, program.clone());
    assert_eq!(program.stmts.len(), 4);
    let mut changed = program.clone();
    changed.stmts.pop();
    assert_ne!(program, changed);
}

#[test]
fn expr_clone_shares_the_node() {
    let e = Expr::new(ExprKind::Array(vec![num(1.0), num(2.0)]), None);
    let copy = e.clone();
    assert_eq!(e, copy);
    assert!(matches!(copy.kind(), ExprKind::Array(items) if items.len() == 2));
}

#[test]
fn nested_places_and_chase() {
    let grid = Place::Index {
        base: Box::new(Place::Index {
            base: Box::new(Place::Global(VarId(1))),
            index: num(2.0),
        }),
        index: num(3.0),
    };
    let stmt = Stmt::new(StmtKind::Chase {
        target: grid.clone(),
        dest: num(10.0),
        rate_or_duration: num(1.0),
        reeval: Expr::new(
            ExprKind::Const(ConstRef {
                enum_name: "ChaseRateReeval",
                key: "DESTINATION_AND_RATE",
            }),
            None,
        ),
        over_time: false,
    });
    assert_eq!(stmt.clone(), stmt);
    assert_eq!(grid.clone(), grid);
}

#[test]
fn nodes_carry_locations_and_comments() {
    let loc = Location {
        file: "src/main.rs",
        line: 12,
        column: 5,
    };
    let e = Expr::new(ExprKind::Null, Some(loc));
    assert_eq!(e.loc(), Some(loc));
    let mut s = Stmt::new(StmtKind::Pass);
    s.comment = Some("note".into());
    s.loc = Some(loc);
    assert_eq!(s.comment.as_deref(), Some("note"));
}
