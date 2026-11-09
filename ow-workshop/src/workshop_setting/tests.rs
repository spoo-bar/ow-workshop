use ow_workshop_data::Hero;

use super::*;
use crate::lower::testutil::show;
use crate::ty::{Bool, Num};

fn shown<T: crate::ty::ValueType>(e: impl IntoExpr<T>) -> String {
    show(e.into_expr().ir())
}

#[test]
fn toggle_lowers_to_the_toggle_value() {
    let e: Expr<Bool> = toggle("Gameplay", "Friendly fire", false).sort(2).into();
    let ExprKind::Value { id, args } = e.ir().kind() else {
        panic!("not a value");
    };
    assert_eq!(*id, ValueId::CreateWorkshopSettingBool);
    assert_eq!(args.len(), 4);
    assert_eq!(show(&args[2]), "false");
    assert_eq!(show(&args[3]), "2");
    assert!(e.ir().loc().is_some());
}

#[test]
fn int_defaults_to_the_widest_range() {
    let s = shown::<Num>(int("c", "n", 7));
    assert!(s.ends_with("7, -999999999999, 999999999999, 0)"), "{s}");
}

#[test]
fn int_ranges() {
    let end = |s: String| s.split(", ").skip(3).collect::<Vec<_>>().join(", ");
    assert_eq!(
        end(shown::<Num>(int("c", "n", 3).range(1..=50))),
        "1, 50, 0)"
    );
    assert_eq!(
        end(shown::<Num>(int("c", "n", 3).unsigned())),
        "0, 999999999999, 0)"
    );
    assert_eq!(
        end(shown::<Num>(int("c", "n", 3).signed())),
        "-999999999999, 0, 0)"
    );
    assert_eq!(
        end(shown::<Num>(int("c", "n", 3).unsigned().range(2..=9))),
        "2, 9, 0)"
    );
}

#[test]
fn real_lowers_to_the_real_value() {
    let e: Expr<Num> = real("c", "n", 1.5).range(0.5..=3.0).sort(1).into();
    let ExprKind::Value { id, args } = e.ir().kind() else {
        panic!("not a value");
    };
    assert_eq!(*id, ValueId::CreateWorkshopSettingFloat);
    let nums: Vec<String> = args[2..].iter().map(show).collect();
    assert_eq!(nums, ["1.5", "0.5", "3", "1"]);
}

#[test]
fn hero_default_is_a_constant() {
    let e: Expr<Hero> = hero("c", "n", Hero::Reinhardt).into();
    let ExprKind::Value { id, args } = e.ir().kind() else {
        panic!("not a value");
    };
    assert_eq!(*id, ValueId::CreateWorkshopSettingHero);
    assert!(matches!(
        args[2].kind(),
        ExprKind::Const(c) if c.enum_name == "Hero" && c.key == "reinhardt"
    ));
}

#[test]
fn combo_lists_its_options() {
    let e: Expr<Num> = combo("c", "n", ["Easy", "Normal", "Hard"], 1).into();
    let ExprKind::Value { id, args } = e.ir().kind() else {
        panic!("not a value");
    };
    assert_eq!(*id, ValueId::CreateWorkshopSettingEnum);
    assert_eq!(show(&args[2]), "1");
    let ExprKind::Array(options) = args[3].kind() else {
        panic!("no options");
    };
    assert_eq!(options.len(), 3);
    assert_eq!(args.len(), 5);
}

#[test]
fn builders_work_wherever_a_value_is_expected() {
    fn takes(_: impl IntoExpr<Bool>) {}
    takes(toggle("c", "n", true));
}
