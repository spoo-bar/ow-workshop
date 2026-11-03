use ow_workshop_data::ValueId;

use super::api::{GENERATED_VALUES, HAND_WRITTEN_VALUES};
use crate::ir::ExprKind;
use crate::prelude::*;

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
        !GENERATED_VALUES
            .iter()
            .any(|v| HAND_WRITTEN_VALUES.contains(v))
    );
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
        matches!(items.slice(1).ir().kind(), ExprKind::Value { id: ValueId::Slice, args } if args.len() == 3)
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
