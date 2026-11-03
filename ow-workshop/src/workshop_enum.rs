//! Tests for `#[derive(WorkshopEnum)]`.

use crate::ir::ExprKind;
use crate::prelude::*;

#[derive(Clone, Copy, WorkshopEnum)]
enum State {
    Lobby,
    Playing,
    Over,
}

#[derive(Clone, Copy, WorkshopEnum)]
enum Wave {
    Calm = 3,
    Rush,
    Boss = 10,
    Finale,
}

#[derive(Clone, Copy, WorkshopEnum)]
#[ow(ty = Team)]
enum Side {
    #[ow(value = Team::One)]
    Red,
    #[ow(value = Team::Two)]
    Blue,
}

fn num(e: &Expr<Num>) -> f64 {
    match e.ir().kind() {
        ExprKind::Num(n) => *n,
        other => panic!("not a number: {other:?}"),
    }
}

#[test]
fn implicit_discriminants_start_at_zero() {
    let values = [State::Lobby, State::Playing, State::Over].map(|s| num(&s.into_expr()));
    assert_eq!(values, [0.0, 1.0, 2.0]);
    assert_eq!(State::COUNT, 3);
}

#[test]
fn explicit_discriminants_continue_from_the_previous() {
    let values = [Wave::Calm, Wave::Rush, Wave::Boss, Wave::Finale].map(|w| num(&w.into()));
    assert_eq!(values, [3.0, 4.0, 10.0, 11.0]);
    assert_eq!(Wave::COUNT, 4);
}

#[test]
fn values_attribute_gives_typed_constants() {
    let red: Expr<Team> = Side::Red.into();
    let blue = Side::Blue.into_expr();
    let key = |e: &Expr<Team>| match e.ir().kind() {
        ExprKind::Const(c) => (c.enum_name, c.key),
        other => panic!("not a constant: {other:?}"),
    };
    assert_eq!(key(&red), ("Team", Team::One.key()));
    assert_eq!(key(&blue), ("Team", Team::Two.key()));
    assert_eq!(Side::COUNT, 2);
}

#[test]
fn values_lists_every_variant_in_order() {
    let all = State::values();
    let ExprKind::Array(items) = all.ir().kind() else {
        panic!("not an array")
    };
    let numbers: Vec<_> = items
        .iter()
        .map(|i| match i.kind() {
            ExprKind::Num(n) => *n,
            other => panic!("unexpected {other:?}"),
        })
        .collect();
    assert_eq!(numbers, [0.0, 1.0, 2.0]);

    let sides = Side::values();
    let ExprKind::Array(items) = sides.ir().kind() else {
        panic!("not an array")
    };
    assert_eq!(items.len(), 2);
}

#[test]
fn usable_wherever_a_value_is_expected() {
    let state: Expr<Num> = Expr::from(State::Playing);
    assert!(matches!(
        state.eq(State::Over).ir().kind(),
        ExprKind::Compare { .. }
    ));
}
