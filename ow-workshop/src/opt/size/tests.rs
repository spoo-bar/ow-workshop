use ow_workshop_data::{ActionId, ValueId as V};

use crate::ir::CmpOp::Gt;
use crate::ir::{Expr, ExprKind, ModifyOp, Place, Program, StmtKind, VarId};
use crate::lower::testutil::{act, konst, null, num, options, program, run};
use crate::opt::testutil::*;
use crate::options::OptLevel;

fn optimized(body: Vec<StmtKind>, level: OptLevel) -> Program {
    let mut body = body;
    body.push(act(ActionId::Kill, vec![]));
    let mut p = program(body);
    run(crate::opt::run, &mut p, &options(level)).expect("optimizes");
    p
}

fn sized(body: Vec<StmtKind>) -> Program {
    optimized(body, OptLevel::Size)
}

fn place(n: u32) -> Place {
    Place::Global(VarId(n))
}

fn set(n: u32, value: Expr) -> StmtKind {
    StmtKind::Set {
        target: place(n),
        value,
    }
}

/// The value assigned by the first statement.
fn assigned(p: &Program) -> String {
    match &p.rules[0].body.stmts[0].kind {
        StmtKind::Set { value, .. } | StmtKind::Modify { value, .. } => sh(value),
        other => panic!("not an assignment: {other:?}"),
    }
}

fn arg(p: &Program, i: usize) -> String {
    match &p.rules[0].body.stmts[0].kind {
        StmtKind::Action { args, .. } => sh(&args[i]),
        other => panic!("not an action: {other:?}"),
    }
}

// ---- the element-saving snippet ------------------------------------------------------------

#[test]
fn compare_with_zero_uses_null() {
    let p = sized(vec![set(0, cmp(Gt, g(1), num(0.0)))]);
    assert_eq!(assigned(&p), "(g1 > Null)");
    let mirrored = sized(vec![set(0, cmp(Gt, num(0.0), g(1)))]);
    assert_eq!(assigned(&mirrored), "(Null > g1)");
}

#[test]
fn arrays_use_null_for_zero_but_keep_one() {
    let p = sized(vec![set(0, arr(vec![num(0.0), num(1.0), num(0.0)]))]);
    assert_eq!(assigned(&p), "[Null, 1, Null]");
}

#[test]
fn array_contains_searches_for_null() {
    let p = sized(vec![set(0, val(V::ArrayContains, vec![g(1), num(0.0)]))]);
    assert_eq!(assigned(&p), "ArrayContains(g1, Null)");
}

#[test]
fn assigning_zero_assigns_null() {
    let p = sized(vec![set(0, num(0.0))]);
    assert_eq!(assigned(&p), "Null");
    let one = sized(vec![set(0, num(1.0))]);
    assert_eq!(assigned(&one), "1");
}

#[test]
fn assigning_at_an_index_replaces_value_and_index() {
    let target = Place::Index {
        base: Box::new(place(0)),
        index: g(1),
    };
    let p = sized(vec![StmtKind::Set {
        target,
        value: num(0.0),
    }]);
    assert_eq!(assigned(&p), "Null");
    let zero_index = Place::Index {
        base: Box::new(place(0)),
        index: num(0.0),
    };
    let q = sized(vec![StmtKind::Set {
        target: zero_index,
        value: g(2),
    }]);
    let StmtKind::Set {
        target: Place::Index { index, .. },
        ..
    } = &q.rules[0].body.stmts[0].kind
    else {
        panic!("not an indexed set")
    };
    assert_eq!(sh(index), "false");
    let one_index = sized(vec![StmtKind::Set {
        target: Place::Index {
            base: Box::new(place(0)),
            index: num(1.0),
        },
        value: g(2),
    }]);
    let StmtKind::Set {
        target: Place::Index { index, .. },
        ..
    } = &one_index.rules[0].body.stmts[0].kind
    else {
        panic!("not an indexed set")
    };
    assert_eq!(sh(index), "true");
}

#[test]
fn player_variables_get_null_too() {
    let target = Place::Player {
        player: g(1),
        var: VarId(0),
    };
    let p = sized(vec![StmtKind::Set {
        target,
        value: num(0.0),
    }]);
    assert_eq!(assigned(&p), "Null");
}

#[test]
fn nothing_changes_outside_size_mode() {
    let body = || vec![set(0, cmp(Gt, g(1), num(0.0)))];
    assert_eq!(assigned(&optimized(body(), OptLevel::Speed)), "(g1 > 0)");
    assert_eq!(assigned(&optimized(body(), OptLevel::Off)), "(g1 > 0)");
}

// ---- argument flags ------------------------------------------------------------------------

