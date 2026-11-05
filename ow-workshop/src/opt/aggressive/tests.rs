use ow_workshop_data::{ActionId, ValueId as V};

use crate::ir::CmpOp::Eq;
use crate::ir::{Expr, ExprKind, Place, Program, SkipTarget, StmtKind, VarId};
use crate::lower::labels::resolve_labels;
use crate::lower::testutil::{act, num, program, run};
use crate::opt::testutil::*;
use crate::options::{CompileOptions, OptLevel};

fn opts(level: OptLevel, aggressive: bool) -> CompileOptions {
    CompileOptions {
        optimize: level,
        aggressive_size: aggressive,
        ..CompileOptions::default()
    }
}

fn kill() -> StmtKind {
    act(ActionId::Kill, vec![])
}

fn optimized_with(body: Vec<StmtKind>, options: &CompileOptions) -> Program {
    let mut p = program(body);
    run(crate::opt::run, &mut p, options).expect("optimizes");
    p
}

fn aggressive(body: Vec<StmtKind>) -> Program {
    optimized_with(body, &opts(OptLevel::Size, true))
}

fn if_(cond: Expr, then: Vec<StmtKind>) -> StmtKind {
    crate::lower::testutil::if_(cond, then)
}

fn shape(p: &Program) -> Vec<String> {
    p.rules[0]
        .body
        .stmts
        .iter()
        .map(|s| match &s.kind {
            StmtKind::SkipIf(c, SkipTarget::Label(l)) => format!("skipif {} -> L{}", sh(c), l.0),
            StmtKind::SkipIf(c, SkipTarget::Dynamic(d)) => format!("skipif {} by {}", sh(c), sh(d)),
            StmtKind::Label(l) => format!("L{}", l.0),
            StmtKind::Pass => "pass".to_owned(),
            StmtKind::If { cond, .. } => format!("if {}", sh(cond)),
            StmtKind::Action { id, .. } => format!("{id:?}"),
            other => format!("{other:?}"),
        })
        .collect()
}

// ---- If → Skip If -------------------------------------------------------------------------

#[test]
fn if_not_becomes_skip_if() {
    let p = aggressive(vec![if_(not(g(0)), vec![kill(), kill()]), kill()]);
    assert_eq!(
        shape(&p),
        ["skipif g0 -> L0", "Kill", "Kill", "L0", "pass", "Kill"]
    );
}

#[test]
fn if_comparison_with_one_child_becomes_inverted_skip_if() {
    let p = aggressive(vec![if_(cmp(Eq, g(0), g(1)), vec![kill()]), kill()]);
    assert_eq!(
        shape(&p),
        ["skipif (g0 != g1) -> L0", "Kill", "L0", "pass", "Kill"]
    );
}

#[test]
fn if_comparison_with_several_children_stays() {
    let p = aggressive(vec![if_(cmp(Eq, g(0), g(1)), vec![kill(), kill()]), kill()]);
    assert_eq!(shape(&p), ["if (g0 == g1)", "Kill"]);
}

#[test]
fn other_conditions_stay() {
    let p = aggressive(vec![if_(g(0), vec![kill()]), kill()]);
    assert_eq!(shape(&p), ["if g0", "Kill"]);
}

#[test]
fn chains_stay() {
    let chain = StmtKind::If {
        cond: not(g(0)),
        then: crate::lower::testutil::block(vec![kill()]),
        elifs: vec![],
        else_: Some(crate::lower::testutil::block(vec![kill()])),
    };
    let p = aggressive(vec![chain]);
    assert_eq!(shape(&p), ["if !g0"]);
}

