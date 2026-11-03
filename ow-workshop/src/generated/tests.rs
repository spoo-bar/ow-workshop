use ow_workshop_data::{ActionId, ValueId};

use super::api::{GENERATED_ACTIONS, GENERATED_VALUES, HAND_WRITTEN_ACTIONS, HAND_WRITTEN_VALUES};
use crate::ir::{ExprKind, StmtKind};
use crate::prelude::*;

#[test]
fn every_public_action_is_reachable() {
    for &id in ActionId::ALL {
        if id.signature().flags.internal() {
            continue;
        }
        assert!(
            GENERATED_ACTIONS.contains(&id) || HAND_WRITTEN_ACTIONS.contains(&id),
            "{id:?} has no API"
        );
    }
}

#[test]
fn every_public_value_is_reachable() {
    for &id in ValueId::ALL {
        if id.signature().flags.internal() {
            continue;
        }
        assert!(
            GENERATED_VALUES.contains(&id) || HAND_WRITTEN_VALUES.contains(&id),
            "{id:?} has no API"
        );
    }
}

#[test]
fn nothing_is_both_generated_and_hand_written() {
    assert!(
        !GENERATED_ACTIONS
            .iter()
            .any(|a| HAND_WRITTEN_ACTIONS.contains(a))
    );
    assert!(
        !GENERATED_VALUES
            .iter()
            .any(|v| HAND_WRITTEN_VALUES.contains(v))
    );
}

fn only(b: Block) -> crate::ir::Stmt {
    let mut stmts = b.into_ir().stmts;
    assert_eq!(stmts.len(), 1);
    stmts.remove(0)
}

#[test]
fn actions_record_positional_arguments_in_order() {
    let me: Expr<Player> = null();
    let mut b = Block::new();
    b.teleport(&me, vect(1, 2, 3));
    let stmt = only(b);
    assert!(stmt.loc.is_some_and(|l| l.file.ends_with("tests.rs")));
    let StmtKind::Action { id, args } = stmt.kind else {
        panic!("not an action")
    };
    assert_eq!(id, ActionId::Teleport);
    assert_eq!(args.len(), 2);
    assert!(matches!(args[1].kind(), ExprKind::Vector(_)));
}

#[test]
fn optional_arguments_default_and_override() {
    let mut b = Block::new();
    b.hud_text();
    b.hud_text_with(HudTextArgs {
        header: "Score".into_expr(),
        sort_order: 5.into(),
        ..HudTextArgs::default()
    });
    let stmts = b.into_ir().stmts;
    let arg = |i: usize, n: usize| match &stmts[i].kind {
        StmtKind::Action { id, args } => {
            assert_eq!(*id, ActionId::HudText);
            args[n].clone()
        }
        other => panic!("unexpected {other:?}"),
    };
    // Defaults: header is `Null`, sort order 0; the override replaces both.
    assert_eq!(arg(0, 1).kind(), &ExprKind::Null);
    assert_eq!(arg(0, 5).kind(), &ExprKind::Num(0.0));
    assert!(matches!(arg(1, 1).kind(), ExprKind::Str(_)));
    assert_eq!(arg(1, 5).kind(), &ExprKind::Num(5.0));
    // The default for `visible_to` is `All Players(All Teams)`.
    assert!(matches!(
        arg(0, 0).kind(),
        ExprKind::Value {
            id: ValueId::GetPlayers,
            ..
        }
    ));
}

#[test]
fn values_are_functions_and_receiver_methods() {
    let me: Expr<Player> = null();
    let d = distance(vect(0, 0, 0), me.position());
    let ExprKind::Value { id, args } = d.ir().kind() else {
        panic!("not a value")
    };
    assert_eq!(*id, ValueId::Distance);
    assert_eq!(args.len(), 2);
    let ExprKind::Value { id, args } = args[1].kind() else {
        panic!("not a value")
    };
    assert_eq!(*id, ValueId::GetPosition);
    assert_eq!(args.len(), 1);

    let parts = Expr::<Str>::from("a,b").split(",");
    assert!(matches!(
        parts.ir().kind(),
        ExprKind::Value {
            id: ValueId::Split,
            ..
        }
    ));
    let items: Expr<Array<Num>> = [1, 2].into_expr();
    assert!(matches!(
        items.last().ir().kind(),
        ExprKind::Value {
            id: ValueId::Last,
            ..
        }
    ));
    assert!(
        matches!(items.slice(1, 5).ir().kind(), ExprKind::Value { id: ValueId::Slice, args } if args.len() == 3)
    );
}

#[test]
fn renamed_values_live_in_modules() {
    let n = random::int(1, 6);
    assert!(matches!(
        n.ir().kind(),
        ExprKind::Value {
            id: ValueId::RandomRandint,
            ..
        }
    ));
    let shuffled = random::shuffle([1, 2, 3]);
    assert!(matches!(
        shuffled.ir().kind(),
        ExprKind::Value {
            id: ValueId::RandomShuffle,
            ..
        }
    ));
}