#[test]
fn zero_and_one_follow_the_signature() {
    let call = |id, args| sized(vec![set(0, val(id, args))]);
    // `Slice` index and count: 0 → False, 1 → True.
    assert_eq!(
        assigned(&call(V::Slice, vec![g(1), num(0.0), num(1.0)])),
        "Slice(g1, false, true)"
    );
    // `Min`/`Max` accept both too.
    assert_eq!(
        assigned(&call(V::Min, vec![g(1), num(0.0)])),
        "Min(g1, false)"
    );
    // `Abs` has no flags.
    assert_eq!(assigned(&call(V::Abs, vec![g(1)])), "Abs(g1)");
    // `Add` replaces 1 only.
    assert_eq!(
        assigned(&sized(vec![set(0, add(g(1), num(1.0)))])),
        "(g1 + true)"
    );
}

#[test]
fn if_then_else_branches_use_null() {
    let p = sized(vec![set(0, ite(g(1), num(0.0), num(5.0)))]);
    assert_eq!(assigned(&p), "(g1 ? Null : 5)");
}

#[test]
fn custom_string_arguments_use_null() {
    // A zero literal is folded into the text; any other zero-valued argument becomes `Null`.
    let formatted = |arg| {
        Expr::new(
            ExprKind::Str(crate::ir::StrExpr::Custom {
                tokens: vec![crate::ir::StrToken::Arg(0)],
                args: vec![arg],
            }),
            None,
        )
    };
    let p = sized(vec![set(0, formatted(num(0.0)))]);
    assert_eq!(assigned(&p), "\"0\"()");
    let p = sized(vec![set(0, formatted(val(V::Index, vec![g(1), num(0.0)])))]);
    assert_eq!(assigned(&p), "\"{0}\"(Index(g1, Null))");
}

#[test]
#[allow(clippy::many_single_char_names)]
fn the_zero_vector_becomes_null_or_left_plus_right() {
    // The position of in-world text allows `Null`.
    let p = sized(vec![act(
        ActionId::CreateInWorldText,
        vec![("position", v3(0.0, 0.0, 0.0))],
    )]);
    assert_eq!(arg(&p, 2), "Null");
    // `Teleport.position` does not.
    let t = sized(vec![act(
        ActionId::Teleport,
        vec![("position", v3(0.0, 0.0, 0.0))],
    )]);
    assert_eq!(arg(&t, 1), "(VectorLeft() + VectorRight())");
    // Elsewhere it is the sum of opposite directions.
    let q = sized(vec![set(0, val(V::Magnitude, vec![v3(0.0, 0.0, 0.0)]))]);
    assert_eq!(
        assigned(&q),
        "Null",
        "folded to 0 first, then assigned as Null"
    );
    let r = sized(vec![set(0, val(V::Normalize, vec![g(1)]))]);
    assert_eq!(assigned(&r), "Normalize(g1)");
    let s = sized(vec![set(
        0,
        val(V::DotProduct, vec![g(1), v3(0.0, 0.0, 0.0)]),
    )]);
    assert_eq!(
        assigned(&s),
        "DotProduct(g1, (VectorLeft() + VectorRight()))"
    );
    let t = sized(vec![set(0, v3(0.0, 0.0, 0.0))]);
    assert_eq!(assigned(&t), "(VectorLeft() + VectorRight())");
}

#[test]
fn the_empty_string_has_three_spellings() {
    // `StrContains` takes an empty array in place of "".
    let p = sized(vec![set(0, val(V::StrContains, vec![g(1), cstr("")]))]);
    assert_eq!(assigned(&p), "StrContains(g1, [])");
    // Without the flag, the first character of nothing.
    let q = sized(vec![set(0, cstr(""))]);
    assert_eq!(assigned(&q), "CharAt([], false)");
}

#[test]
fn workshop_settings_keep_their_literals() {
    let setting = val(
        V::CreateWorkshopSettingInt,
        vec![null(), null(), num(0.0), num(0.0), num(1.0), num(0.0)],
    );
    let before = sh(&setting);
    let p = sized(vec![set(0, setting)]);
    assert_eq!(assigned(&p), before);
}

// ---- modify -------------------------------------------------------------------------------

fn modify(op: ModifyOp, value: Expr) -> StmtKind {
    StmtKind::Modify {
        target: place(0),
        op,
        value,
    }
}

#[test]
fn modify_with_zero_or_one() {
    for op in [
        ModifyOp::Add,
        ModifyOp::Subtract,
        ModifyOp::Modulo,
        ModifyOp::Max,
        ModifyOp::Min,
        ModifyOp::RemoveFromArrayByIndex,
    ] {
        if matches!(op, ModifyOp::Add | ModifyOp::Subtract) {
            // Adding or subtracting zero is dropped outright.
            let p = sized(vec![modify(op, num(0.0))]);
            assert_eq!(p.rules[0].body.stmts.len(), 1, "{op:?}");
        } else {
            assert_eq!(
                assigned(&sized(vec![modify(op, num(0.0))])),
                "false",
                "{op:?}"
            );
        }
        assert_eq!(
            assigned(&sized(vec![modify(op, num(1.0))])),
            "true",
            "{op:?}"
        );
        assert_eq!(assigned(&sized(vec![modify(op, num(2.0))])), "2", "{op:?}");
    }
    for op in [ModifyOp::Multiply, ModifyOp::Divide, ModifyOp::RaiseToPower] {
        assert_eq!(assigned(&sized(vec![modify(op, num(1.0))])), "1", "{op:?}");
    }
}

