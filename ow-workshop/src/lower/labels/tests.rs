use super::*;
use crate::ir::{Event, Rule};
use crate::lower::testutil::*;

fn dist(n: u8) -> StmtKind {
    StmtKind::Skip(SkipTarget::Dynamic(num(f64::from(n))))
}

fn resolve(body: Vec<StmtKind>, opt: OptLevel) -> Result<Program, Diagnostics> {
    let mut p = program(body);
    run(resolve_labels, &mut p, &options(opt))?;
    Ok(p)
}

#[test]
fn counts_the_lines_between_skip_and_label() {
    let p = resolve(
        vec![skip_to(0), action(), action(), place(0), action()],
        OptLevel::Off,
    )
    .expect("resolves");
    assert_eq!(kinds(&p)[0], &dist(2));
}

#[test]
fn nested_blocks_count_their_lines_and_end() {
    // Skip, If, Wait, End, [label], Wait: the If's End is followed by a line, so it counts.
    let p = resolve(
        vec![
            skip_to(0),
            if_(boolean(true), vec![action()]),
            place(0),
            action(),
        ],
        OptLevel::Off,
    )
    .expect("resolves");
    assert_eq!(kinds(&p)[0], &dist(3));
}

#[test]
fn trailing_ends_are_not_counted() {
    // The rule ends with the If, so flatten omits its End.
    let p = resolve(
        vec![skip_to(0), if_(boolean(true), vec![action()]), place(0)],
        OptLevel::Off,
    )
    .expect("resolves");
    assert_eq!(kinds(&p)[0], &dist(2));
}

#[test]
fn skip_from_inside_a_block_to_after_it() {
    let p = resolve(
        vec![
            if_(boolean(true), vec![skip_to(0), action()]),
            place(0),
            action(),
        ],
        OptLevel::Off,
    )
    .expect("resolves");
    let StmtKind::If { then, .. } = kinds(&p)[0] else {
        panic!("not an if")
    };
    // Wait, End.
    assert_eq!(then.stmts[0].kind, dist(2));
}

#[test]
fn labels_comments_and_pass_do_not_count() {
    let p = resolve(
        vec![
            skip_to(1),
            place(0),
            StmtKind::Pass,
            StmtKind::Comment("hi".into()),
            action(),
            place(1),
        ],
        OptLevel::Off,
    )
    .expect("resolves");
    assert_eq!(kinds(&p)[0], &dist(1));
}

#[test]
fn skip_if_keeps_its_condition() {
    let p = resolve(
        vec![
            StmtKind::SkipIf(boolean(true), SkipTarget::Label(LabelId(0))),
            action(),
            place(0),
            action(),
        ],
        OptLevel::Off,
    )
    .expect("resolves");
    assert_eq!(
        kinds(&p)[0],
        &StmtKind::SkipIf(boolean(true), SkipTarget::Dynamic(num(1.0)))
    );
}

#[test]
fn missing_and_backward_labels_are_errors() {
    let err = resolve(vec![skip_to(0), action()], OptLevel::Off).expect_err("no label");
    assert_eq!(codes(&err), ["label-not-found"]);
    let err = resolve(vec![place(0), action(), skip_to(0)], OptLevel::Off).expect_err("backward");
    assert_eq!(codes(&err), ["label-not-found"]);
}

#[test]
fn a_label_placed_twice_is_an_error() {
    let err = resolve(
        vec![skip_to(0), place(0), action(), place(0)],
        OptLevel::Off,
    )
    .expect_err("duplicate");
    assert_eq!(codes(&err), ["duplicate-label"]);
}

#[test]
fn zero_distance_skips_are_removed_when_optimizing() {
    // Removing the inner `Skip(0)` shifts the outer distance from 2 to 1.
    let body = vec![
        skip_to(1),
        skip_to(0),
        place(0),
        action(),
        place(1),
        action(),
    ];
    let p = resolve(body.clone(), OptLevel::Size).expect("resolves");
    assert_eq!(
        kinds(&p),
        [&dist(1), &place(0), &action(), &place(1), &action()]
    );
    let p = resolve(body, OptLevel::Off).expect("resolves");
    assert_eq!(kinds(&p)[0], &dist(2));
    assert_eq!(kinds(&p)[1], &dist(0));
}

#[test]
fn removal_cascades_until_stable() {
    // Once the inner skip is gone, the outer one also jumps over nothing.
    let p = resolve(
        vec![skip_to(1), skip_to(0), place(0), place(1), action()],
        OptLevel::Speed,
    )
    .expect("resolves");
    assert_eq!(kinds(&p).len(), 3);
    assert!(matches!(kinds(&p)[0], StmtKind::Label(_)));
}

#[test]
fn a_dynamic_skip_keeps_every_statement() {
    let p = resolve(
        vec![
            StmtKind::Skip(SkipTarget::Dynamic(expr(ExprKind::GlobalVar(
                crate::ir::VarId(0),
            )))),
            skip_to(0),
            place(0),
            action(),
        ],
        OptLevel::Size,
    )
    .expect("resolves");
    assert_eq!(kinds(&p)[1], &dist(0));
    let rule = &p.rules[0];
    assert!(has_dynamic_skip(rule));
}

#[test]
fn label_distance_placeholders_are_measured_from_their_skip() {
    let table = expr(ExprKind::Array(vec![
        expr(ExprKind::LabelDistance(LabelId(0))),
        expr(ExprKind::LabelDistance(LabelId(1))),
    ]));
    let body = vec![
        StmtKind::Skip(SkipTarget::Dynamic(table)),
        place(0),
        action(),
        place(1),
        action(),
    ];
    // Placeholders are resolvable, so they do not count as a dynamic skip.
    assert!(!has_dynamic_skip(&program(body.clone()).rules[0]));
    let p = resolve(body, OptLevel::Size).expect("resolves");
    assert_eq!(
        kinds(&p)[0],
        &StmtKind::Skip(SkipTarget::Dynamic(expr(ExprKind::Array(vec![
            num(0.0),
            num(1.0)
        ]))))
    );
}

#[test]
fn every_rule_is_resolved_on_its_own() {
    let mut p = program(vec![skip_to(0), action(), place(0)]);
    p.rules.push(Rule::new(
        "second",
        Event::OngoingGlobal,
        block(vec![skip_to(0), action(), action(), place(0)]),
    ));
    run(resolve_labels, &mut p, &options(OptLevel::Off)).expect("resolves");
    assert_eq!(p.rules[0].body.stmts[0].kind, dist(1));
    assert_eq!(p.rules[1].body.stmts[0].kind, dist(2));
}
