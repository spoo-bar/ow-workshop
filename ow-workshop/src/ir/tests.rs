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

/// Wraps `Rule::new` the way a user-facing builder would.
#[track_caller]
fn build_rule() -> Rule {
    Rule::new("Init", Event::OngoingGlobal, Block::default())
}

#[test]
fn rule_location_is_the_outermost_call_site() {
    let (rule, line) = (build_rule(), line!());
    let loc = rule.location.expect("rule has a location");
    assert_eq!(loc.file, file!());
    assert_eq!(loc.line, line);
    assert_eq!(
        loc.to_string(),
        format!("{}:{line}:{}", file!(), loc.column)
    );
}

#[test]
fn other_nodes_capture_locations() {
    let (stmt, line) = (Stmt::new(StmtKind::Pass), line!());
    assert_eq!(stmt.loc.map(|l| l.line), Some(line));
    let (expr, line) = (Expr::here(ExprKind::Null), line!());
    assert_eq!(expr.loc().map(|l| l.line), Some(line));
    let (var, line) = (VarDecl::new("v"), line!());
    assert_eq!(var.location.map(|l| l.line), Some(line));
    let (sub, line) = (SubDecl::new("s"), line!());
    assert_eq!(sub.location.map(|l| l.line), Some(line));
}