#[test]
fn modify_append_and_remove_by_value_use_null() {
    for op in [ModifyOp::AppendToArray, ModifyOp::RemoveFromArrayByValue] {
        assert_eq!(
            assigned(&sized(vec![modify(op, num(0.0))])),
            "Null",
            "{op:?}"
        );
        assert_eq!(assigned(&sized(vec![modify(op, num(1.0))])), "1", "{op:?}");
    }
}

// ---- other --------------------------------------------------------------------------------

#[test]
fn short_waits_become_zero_then_false() {
    for d in [0.0, 0.01, 0.016] {
        let p = sized(vec![act(ActionId::Wait, vec![("time", num(d))])]);
        assert_eq!(arg(&p, 0), "false", "{d}");
    }
    let long = sized(vec![act(ActionId::Wait, vec![("time", num(0.017))])]);
    assert_eq!(arg(&long, 0), "0.017");
    let speed = optimized(
        vec![act(ActionId::Wait, vec![("time", num(0.01))])],
        OptLevel::Speed,
    );
    assert_eq!(arg(&speed, 0), "0.01");
}

#[test]
fn hud_text_colors_are_dropped_for_null_text() {
    let color = konst("Color", "RED");
    let p = sized(vec![act(
        ActionId::HudText,
        vec![
            ("header", null()),
            ("subheader", cstr("s")),
            ("text", null()),
            ("headerColor", color.clone()),
            ("subheaderColor", color.clone()),
            ("textColor", color),
        ],
    )]);
    assert_eq!(arg(&p, 6), "Null");
    assert_eq!(arg(&p, 7), "Color.RED");
    assert_eq!(arg(&p, 8), "Null");
}

#[test]
fn progress_bars_and_world_text_drop_colors_for_falsy_values() {
    let color = || konst("Color", "RED");
    let bar = sized(vec![act(
        ActionId::ProgressBarHud,
        vec![
            ("value", num(0.0)),
            ("text", cstr("t")),
            ("progressBarColor", color()),
            ("textColor", color()),
        ],
    )]);
    assert_eq!(arg(&bar, 5), "Null");
    assert_eq!(arg(&bar, 6), "Color.RED");
    let world = sized(vec![act(
        ActionId::CreateInWorldText,
        vec![("text", null()), ("color", color())],
    )]);
    assert_eq!(arg(&world, 6), "Null");
    let bar_world = sized(vec![act(
        ActionId::CreateProgressBarInWorldText,
        vec![
            ("value", g(1)),
            ("text", fals()),
            ("progressBarColor", color()),
            ("textColor", color()),
        ],
    )]);
    assert_eq!(arg(&bar_world, 6), "Color.RED");
    assert_eq!(arg(&bar_world, 7), "Null");
}

#[test]
fn grapple_beams_ignore_their_color() {
    let beam = |kind| {
        sized(vec![act(
            ActionId::CreateBeam,
            vec![
                ("type", konst("Beam", kind)),
                ("color", konst("Color", "RED")),
            ],
        )])
    };
    assert_eq!(arg(&beam("GRAPPLE"), 4), "Null");
    assert_eq!(arg(&beam("GOOD"), 4), "Color.RED");
}

#[test]
fn folded_arithmetic_is_not_replaced() {
    // `1 + 1` folds to 2 before any replacement could turn a 1 into `True`.
    let p = sized(vec![set(0, add(num(1.0), num(1.0)))]);
    assert_eq!(assigned(&p), "2");
}

#[test]
fn chase_and_for_use_the_cheaper_values() {
    let chase = StmtKind::Chase {
        target: place(0),
        dest: num(0.0),
        rate_or_duration: num(1.0),
        reeval: null(),
        over_time: false,
    };
    let p = sized(vec![chase]);
    let StmtKind::Chase {
        dest,
        rate_or_duration,
        ..
    } = &p.rules[0].body.stmts[0].kind
    else {
        panic!("not a chase")
    };
    assert_eq!(
        (sh(dest).as_str(), sh(rate_or_duration).as_str()),
        ("false", "true")
    );

    let for_loop = StmtKind::For {
        var: place(0),
        start: num(1.0),
        stop: num(0.0),
        step: num(1.0),
        body: crate::ir::Block::default(),
    };
    let p = sized(vec![for_loop]);
    let StmtKind::For {
        start, stop, step, ..
    } = &p.rules[0].body.stmts[0].kind
    else {
        panic!("not a for")
    };
    assert_eq!(
        (sh(start).as_str(), sh(stop).as_str(), sh(step).as_str()),
        ("1", "false", "true")
    );
}
