use super::*;
use crate::ir::StmtKind;
use crate::lower::testutil::*;
use crate::options::OptLevel;
use crate::prelude::{Expr as TExpr, LocalizedString, Num, Str, localized, localized_en};

fn lowered(e: TExpr<Str>) -> Result<String, Vec<&'static str>> {
    let mut p = program(vec![StmtKind::Action {
        id: ow_workshop_data::ActionId::Wait,
        args: vec![e.into_ir()],
    }]);
    match run(lower, &mut p, &options(OptLevel::Off)) {
        Ok(_) => {
            let StmtKind::Action { args, .. } = &p.rules[0].body.stmts[0].kind else {
                panic!("not an action");
            };
            Ok(show(&args[0]))
        }
        Err(d) => Err(codes(&d)),
    }
}

#[test]
fn arguments_are_padded_with_null() {
    let a = TExpr::<Num>::from(1);
    let shown = lowered(localized(LocalizedString::SymHashA, [a])).expect("lowers");
    assert_eq!(
        shown,
        "LocalizedString(Str(Localized(SymHashA)), 1, Null, Null)"
    );
}

#[test]
fn english_text_is_looked_up() {
    let a = TExpr::<Num>::from(1);
    let b = TExpr::<Num>::from(2);
    let shown = lowered(localized_en("{0} - {1}", [a, b])).expect("lowers");
    assert!(shown.contains("Localized("), "{shown}");
}

#[test]
fn unknown_phrases_are_errors() {
    let none: [TExpr<Num>; 0] = [];
    assert_eq!(
        lowered(localized_en("definitely not a phrase", none)),
        Err(vec!["unknown-localized-string"])
    );
}

#[test]
fn more_than_three_arguments_are_errors() {
    assert_eq!(
        lowered(localized(LocalizedString::SymHashA, [1, 2, 3, 4])),
        Err(vec!["too-many-localized-args"])
    );
}
