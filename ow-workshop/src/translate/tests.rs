use super::*;
use crate::ty::Num;

fn kind(e: &Expr<Str>) -> ExprKind {
    e.clone().into_ir().kind().clone()
}

#[test]
fn t_records_text_context_and_arguments() {
    let e = t_ctx!("ctx", "{} left", 1);
    let ExprKind::Translated(t) = kind(&e) else {
        panic!("not translated");
    };
    assert_eq!(t.key, "{} left");
    assert_eq!(t.context.as_deref(), Some("ctx"));
    assert_eq!(t.args.len(), 1);
    assert!(!t.raw && !t.spectator);
    assert!(e.into_ir().loc().is_some());
}

#[test]
fn tr_of_a_literal_string_is_the_string() {
    let ExprKind::Translated(t) = kind(&tr(tr_raw(t!("a")))) else {
        panic!("not translated");
    };
    assert!(!t.raw);
    let ExprKind::Translated(t) = kind(&tr_spectator(t!("a"))) else {
        panic!("not translated");
    };
    assert!(t.spectator);
}

#[test]
fn tr_of_anything_else_resolves() {
    let value = Expr::<Num>::from(1).erase();
    let ExprKind::Resolve { spectator, .. } = kind(&tr(value.clone())) else {
        panic!("not resolved");
    };
    assert!(!spectator);
    let ExprKind::Resolve { spectator, .. } = kind(&tr_spectator(value)) else {
        panic!("not resolved");
    };
    assert!(spectator);
}

#[test]
fn tr_raw_marks_the_string() {
    let ExprKind::Translated(t) = tr_raw(t!("a")).into_ir().kind().clone() else {
        panic!("not translated");
    };
    assert!(t.raw);
}