#[test]
fn skip_distances_count_the_skipped_lines() {
    let mut p = program(vec![if_(not(g(0)), vec![kill(), kill(), kill()]), kill()]);
    let options = opts(OptLevel::Size, true);
    run(crate::opt::run, &mut p, &options).expect("optimizes");
    run(resolve_labels, &mut p, &options).expect("resolves");
    let StmtKind::SkipIf(_, SkipTarget::Dynamic(distance)) = &p.rules[0].body.stmts[0].kind else {
        panic!("not a resolved skip")
    };
    assert_eq!(sh(distance), "3");
}

#[test]
fn nested_ifs_keep_correct_distances() {
    let inner = if_(not(g(1)), vec![kill(), kill()]);
    let mut p = program(vec![if_(not(g(0)), vec![kill(), inner, kill()]), kill()]);
    let options = opts(OptLevel::Size, true);
    run(crate::opt::run, &mut p, &options).expect("optimizes");
    run(resolve_labels, &mut p, &options).expect("resolves");
    let skips: Vec<String> = p.rules[0]
        .body
        .stmts
        .iter()
        .filter_map(|s| match &s.kind {
            StmtKind::SkipIf(c, SkipTarget::Dynamic(d)) => Some(format!("{} {}", sh(c), sh(d))),
            _ => None,
        })
        .collect();
    // Outer: Kill, inner skip, Kill, Kill, Kill = 5 lines; inner: 2 lines.
    assert_eq!(skips, ["g0 5", "g1 2"]);
}

#[test]
fn nothing_changes_without_aggressive_size_or_size() {
    let body = || vec![if_(not(g(0)), vec![kill()]), kill()];
    for options in [
        opts(OptLevel::Size, false),
        opts(OptLevel::Speed, true),
        opts(OptLevel::Off, true),
    ] {
        let p = optimized_with(body(), &options);
        assert_eq!(shape(&p), ["if !g0", "Kill"], "{options:?}");
    }
}

#[test]
fn rules_with_a_dynamic_skip_are_left_alone() {
    let dynamic = StmtKind::Skip(SkipTarget::Dynamic(g(5)));
    let p = aggressive(vec![dynamic, if_(not(g(0)), vec![kill()]), kill()]);
    assert_eq!(shape(&p)[1], "if !g0");
}

// ---- arrays -------------------------------------------------------------------------------

fn set_value(value: Expr) -> StmtKind {
    StmtKind::Set {
        target: Place::Global(VarId(0)),
        value,
    }
}

fn assigned(body: Expr) -> String {
    let p = aggressive(vec![set_value(body), kill()]);
    match &p.rules[0].body.stmts[0].kind {
        StmtKind::Set { value, .. } => sh(value),
        other => panic!("not a set: {other:?}"),
    }
}

fn strings(texts: &[&str]) -> Expr {
    arr(texts.iter().map(|t| cstr(t)).collect())
}

#[test]
fn string_arrays_become_splits() {
    assert_eq!(
        assigned(strings(&["a", "b", "c", "d"])),
        "Split(\"a0b0c0d\"(), FirstOf(Null))"
    );
}

#[test]
fn zeros_in_the_text_pick_the_vector_separator() {
    assert_eq!(
        assigned(strings(&["a0", "b", "c", "d"])),
        "Split(\"a0(1.00, 0.00, 0.00)b(1.00, 0.00, 0.00)c(1.00, 0.00, 0.00)d\"(), FirstOf(VectorLeft()))"
    );
}

#[test]
fn text_with_both_separators_uses_the_private_one() {
    let p = assigned(strings(&["a0", "(1.00, 0.00, 0.00)", "c"]));
    assert_eq!(
        p,
        "Split(\"a0\u{EC51}(1.00, 0.00, 0.00)\u{EC51}c\"(), \"\u{EC51}\"())"
    );
}

#[test]
fn short_string_arrays_stay() {
    assert_eq!(
        assigned(strings(&["a", "b", "c"])),
        "[\"a\"(), \"b\"(), \"c\"()]"
    );
    assert_eq!(assigned(strings(&["a", "b"])), "[\"a\"(), \"b\"()]");
}

#[test]
fn strings_that_do_not_fit_stay() {
    let (x, y, z) = ("x".repeat(60), "y".repeat(60), "z".repeat(60));
    let texts = [x.as_str(), y.as_str(), z.as_str()];
    let before = sh(&strings(&texts));
    assert_eq!(assigned(strings(&texts)), before);
}

#[test]
fn arrays_with_other_items_stay() {
    let mixed = arr(vec![cstr("a"), g(0), cstr("c"), cstr("d")]);
    assert_eq!(assigned(mixed), "[\"a\"(), g0, \"c\"(), \"d\"()]");
    // An empty string cannot be split out; size mode spells it differently first.
    let empty_text = strings(&["a", "", "c", "d"]);
    assert_eq!(
        assigned(empty_text),
        "[\"a\"(), CharAt([], false), \"c\"(), \"d\"()]"
    );
}

#[test]
fn the_options_of_a_combo_setting_stay_strings() {
    let combo = val(
        V::CreateWorkshopSettingEnum,
        vec![
            cstr("cat"),
            cstr("name"),
            num(0.0),
            strings(&["a", "b", "c", "d"]),
            num(0.0),
        ],
    );
    let before = sh(&combo);
    assert_eq!(assigned(combo), before);
}

fn repeated(item: &Expr, n: usize) -> Expr {
    arr(vec![item.clone(); n])
}

#[test]
fn repeated_elements_become_a_mapped_split() {
    // A number costs 1 element: it takes 11 of them.
    assert_eq!(
        assigned(repeated(&num(7.0), 11)),
        "MappedArray(Split(\"0000000000\"(), FirstOf(Null)), 7)"
    );
    assert_eq!(
        assigned(repeated(&num(7.0), 10)),
        sh(&repeated(&num(7.0), 10))
    );
}

#[test]
fn bigger_elements_need_fewer_repeats() {
    let abs = val(V::Abs, vec![g(1)]); // 2 elements: 5 repeats
    assert!(assigned(repeated(&abs, 5)).starts_with("MappedArray("));
    assert!(assigned(repeated(&abs, 4)).starts_with('['));
    let sum = val(V::Min, vec![g(1), g(2)]); // 3 elements: 4 repeats
    assert!(assigned(repeated(&sum, 4)).starts_with("MappedArray("));
    assert!(assigned(repeated(&sum, 3)).starts_with('['));
    let four = val(V::Slice, vec![g(1), g(2), g(3)]); // 4 elements: 3 repeats
    assert!(assigned(repeated(&four, 3)).starts_with("MappedArray("));
    assert!(assigned(repeated(&four, 2)).starts_with('['));
}

#[test]
fn different_or_random_elements_stay() {
    let mixed = arr(vec![num(7.0); 10].into_iter().chain([num(8.0)]).collect());
    assert!(assigned(mixed).starts_with('['));
    let roll = val(V::RandomRandint, vec![num(1.0), num(2.0)]);
    assert!(assigned(repeated(&roll, 6)).starts_with('['));
}

#[test]
fn lambda_elements_stay() {
    let elem = Expr::new(ExprKind::ArrayElem, None);
    let wrapped = val(V::Abs, vec![elem]);
    assert!(assigned(repeated(&wrapped, 6)).starts_with('['));
}

#[test]
fn a_repeated_array_inside_a_lambda_stays() {
    let abs = val(V::Abs, vec![g(1)]);
    let body = cmp(Eq, Expr::new(ExprKind::ArrayElem, None), repeated(&abs, 5));
    let filtered = val(V::FilteredArray, vec![g(0), body]);
    let shown = assigned(filtered);
    assert!(!shown.contains("MappedArray"), "{shown}");
    // The same array outside a lambda is packed.
    let outside = val(V::FilteredArray, vec![repeated(&abs, 5), g(2)]);
    assert!(assigned(outside).contains("MappedArray"));
}
