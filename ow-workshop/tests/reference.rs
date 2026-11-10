//! Ports of the reference fixtures (`tests/fixtures/reference/`) to `Workshop` impls.
//!
//! Each test builds the equivalent mod in Rust and compares its output to the fixture's expected
//! `.txt`, ignoring whitespace-only lines and CRLF. Until a fixture is ported its test is an
//! ignored stub (#99); #100 and #101 replace the stubs, and #109 then moves the expected output
//! into `insta` snapshots and deletes the fixtures.

// The reference mods are literal translations: one-letter variable names (`A`..`K`) and
// arithmetic on constants (`B * 0`) are the point of the fixtures, and each mod is one long list.
#![allow(
    clippy::many_single_char_names,
    clippy::erasing_op,
    clippy::too_many_lines
)]

mod common;

use common::assert_workshop_code_eq;
use ow_workshop::__private::{Piece, template};
use ow_workshop::Catalog;
use ow_workshop::Kind;
use ow_workshop::prelude::*;
use ow_workshop_data::Extension;
use wasm_bindgen_test::wasm_bindgen_test;

/// An ignored placeholder for a fixture that is not ported yet.
macro_rules! stub {
    ($name:ident, $file:literal, $issue:literal) => {
        #[test]
        #[wasm_bindgen_test]
        #[ignore]
        fn $name() {
            // Keeps the expected output compiled in, so a missing fixture breaks the build.
            let _expected = include_str!(concat!("fixtures/reference/", $file, ".txt"));
            todo!(concat!(
                "port ",
                $file,
                ".md to a Workshop impl (",
                $issue,
                ")"
            ));
        }
    };
}

stub!(enums, "enums", "#101");

/// Size mode, as every fixture in #100 uses.
fn size() -> CompileOptions {
    let mut options = CompileOptions::default();
    options.optimize = OptLevel::Size;
    options
}

/// Drops `//__label_..__:` lines: label comment names are an allowed difference.
fn without_labels(code: &str) -> String {
    let kept: Vec<_> = code
        .lines()
        .filter(|line| {
            !line.trim_start().starts_with("//__label_")
                && !line.trim_start().starts_with("//label_")
        })
        .collect();
    kept.join("\n")
}

/// Compiles `mode` for en-US and compares with the fixture's expected output, after rewriting
/// the expected text with `allowed` (`(from, to)` pairs for documented differences).
#[track_caller]
fn check_with(mode: &impl Workshop, expected: &str, allowed: &[(&str, &str)]) {
    let output = mode.compile(Lang::EnUs).expect("compiles");
    let expected = allowed
        .iter()
        .fold(expected.to_owned(), |text, (from, to)| {
            text.replace(from, to)
        });
    assert_workshop_code_eq(&output.code, &expected);
}

/// Like [`check_with`], with label comment lines ignored on both sides.
#[track_caller]
fn check_ignoring_labels_with(mode: &impl Workshop, expected: &str, allowed: &[(&str, &str)]) {
    let output = mode.compile(Lang::EnUs).expect("compiles");
    let expected = allowed
        .iter()
        .fold(expected.to_owned(), |text, (from, to)| {
            text.replace(from, to)
        });
    // Also allowed here: indentation after a label, and `0` rather than `Null` as the table filler.
    let flat = |text: &str| {
        without_labels(text)
            .lines()
            .map(|line| line.trim_start().replace(", Null,", ", 0,"))
            .collect::<Vec<_>>()
            .join("\n")
    };
    assert_workshop_code_eq(&flat(&output.code), &flat(&expected));
}

#[track_caller]
fn check_ignoring_labels(mode: &impl Workshop, expected: &str) {
    check_ignoring_labels_with(mode, expected, &[]);
}

/// Compares with the fixture's expected output exactly (up to whitespace-only lines).
#[track_caller]
fn check(mode: &impl Workshop, expected: &str) {
    check_with(mode, expected, &[]);
}

// variables: the reference printed `1e10` from source text; we print the f64 (#47).
#[derive(GlobalVars)]
struct VarGlobals {
    #[ow(name = "A", index = 0)]
    a: Global<Num>,
    foo: Global<Num>,
    bar: Global<Num>,
    #[ow(name = "E", index = 4)]
    e: Global<Num>,
}

#[derive(PlayerVars)]
struct VarPlayers {
    #[ow(name = "A", index = 0)]
    a: PlayerVar<Num>,
    foo: PlayerVar<Num>,
    bar: PlayerVar<Num>,
    #[ow(name = "E", index = 4)]
    e: PlayerVar<Num>,
}

struct Variables;

impl Workshop for Variables {
    type Globals = VarGlobals;
    type Players = VarPlayers;

    fn build(&self, ws: &mut Builder, g: &VarGlobals, p: &VarPlayers) {
        ws.rule("variables", OngoingEachPlayer, |r, ev| {
            r.actions(|b| {
                b.set(g.foo, g.bar.get() + g.a.get() + g.e.get());
                let me = ev.event_player();
                b.set(
                    me.var(p.foo),
                    me.var(p.bar).get() + me.var(p.a).get() + me.var(p.e).get(),
                );
            });
        });
        ws.rule("numbers", OngoingGlobal, |r, _| {
            r.actions(|b| {
                for n in [74940.0, -74940.0, -292.0, 74940.0, 292.0, 1e10] {
                    b.set(g.a, n);
                }
            });
        });
    }

    fn options(&self) -> CompileOptions {
        size()
    }
}

#[test]
#[wasm_bindgen_test]
fn variables() {
    check_with(
        &Variables,
        include_str!("fixtures/reference/variables.txt"),
        &[("1e10", "10000000000")],
    );
}

// subroutines
#[derive(GlobalVars)]
struct SubGlobals {
    #[ow(name = "A", index = 0)]
    a: Global<Num>,
    #[ow(name = "B", index = 1)]
    b: Global<Num>,
}

struct Subroutines;

impl Workshop for Subroutines {
    type Globals = SubGlobals;
    type Players = ();

    fn build(&self, ws: &mut Builder, g: &SubGlobals, (): &()) {
        let main_sub =
            ws.declare_subroutine_with("foo", SubroutineOptions::new().rule_name("my subroutine"));
        let bar = ws.declare_subroutine_with("bar", SubroutineOptions::new().index(1));
        // Declared in this order: automatic indices follow declaration order here, where the
        // reference allocated them alphabetically.
        let sub0 = ws.declare_subroutine_with("Sub0", SubroutineOptions::new().index(0));
        let with_wait = ws.declare_subroutine("sub_with_wait");
        let nested = ws.declare_subroutine("sub_with_wait_nested");

        ws.define(main_sub, |b, _| {
            b.add_to(g.a, 1);
            b.call(bar);
            b.start_rule(sub0, StartRuleBehavior::Noop);
            b.start_rule(nested, StartRuleBehavior::Noop);
            b.comment("should throw warning");
            b.start_rule(nested, StartRuleBehavior::Restart);
        });
        ws.define(bar, |b, _| b.pass());
        ws.define(sub0, |b, _| b.add_to(g.b, 1));
        ws.define(with_wait, |b, _| {
            b.set(g.a, 2);
            b.wait(0);
        });
        ws.define(nested, |b, _| {
            b.set(g.a, 3);
            b.call(with_wait);
        });
    }

    fn options(&self) -> CompileOptions {
        size()
    }
}

#[test]
#[wasm_bindgen_test]
fn subroutines() {
    let output = Subroutines.compile(Lang::EnUs).expect("compiles");
    assert!(
        output
            .warnings
            .iter()
            .any(|w| matches!(w.kind, Kind::Warning(Warning::StartRuleCrash))),
        "{:?}",
        output.warnings
    );
    check(
        &Subroutines,
        include_str!("fixtures/reference/subroutines.txt"),
    );
}

// switches: label comment names differ.
#[derive(GlobalVars)]
struct SwitchGlobals {
    #[ow(name = "A", index = 0)]
    a: Global<Num>,
    #[ow(name = "B", index = 1)]
    b: Global<Num>,
    #[ow(name = "C", index = 2)]
    c: Global<Num>,
    #[ow(name = "D", index = 3)]
    d: Global<Num>,
    #[ow(name = "E", index = 4)]
    e: Global<Num>,
}

struct Switches;

impl Workshop for Switches {
    type Globals = SwitchGlobals;
    type Players = ();

    fn build(&self, ws: &mut Builder, g: &SwitchGlobals, (): &()) {
        ws.rule("switches", OngoingGlobal, |r, _| {
            r.actions(|b| {
                b.switch::<Num>(g.a.get())
                    .case(1, |b| {
                        b.add_to(g.b, 1);
                        b.break_();
                    })
                    .case(2, |b| {
                        b.add_to(g.c, 1);
                        b.break_();
                    });
                b.switch::<Any>(g.b.get().erase())
                    .case(Expr::<Num>::from(1.0).erase(), |b| b.add_to(g.b, 1))
                    .case(Expr::<Num>::from(2.0).erase(), |_| {})
                    .case(Expr::<Num>::from(3.0).erase(), |b| b.add_to(g.c, 1))
                    .default(|_| {})
                    .case(IntoExpr::<Heroes>::into_expr(Hero::Brigitte).erase(), |b| {
                        b.add_to(g.d, 1);
                    });
            });
        });
        ws.rule("nested switches", OngoingGlobal, |r, _| {
            r.actions(|b| {
                b.if_(g.a.get().erase().cast::<Bool>(), |b| {
                    b.switch::<Num>(g.b.get())
                        .case(1, |b| {
                            b.add_to(g.c, 1);
                            b.break_();
                        })
                        .case(2, |b| {
                            b.add_to(g.d, 1);
                            b.break_();
                        })
                        .case(3, |b| {
                            b.switch::<Num>(g.c.get())
                                .case(1, |b| {
                                    b.add_to(g.a, 1);
                                    b.add_to(g.b, 1);
                                    b.break_();
                                })
                                .case(2, |b| b.add_to(g.c, 1))
                                .default(|b| {
                                    b.add_to(g.d, 1);
                                });
                        });
                })
                .else_(|b| b.add_to(g.e, 1));
            });
        });
    }

    fn options(&self) -> CompileOptions {
        size()
    }
}

#[test]
#[wasm_bindgen_test]
fn switches() {
    check_ignoring_labels(&Switches, include_str!("fixtures/reference/switches.txt"));
}

/// Removes the named rules from expected output: features of the old text language that have no
/// Rust equivalent (a lone `else`) are an allowed difference.
fn without_rules(expected: &str, names: &[&str]) -> String {
    let mut out = Vec::new();
    let mut skipping = false;
    for line in expected.lines() {
        if let Some(rest) = line.strip_prefix("rule (\"") {
            let name = rest.split("\")").next().unwrap_or_default();
            skipping = names.contains(&name);
        }
        if !skipping {
            out.push(line);
        }
        if skipping && line == "}" {
            skipping = false;
        }
    }
    out.join("\n")
}

// conditions: a lone `else`/`elif` and a second `else` cannot be written with the builder, so
// those three rules are dropped from the expected output.
#[derive(GlobalVars)]
#[allow(dead_code)] // `i` and `j` are used only by the rules left out above.
struct CondGlobals {
    #[ow(name = "A", index = 0)]
    a: Global<Num>,
    #[ow(name = "B", index = 1)]
    b: Global<Num>,
    #[ow(name = "C", index = 2)]
    c: Global<Num>,
    #[ow(name = "D", index = 3)]
    d: Global<Num>,
    #[ow(name = "E", index = 4)]
    e: Global<Num>,
    #[ow(name = "F", index = 5)]
    f: Global<Num>,
    #[ow(name = "G", index = 6)]
    g: Global<Num>,
    #[ow(name = "H", index = 7)]
    h: Global<Num>,
    #[ow(name = "I", index = 8)]
    i: Global<Num>,
    #[ow(name = "J", index = 9)]
    j: Global<Num>,
    #[ow(name = "K", index = 10)]
    k: Global<Num>,
}

/// A number variable used as a condition (the reference treated any value as a condition).
fn truthy(var: Global<Num>) -> Expr<Bool> {
    var.get().erase().cast::<Bool>()
}

struct Conditions;

impl Workshop for Conditions {
    type Globals = CondGlobals;
    type Players = ();

    fn build(&self, ws: &mut Builder, v: &CondGlobals, (): &()) {
        ws.rule("if/else", OngoingGlobal, |r, _| {
            r.actions(|b| {
                b.if_(v.a.get().eq(v.b.get()), |b| b.add_to(v.a, 1))
                    .else_(|b| b.add_to(v.b, 1));
                b.add_to(v.c, 1);
            });
        });
        ws.rule("if/elif/else", OngoingGlobal, |r, _| {
            r.actions(|b| {
                b.if_(v.a.get().eq(v.b.get()), |b| b.add_to(v.a, 1))
                    .elif(v.b.get().eq(v.c.get()), |b| b.add_to(v.b, 1))
                    .elif(v.c.get().eq(v.d.get()), |b| b.add_to(v.c, 1))
                    .else_(|b| b.add_to(v.d, 1));
                b.add_to(v.e, 1);
            });
        });
        ws.rule("nested if/elif/else", OngoingGlobal, |r, _| {
            r.actions(|b| {
                b.if_(v.a.get().eq(v.b.get()), |b| {
                    b.add_to(v.a, 1);
                    b.if_(v.b.get().eq(v.c.get()), |b| b.add_to(v.b, 1))
                        .elif(v.c.get().eq(v.d.get()), |b| b.add_to(v.c, 1))
                        .else_(|b| b.add_to(v.d, 1));
                })
                .elif(v.b.get().eq(v.c.get()), |b| b.add_to(v.b, 1))
                .else_(|b| b.add_to(v.c, 1));
                b.add_to(v.d, 1);
            });
        });
        ws.rule(
            "optimized out empty ifs & if/elif false/true",
            OngoingGlobal,
            |r, _| {
                r.actions(|b| {
                    b.if_(false, |b| {
                        b.set(v.a, 2);
                        b.set(v.a, 2);
                    });
                    b.if_(true, |b| {
                        b.set(v.a, 2);
                        b.set(v.a, 2);
                    });
                    b.set(v.b, 3);
                    b.if_(false, |b| b.set(v.b, 5)).else_(|b| {
                        b.set(v.c, 6);
                        b.if_(true, |b| b.add_to(v.d, 1))
                            .else_(|b| b.add_to(v.e, 1));
                    });

                    b.if_(true, |b| b.set(v.a, 2))
                        .elif(truthy(v.a), |b| b.set(v.a, 3));
                    b.set(v.b, 4);
                    b.if_(false, |b| b.set(v.b, 5))
                        .elif(truthy(v.a), |b| b.set(v.b, 6))
                        .elif(false, |b| b.add_to(v.h, 1))
                        .else_(|b| b.add_to(v.k, 1));

                    b.if_(false, |b| b.add_to(v.b, 1))
                        .elif(false, |b| b.add_to(v.c, 1))
                        .elif(true, |b| b.add_to(v.d, 1));

                    b.if_(truthy(v.a), |b| b.add_to(v.b, 1))
                        .elif(truthy(v.c), |b| b.add_to(v.d, 1))
                        .elif(true, |b| b.add_to(v.e, 1))
                        .elif(truthy(v.f), |b| b.add_to(v.g, 1));

                    b.set(v.c, 4);
                    b.if_(false, |b| b.set(v.b, 4));
                    b.if_(false, |_| {});
                    b.set(v.a, 3);
                    b.if_(truthy(v.g), |_| {});
                });
            },
        );
        ws.rule(
            "optimized out empty if at end of rule",
            OngoingGlobal,
            |r, _| {
                r.actions(|b| {
                    b.add_to(v.a, 1);
                    b.if_(truthy(v.b), |b| b.set(v.a, v.a.get()));
                });
            },
        );
        ws.rule(
            "optimized out elif false at end of rule",
            OngoingGlobal,
            |r, _| {
                r.actions(|b| {
                    b.if_(false, |b| b.set(v.a, 1))
                        .elif(false, |b| b.set(v.a, 2));
                });
            },
        );
    }

    fn options(&self) -> CompileOptions {
        size()
    }
}

#[test]
#[wasm_bindgen_test]
fn conditions() {
    let expected = without_rules(
        include_str!("fixtures/reference/conditions.txt"),
        &["lone else", "lone elif/else", "if/elif multiple elses"],
    );
    // The goto/label section of the big rule is also left out: the reference keeps `If(True)`
    // around a body a skip lands in, but folds the other `if true`s of a rule that has skips,
    // which this optimizer deliberately does not (`a_true_if_stays_when_the_rule_has_skips`).
    let goto_part = "        Skip(3);\n        If(True);\n            Set Global Variable(A, 2);\n        Else If(Global.A);\n            //lbl_1:\n            Set Global Variable(A, 3);\n        End;\n        Set Global Variable(B, 3);\n        Skip(True);\n        If(False);\n            //lbl_2:\n            Set Global Variable(B, 6);\n        End;\n        Set Global Variable(B, 9);\n";
    assert!(expected.contains(goto_part));
    check_with(&Conditions, &expected, &[(goto_part, "")]);
}

/// Allowed difference: `B == 0` (either way round) becomes `Not(B)`, one element fewer.
const EQ_ZERO_AS_NOT: [(&str, &str); 2] = [
    ("Compare(Global.B, ==, Null)", "Not(Global.B)"),
    ("Compare(Null, ==, Global.B)", "Not(Global.B)"),
];

/// Keeps only the named rules (and everything before the first rule) of expected output.
fn only_rules(expected: &str, names: &[&str]) -> String {
    let mut out = Vec::new();
    let mut keeping = true;
    for line in expected.lines() {
        if let Some(rest) = line.strip_prefix("rule (\"") {
            let name = rest.split("\")").next().unwrap_or_default();
            keeping = names.contains(&name);
        }
        if keeping {
            out.push(line);
        }
    }
    out.join("\n")
}

/// Assigns any value to a number variable: the reference variables are untyped.
fn put<T: ValueType>(b: &mut Block, var: Global<Num>, value: Expr<T>) {
    b.set(var, value.erase().cast::<Num>());
}

/// `put` with the source line's trailing comment printed above it.
fn put_c<T: ValueType>(b: &mut Block, var: Global<Num>, note: &str, value: Expr<T>) {
    b.comment(note);
    put(b, var, value);
}

// operators: the rule `strict optimizations` needs strict mode, so it compiles on its own; the
// per-rule `#!optimizeStrict` override has no public equivalent. Numbers print from `f64`.
#[derive(GlobalVars)]
struct OpGlobals {
    #[ow(name = "A", index = 0)]
    a: Global<Num>,
    #[ow(name = "B", index = 1)]
    b: Global<Num>,
    #[ow(name = "C", index = 2)]
    c: Global<Num>,
    #[ow(name = "D", index = 3)]
    d: Global<Num>,
}

#[derive(PlayerVars)]
#[allow(dead_code)] // declared so the variables block matches the fixture
struct OpPlayers {
    #[ow(name = "A", index = 0)]
    a: PlayerVar<Num>,
}

struct OperatorsStrict;

impl Workshop for OperatorsStrict {
    type Globals = OpGlobals;
    type Players = OpPlayers;

    fn build(&self, ws: &mut Builder, g: &OpGlobals, _: &OpPlayers) {
        let b = || g.b.get();
        ws.rule("strict optimizations", OngoingGlobal, |r, _| {
            r.actions(|bl| {
                bl.comment("Unless specified otherwise, none of these should be optimized");
                put(bl, g.a, Expr::<Str>::from("am").eq("**"));
                put(bl, g.a, b() + 0);
                put(bl, g.a, b().erase().cast::<Vector>() + vect(0, 0, 0));
                put(bl, g.a, b() - b());
                put(bl, g.a, b() * 0);
                put_c(bl, g.a, "0", len(b().erase().cast::<Array<Any>>()) * 0);
                put(bl, g.a, b() * 1);
                put(bl, g.a, b() / 1);
                put(bl, g.a, Expr::<Num>::from(0) / b());
                put(bl, g.a, b().and(false));
                put(bl, g.a, b().and(true));
                put(bl, g.a, b().and(!b().erase().cast::<Bool>()));
                put(bl, g.a, b().or(false));
                put(bl, g.a, b().or(true));
                put(bl, g.a, b().or(!b().erase().cast::<Bool>()));
            });
        });
    }

    fn options(&self) -> CompileOptions {
        let mut options = size();
        options.strict = true;
        options
    }
}

/// A number as a condition.
fn truth(e: Expr<Num>) -> Expr<Bool> {
    e.erase().cast::<Bool>()
}

/// A number as a vector.
fn as_vec(e: Expr<Num>) -> Expr<Vector> {
    e.erase().cast::<Vector>()
}

fn n(x: i32) -> Expr<Num> {
    Expr::from(x)
}

struct Operators;

impl Workshop for Operators {
    type Globals = OpGlobals;
    type Players = OpPlayers;

    fn build(&self, ws: &mut Builder, g: &OpGlobals, _: &OpPlayers) {
        let (a, b, c, d) = (|| g.a.get(), || g.b.get(), || g.c.get(), || g.d.get());
        let hero = || IntoExpr::<Heroes>::into_expr(Hero::Ana).erase();
        ws.rule("comparison", OngoingGlobal, |r, _| {
            r.actions(|x| {
                put(x, g.a, b().eq(c()));
                put_c(x, g.a, "false", n(1).eq(0));
                put_c(x, g.a, "true", b().eq(b()));
                put_c(x, g.a, "not B", b().eq(0));
                put_c(x, g.a, "isGameInProgress()", is_game_in_progress().eq(true));
                put_c(
                    x,
                    g.a,
                    "false",
                    hero().eq(Expr::<Map>::from(Map::KingsRow).erase()),
                );
                put_c(
                    x,
                    g.a,
                    "should not be optimized",
                    hero().eq(ws_fmt!("azer{}", a()).erase()),
                );

                put(x, g.a, b().ne(c()));
                put_c(x, g.a, "true", n(1).ne(0));
                put_c(x, g.a, "false", b().ne(b()));
                put(x, g.a, b().ne(0));
                put_c(x, g.a, "not B", !b().ne(0));
                put_c(
                    x,
                    g.a,
                    "not isGameInProgress()",
                    is_game_in_progress().ne(true),
                );
                put_c(
                    x,
                    g.a,
                    "true",
                    hero().ne(Expr::<Map>::from(Map::KingsRow).erase()),
                );
                put_c(
                    x,
                    g.a,
                    "should not be optimized",
                    hero().ne(ws_fmt!("azer{}", a()).erase()),
                );

                put(x, g.a, b().ge(c()));
                put_c(x, g.a, "true", n(1).ge(0));
                put_c(x, g.a, "true", b().ge(b()));
                put(x, g.a, b().le(c()));
                put_c(x, g.a, "false", n(1).le(0));
                put_c(x, g.a, "true", b().le(b()));
                put(x, g.a, b().gt(c()));
                put_c(x, g.a, "true", n(1).gt(0));
                put_c(x, g.a, "false", b().gt(b()));
                put(x, g.a, b().lt(c()));
                put_c(x, g.a, "false", n(1).lt(0));
                put_c(x, g.a, "false", b().lt(b()));
            });
        });
        ws.rule("add", OngoingGlobal, |r, _| {
            r.actions(|x| {
                put(x, g.a, b() + c());
                put_c(x, g.a, "B", b() + 0);
                put_c(x, g.a, "B*2", b() + b());
                put_c(x, g.a, "3", n(1) + 2);
                put_c(x, g.a, "B", as_vec(b()) + vect(0, 0, 0));
                put_c(x, g.a, "B", vect(0, 0, 0) + as_vec(b()));
                put(x, g.a, vect(b(), c(), d()) + vect(1, 2, 3));
                put_c(x, g.a, "vect(5,7,9)", vect(1, 2, 3) + vect(4, 5, 6));
                put_c(x, g.a, "left + right", vect(0, 0, 0));
                put_c(x, g.a, "up", vect(0, 1, 0));
                put_c(
                    x,
                    g.b,
                    "vect(1, -1, -1)",
                    vector_left() + vector_down() + vector_backward(),
                );
                put_c(
                    x,
                    g.c,
                    "vect(-1, 1, 1)",
                    vector_right() + vector_up() + vector_forward(),
                );
            });
        });
        ws.rule("subtract", OngoingGlobal, |r, _| {
            r.actions(|x| {
                put(x, g.a, b() - c());
                put(x, g.a, b() - c() - d());
                put_c(x, g.a, "B", b() - 0);
                put_c(x, g.a, "0", b() - b());
                put_c(x, g.a, "-1", n(1) - 2);
                put_c(x, g.a, "B", as_vec(b()) - vect(0, 0, 0));
                put(x, g.a, vect(b(), c(), d()) - vect(1, 2, 3));
                put_c(x, g.a, "vect(-3,-3,-3)", vect(1, 2, 3) - vect(4, 5, 6));
            });
        });
        ws.rule("negate", OngoingGlobal, |r, _| {
            r.actions(|x| {
                put(x, g.a, -b());
                put(x, g.a, -b() % c());
                put(x, g.a, -vect(1, b(), -2));
                put_c(x, g.a, "vect(-1,-2,3)", -vect(1, 2, -3));
                put(x, g.a, n(-1));
                put(x, g.a, n(-1) * b());
                put_c(x, g.a, "B", -(-b()));
                put_c(x, g.a, "1", -(-n(1)));
                put_c(x, g.a, "B", -(-b()));
                put_c(x, g.a, "-1", -(-(-n(1))));
                put_c(x, g.a, "-B", -(-(-b())));
                put_c(x, g.a, "0.01", -(-Expr::<Num>::from(0.01)));
                put(x, g.a, b() + -c());
                put(x, g.a, !truth(-c()));
                put(x, g.a, b() * -c());
            });
        });
        ws.rule("multiply", OngoingGlobal, |r, _| {
            r.actions(|x| {
                put(x, g.a, b() * c());
                put_c(x, g.a, "B", b() * 1);
                put_c(x, g.a, "B**2", b() * b());
                put_c(x, g.a, "0", b() * 0);
                put_c(x, g.a, "6", n(2) * 3);
                put_c(x, g.a, "vect(6,8,10)", n(2) * vect(3, 4, 5));
                put_c(x, g.a, "vect(6,8,10)", vect(3, 4, 5) * 2);
                put_c(x, g.a, "vect(4,10,18)", vect(1, 2, 3) * vect(4, 5, 6));
            });
        });
        ws.rule("divide", OngoingGlobal, |r, _| {
            r.actions(|x| {
                put(x, g.a, b() / c());
                put_c(x, g.a, "B", b() / 1);
                put_c(x, g.a, "0", b() / 0);
                put_c(x, g.a, "0", n(0) / b());
                put_c(x, g.a, "0.5", n(1) / 2);
                put(x, g.a, n(-3) / b());
                put_c(x, g.a, "vect(3,1.5,1)", vect(3, 3, 3) / vect(1, 2, 3));
                put_c(x, g.a, "vect(0.5,1,1.5)", vect(1, 2, 3) / 2);
            });
        });
        ws.rule("modulo", OngoingGlobal, |r, _| {
            r.actions(|x| {
                put(x, g.a, b() % c());
                put_c(x, g.a, "0", b() % 0);
                put_c(x, g.a, "0", n(0) % b());
                put_c(x, g.a, "0", b() % b());
                put_c(x, g.a, "1", n(7) % 3);
                put_c(x, g.a, "-1", n(-7) % 3);
                put_c(x, g.a, "-1", n(-7) % -3);
                put_c(x, g.a, "1", n(7) % -3);
            });
        });
        ws.rule("power", OngoingGlobal, |r, _| {
            r.actions(|x| {
                put(x, g.a, b().pow(c()));
                put(x, g.a, b().pow(c().pow(d())));
                put_c(x, g.a, "B", b().pow(1));
                put_c(x, g.a, "0", n(0).pow(b()));
                put_c(x, g.a, "1", n(1).pow(b()));
                put_c(x, g.a, "0", n(-1).pow(b()));
                put_c(x, g.a, "8", n(2).pow(3));
                put_c(x, g.a, "0", n(-2).pow(3));
            });
        });
        ws.rule("and", OngoingGlobal, |r, _| {
            r.actions(|x| {
                put(x, g.a, a().and(b()));
                put_c(x, g.a, "B", b().and(true));
                put_c(x, g.a, "false", b().and(false));
                put_c(x, g.a, "B", b().and(b()));
                put_c(x, g.a, "false", b().and(!truth(b())));
                put_c(x, g.a, "not (B or C)", (!truth(b())).and(!truth(c())));
            });
        });
        ws.rule("or", OngoingGlobal, |r, _| {
            r.actions(|x| {
                put(x, g.a, a().or(b()));
                put_c(x, g.a, "B", b().or(b()));
                put_c(x, g.a, "true", a().or(true));
                put_c(x, g.a, "B", b().or(false));
                put_c(x, g.a, "true", b().or(!truth(b())));
                put_c(x, g.a, "not (B and C)", (!truth(b())).or(!truth(c())));
            });
        });
        ws.rule("not", OngoingGlobal, |r, _| {
            r.actions(|x| {
                put(x, g.a, !truth(b()));
                put_c(x, g.a, "not not B", !!truth(b()));
                put_c(x, g.a, "B == C", !!b().eq(c()));
                put_c(x, g.a, "false", !Expr::<Bool>::from(true));
                put_c(x, g.a, "true", !Expr::<Bool>::from(false));
                put(x, g.a, n(3) * (!truth(b())).erase().cast::<Num>());
                put_c(x, g.a, "B != C", !b().eq(c()));
                put_c(x, g.a, "B == C", !b().ne(c()));
                put_c(x, g.a, "B > C", !b().le(c()));
                put_c(x, g.a, "B < C", !b().ge(c()));
                put_c(x, g.a, "B >= C", !b().lt(c()));
                put_c(x, g.a, "B <= C", !b().gt(c()));
            });
        });
        ws.rule("in", OngoingGlobal, |r, _| {
            r.actions(|x| {
                put(x, g.a, b().erase().cast::<Array<Any>>().contains(a()));
                put_c(
                    x,
                    g.a,
                    "true",
                    Expr::<Array<Any>>::from(vec![n(1).erase(), a().erase(), n(3).erase()])
                        .contains(3),
                );
                put(
                    x,
                    g.a,
                    Expr::<Array<Any>>::from(vec![a().erase(), n(5).erase(), n(6).erase()])
                        .contains(3),
                );
                put(x, g.a, !c().erase().cast::<Array<Any>>().contains(b()));
            });
        });
        ws.rule("ternaries", OngoingGlobal, |r, _| {
            r.actions(|x| {
                put(x, g.a, if_then_else(truth(c()), b(), d()));
                put_c(x, g.a, "B", if_then_else(true, b(), d()));
                put_c(x, g.a, "D", if_then_else(false, b(), d()));
                put_c(x, g.a, "B", if_then_else(truth(c()), b(), b()));
                put_c(
                    x,
                    g.a,
                    "if-then-else(C, D, B)",
                    if_then_else(!truth(c()), b(), d()),
                );
                put_c(
                    x,
                    g.a,
                    "D",
                    if_then_else(false, b(), if_then_else(false, c(), d())),
                );
            });
        });
    }

    fn options(&self) -> CompileOptions {
        size()
    }
}

#[test]
#[wasm_bindgen_test]
fn operators_strict() {
    let expected = only_rules(
        include_str!("fixtures/reference/operators.txt"),
        &["strict optimizations"],
    );
    // The strict header is printed before the rule (strict is a whole-mod option here) rather
    // than inside its actions.
    check_with(
        &OperatorsStrict,
        &expected,
        &[
            ("        //Strict optimizations enabled\n", ""),
            (
                "//Optimize for size enabled\n",
                "//Optimize for size enabled\n//Strict optimizations enabled\n",
            ),
        ],
    );
}

const OPERATOR_RULES: &[&str] = &[
    "comparison",
    "add",
    "subtract",
    "negate",
    "multiply",
    "divide",
    "modulo",
    "power",
    "and",
    "or",
    "not",
    "in",
    "ternaries",
];

// operators, assignment: `A` is indexed here, so it is an array variable in this mod.
#[derive(GlobalVars)]
struct AssignGlobals {
    #[ow(name = "A", index = 0)]
    a: Global<Array<Num>>,
    #[ow(name = "B", index = 1)]
    b: Global<Num>,
}

#[derive(PlayerVars)]
struct AssignPlayers {
    #[ow(name = "A", index = 0)]
    a: PlayerVar<Array<Num>>,
}

/// A number where the variable is typed as an array.
fn arr(e: Expr<Num>) -> Expr<Array<Num>> {
    e.erase().cast::<Array<Num>>()
}

struct OperatorsAssign;

impl Workshop for OperatorsAssign {
    type Globals = AssignGlobals;
    type Players = AssignPlayers;

    fn build(&self, ws: &mut Builder, g: &AssignGlobals, p: &AssignPlayers) {
        let b = || arr(g.b.get());
        ws.rule("assignment", OngoingEachPlayer, |r, ev| {
            r.actions(|x| {
                let me = ev.event_player();
                x.set(g.a, b());
                x.set(g.a.at(0), g.b.get());
                x.set(me.var(p.a), b());
                x.set(me.var(p.a).at(0), g.b.get());
                x.set(g.a, g.a.get());
                x.add_to(g.a, b());
                x.add_to(me.var(p.a), b());
                x.add_to(g.a, arr(n(0)));
                x.sub_from(g.a, b());
                x.mul_by(g.a, b());
                x.div_by(g.a, b());
                x.mod_by(g.a, b());
                x.pow_by(g.a, b());
                x.max_with(g.a, b());
                x.min_with(g.a, b());
                x.add_to(g.a, arr(n(1)));
                x.sub_from(g.a, arr(n(1)));
            });
        });
        ws.rule("handling random values", OngoingGlobal, |r, _| {
            let pick = || random::choice(all_players()).erase().cast::<Player>();
            r.actions(|x| {
                x.add_to(pick().var(p.a), arr(n(0)));
                x.add_to(pick().var(p.a), arr(n(1)));
                let current = pick().var(p.a).get().erase().cast::<Num>();
                x.set(pick().var(p.a), arr(current + 1));
                x.max_with(pick().var(p.a), b());
            });
        });
    }

    fn options(&self) -> CompileOptions {
        size()
    }
}

#[test]
#[wasm_bindgen_test]
fn operators_assign() {
    let expected = only_rules(
        include_str!("fixtures/reference/operators.txt"),
        &["assignment", "handling random values"],
    );
    // This mod never declares C and D, which the full fixture lists.
    check_with(
        &OperatorsAssign,
        &expected,
        &[("        2: C\n        3: D\n", "")],
    );
}

#[test]
#[wasm_bindgen_test]
fn operators() {
    let expected = only_rules(
        include_str!("fixtures/reference/operators.txt"),
        OPERATOR_RULES,
    );
    // Allowed differences, all in the optimizer: `B == 0` becomes `Not(B)` (one element
    // fewer than comparing with `Null`), `-vect(1, ..)` keeps its literal `1` instead of
    // `True`, and `(-1) ** B` folds to `0` (a negative base gives 0 in the workshop).
    check_with(
        &Operators,
        &expected,
        &[
            ("Compare(Global.B, ==, Null)", "Not(Global.B)"),
            (
                "Multiply(-1, Vector(True, Global.B, -2))",
                "Multiply(-1, Vector(1, Global.B, -2))",
            ),
            ("Raise To Power(-1, Global.B)", "Null"),
        ],
    );
}

// elementSaving: the reference variables are untyped, but `A` is both used as a loop counter and
// indexed. `A` is a number here; the indexed assignments (`A[0] = B`) are built by a second mod
// where it is an array, and compared line by line with the fixture.
#[derive(GlobalVars)]
struct EsGlobals {
    #[ow(name = "A", index = 0)]
    a: Global<Num>,
    #[ow(name = "B", index = 1)]
    b: Global<Num>,
    #[ow(name = "C", index = 2)]
    c: Global<Num>,
    #[ow(name = "D", index = 3)]
    d: Global<Num>,
    #[ow(name = "E", index = 4)]
    e: Global<Num>,
}

#[derive(PlayerVars)]
struct EsPlayers {
    #[ow(name = "A", index = 0)]
    a: PlayerVar<Num>,
}

/// Assigns any value to `A`.
fn set_a<T: ValueType>(x: &mut Block, g: &EsGlobals, value: Expr<T>) {
    x.set(g.a, value.erase().cast::<Num>());
}

/// Reads a number variable as another type.
fn as_<T: ValueType>(e: Expr<Num>) -> Expr<T> {
    e.erase().cast::<T>()
}

struct ElementSaving;

impl Workshop for ElementSaving {
    type Globals = EsGlobals;
    type Players = EsPlayers;

    fn build(&self, ws: &mut Builder, g: &EsGlobals, p: &EsPlayers) {
        let (a, b, c) = (|| g.a.get(), || g.b.get(), || g.c.get());
        let player_b = || as_::<Player>(g.b.get());
        let any = |v: i32| Expr::<Num>::from(v).erase();
        let hud = |value: i32| HudArgs {
            location: HudPosition::Left.into(),
            sort_order: n(value),
            color: Color::White.into(),
            reevaluation: HudReeval::Visibility.into(),
            spec_visibility: SpecVisibility::Always.into(),
            ..HudArgs::default()
        };
        ws.rule("0 to null", OngoingGlobal, |r, _| {
            r.actions(|x| {
                set_a(x, g, b().gt(0));
                set_a(x, g, n(0).gt(b()));
                set_a(x, g, b().ge(0));
                set_a(x, g, n(0).ge(b()));
                set_a(x, g, b().lt(0));
                set_a(x, g, n(0).lt(b()));
                set_a(x, g, b().le(0));
                set_a(x, g, n(0).le(b()));
                set_a(x, g, ArrayExt::concat(&as_::<Array<Any>>(b()), 0));
                set_a(x, g, Expr::<Array<Any>>::from(vec![any(0), any(1), any(0)]));
                set_a(x, g, as_::<Array<Any>>(b()).contains(0));
                set_a(x, g, b().eq(0));
                set_a(x, g, n(0).eq(b()));
                set_a(x, g, b().ne(0));
                set_a(x, g, n(0).ne(b()));
                set_a(x, g, ws_fmt!("{}{}{}", 0, 0, 0));
                set_a(x, g, localized_en("{0}, {1}, and {2}", [0, 0, 0]));
                x.big_message(0);
                set_a(x, g, n(0));
                x.set(player_b().var(p.a), 0);
                x.small_message(0);
            });
        });
        ws.rule("0 to false", OngoingGlobal, |r, _| {
            r.actions(|x| {
                set_a(x, g, max(0, b()));
                set_a(x, g, max(b(), 0));
                set_a(x, g, min(0, b()));
                set_a(x, g, min(b(), 0));
                set_a(x, g, random::int(0, b()));
                set_a(x, g, random::int(b(), 0));
                set_a(x, g, n(0) - b());
                set_a(x, g, vect(0, 0, 0));
                x.chase_at_rate(g.a, 0, 0, ChaseRateReeval::None);
                x.chase_over_time(g.a, 0, 0, ChaseTimeReeval::None);
                x.chase_at_rate(player_b().var(p.a), 0, 0, ChaseRateReeval::None);
                x.chase_over_time(player_b().var(p.a), 0, 0, ChaseTimeReeval::None);
                let facing = as_::<Direction>(c());
                x.create_dummy_with(
                    Hero::Ana,
                    Team::Two,
                    0,
                    as_::<Position>(b()),
                    CreateDummyArgs { facing },
                );
                x.hud_header_with("123", hud(0));
                x.for_range(g.a, range(0, 0).step(0), |_| {});
                x.for_range(player_b().var(p.a), range(0, 0).step(0), |_| {});
                x.wait(0);
                x.wait(0.016);
                x.wait(0);
            });
        });
        ws.rule("1 to true", OngoingGlobal, |r, _| {
            r.condition(b().gt(1))
                .comment("should not be changed, inequalities with true/false no longer work");
            r.condition(n(1).gt(b()));
            r.condition(b().ge(1));
            r.condition(n(1).ge(b()));
            r.condition(b().lt(1));
            r.condition(n(1).lt(b()));
            r.condition(b().le(1));
            r.condition(n(1).le(b()));
            r.actions(|x| {
                set_a(x, g, b().gt(1));
                set_a(x, g, n(1).gt(b()));
                set_a(x, g, b().ge(1));
                set_a(x, g, n(1).ge(b()));
                set_a(x, g, b().lt(1));
                set_a(x, g, n(1).lt(b()));
                set_a(x, g, b().le(1));
                set_a(x, g, n(1).le(b()));
                set_a(x, g, b() + 1);
                set_a(x, g, n(1) + b());
                set_a(x, g, n(1) / b());
                set_a(x, g, max(1, b()));
                set_a(x, g, max(b(), 1));
                set_a(x, g, min(1, b()));
                set_a(x, g, min(b(), 1));
                set_a(x, g, random::int(1, b()));
                set_a(x, g, random::int(b(), 1));
                set_a(x, g, n(1) - b());
                set_a(x, g, b() - 1);
                set_a(x, g, as_::<Array<Any>>(b()).get(1));
                set_a(x, g, vect(1, 1, 1));
                x.chase_at_rate(g.a, 1, 1, ChaseRateReeval::None);
                x.chase_over_time(g.a, 1, 1, ChaseTimeReeval::None);
                x.chase_at_rate(player_b().var(p.a), 1, 1, ChaseRateReeval::None);
                x.chase_over_time(player_b().var(p.a), 1, 1, ChaseTimeReeval::None);
                let facing = as_::<Direction>(c());
                x.create_dummy_with(
                    Hero::Ana,
                    Team::Two,
                    1,
                    as_::<Position>(b()),
                    CreateDummyArgs { facing },
                );
                x.hud_header_with("123", hud(1));
                x.for_range(g.a, range(1, 1).step(1), |_| {});
                x.for_range(player_b().var(p.a), range(1, 1).step(1), |_| {});
                x.wait(1);
            });
        });
        ws.rule("null vector to null", OngoingGlobal, |r, _| {
            r.actions(|x| {
                let facing = as_::<Direction>(vect(0, 0, 0).erase().cast::<Num>());
                x.create_dummy_with(
                    Hero::Ana,
                    Team::Two,
                    a(),
                    vect(0, 0, 0),
                    CreateDummyArgs { facing },
                );
            });
        });
        ws.rule("empty string", OngoingGlobal, |r, _| {
            let (bs, cs) = (|| as_::<Str>(b()), || as_::<Str>(c()));
            let empty = || Expr::<Str>::from("");
            r.actions(|x| {
                x.hud_text_with(HudTextArgs {
                    header: a().erase(),
                    subheader: empty().erase(),
                    text: empty().erase(),
                    ..HudTextArgs::default()
                });
                set_a(x, g, empty().find(bs()));
                set_a(x, g, bs().find(""));
                set_a(x, g, empty().char_at(b()));
                set_a(x, g, empty().replace(bs(), cs()));
                set_a(x, g, bs().replace("", cs()));
                set_a(x, g, bs().replace(cs(), ""));
                set_a(x, g, bs().split(""));
                set_a(x, g, empty().split(bs()));
                set_a(
                    x,
                    g,
                    empty().slice_with(
                        b(),
                        SubstringArgs {
                            substring_length: c(),
                        },
                    ),
                );
                set_a(x, g, str_contains("", bs()));
                set_a(x, g, str_contains(bs(), ""));
                set_a(x, g, empty());
            });
        });
        ws.rule("if to skip if", OngoingGlobal, |r, _| {
            r.actions(|x| {
                x.comment("should be skip if");
                x.if_(a().eq(b()), |x| x.set(g.d, 4));
                x.comment("should stay if");
                x.if_(a().eq(b()), |x| {
                    x.set(g.d, 4);
                    x.set(g.e, 5);
                });
                x.comment("should be skip if");
                x.if_(!as_::<Bool>(b()), |x| {
                    x.set(g.d, 4);
                    x.set(g.e, 5);
                });
                x.if_(a().eq(2), |x| {
                    x.if_(a().eq(3), |x| x.set(g.b, 4));
                    x.switch::<Num>(a()).case(b(), |x| x.add_to(g.c, 1));
                    x.add_to(g.d, 1);
                    x.add_to(g.e, 1);
                })
                .elif(false, |x| x.set(g.b, 3));
                x.comment("should stay if");
                x.if_(a().eq(b()), |x| x.set(g.d, 4));
            });
        });
        ws.rule("auto array compression", OngoingGlobal, |r, _| {
            let items = |v: Vec<Expr<Num>>| Expr::<Array<Num>>::from(v);
            r.actions(|x| {
                x.comment("shouldn't be compressed");
                set_a(
                    x,
                    g,
                    items(vec![b(), b(), b(), b(), b(), b(), b(), b(), c()]),
                );
                x.comment("shouldn't be compressed, can't nest mapped arrays");
                set_a(
                    x,
                    g,
                    as_::<Array<Num>>(b()).map(|e, _| {
                        e.erase().cast::<Vector>() + items(vec![c(); 7]).erase().cast::<Vector>()
                    }),
                );
                x.comment("should be compressed");
                set_a(x, g, items(vec![b(); 9]));
                set_a(
                    x,
                    g,
                    IntoExpr::<Array<Str>>::into_expr(["abc", "def", "ghi"]),
                );
                x.comment("shouldn't be compressed");
                let tail = ws_fmt!("ghi{}", b());
                set_a(
                    x,
                    g,
                    Expr::<Array<Str>>::from(vec![
                        Expr::<Str>::from("abc"),
                        Expr::<Str>::from("def"),
                        tail,
                    ]),
                );
            });
        });
    }

    fn options(&self) -> CompileOptions {
        let mut options = size();
        options.aggressive_size = true;
        options.suppress.insert(Warning::ChasedVarInFor);
        options
    }
}

// elementSaving, indexed assignments: `A` is an array here.
#[derive(GlobalVars)]
struct EsIndexGlobals {
    #[ow(name = "A", index = 0)]
    a: Global<Array<Any>>,
    #[ow(name = "B", index = 1)]
    b: Global<Num>,
    #[ow(name = "C", index = 2)]
    c: Global<Num>,
}

#[derive(PlayerVars)]
struct EsIndexPlayers {
    #[ow(name = "A", index = 0)]
    a: PlayerVar<Array<Any>>,
}

struct ElementSavingIndex;

impl Workshop for ElementSavingIndex {
    type Globals = EsIndexGlobals;
    type Players = EsIndexPlayers;

    fn build(&self, ws: &mut Builder, g: &EsIndexGlobals, p: &EsIndexPlayers) {
        let player_b = || as_::<Player>(g.b.get());
        for (name, index) in [("0 to null", 0), ("0 to false", 0), ("1 to true", 1)] {
            ws.rule(name, OngoingGlobal, |r, _| {
                r.actions(|x| {
                    if name == "0 to null" {
                        x.set(g.a.at(g.b.get()), 0);
                        x.set(player_b().var(p.a).at(g.c.get()), 0);
                    } else {
                        x.set(g.a.at(index), g.b.get());
                        x.set(player_b().var(p.a).at(index), g.c.get());
                    }
                });
            });
        }
    }

    fn options(&self) -> CompileOptions {
        let mut options = size();
        options.aggressive_size = true;
        options
    }
}

/// The lines of `code` that contain `needle`, trimmed.
fn lines_containing(code: &str, needle: &str) -> Vec<String> {
    code.lines()
        .filter(|line| line.contains(needle))
        .map(|line| line.trim().to_owned())
        .collect()
}

#[test]
#[wasm_bindgen_test]
fn elementsaving() {
    let fixture = include_str!("fixtures/reference/elementSaving.txt");
    let exact = [
        "0 to null",
        "0 to false",
        "1 to true",
        "null vector to null",
        "empty string",
    ];
    // The indexed assignments come from the second mod.
    let without_index = |code: &str| -> String {
        code.lines()
            .filter(|line| !line.contains("At Index("))
            .collect::<Vec<_>>()
            .join("\n")
    };
    let output = ElementSaving.compile(Lang::EnUs).expect("compiles").code;
    let rewritten = EQ_ZERO_AS_NOT
        .iter()
        .fold(without_index(&only_rules(fixture, &exact)), |t, (a, b)| {
            t.replace(a, b)
        });
    assert_workshop_code_eq(&only_rules(&output, &exact), &rewritten);

    let indexed = ElementSavingIndex.compile(Lang::EnUs).expect("compiles");
    assert_eq!(
        lines_containing(&indexed.code, "At Index("),
        lines_containing(fixture, "At Index(")
    );

    // `if to skip if`: label names differ, and so do two aggressive choices. A skip lands on a
    // `Pass` of its own (the reference lets it land on the next action), and a lone-statement
    // `if` at the end of a rule is a `Skip If` here, where the reference keeps the `If`.
    let skip_rule = ["if to skip if"];
    let expected = only_rules(fixture, &skip_rule);
    let mut expected = expected
        .replace(
            "//__label_if_2__:\n",
            "//__label_if_2__:\n\"pass\"\ndisabled Abort;\n",
        )
        .replace(
            "//__label_if_3__:\n",
            "//__label_if_3__:\n\"pass\"\ndisabled Abort;\n",
        )
        .replace(
            "//__label_if_4__:\n",
            "//__label_if_4__:\n\"pass\"\ndisabled Abort;\n",
        );
    let last_if = "        \"should stay if\"\n        If(Compare(Global.A, ==, Global.B));\n            Set Global Variable(D, 4);\n    }";
    assert!(expected.contains(last_if));
    expected = expected.replace(
        last_if,
        "        \"should stay if\"\n        Skip If(Compare(Global.A, !=, Global.B), True);\n        Set Global Variable(D, 4);\n        //label_5:\n        \"pass\"\n        disabled Abort;\n    }",
    );
    let flat = |text: &str| {
        without_labels(text)
            .lines()
            .map(|line| {
                line.trim_start()
                    .replace(", Null,", ", 0,")
                    .replace("(1, Null)", "(1, 0)")
            })
            .collect::<Vec<_>>()
            .join("\n")
    };
    assert_workshop_code_eq(&flat(&only_rules(&output, &skip_rule)), &flat(&expected));

    // `auto array compression`: only the first two statements compress identically. This
    // optimizer leaves a repeated variable and a three-string array as plain arrays.
    let compression = ["auto array compression"];
    let expected = only_rules(fixture, &compression)
        .replace(
            "Mapped Array(String Split(Custom String(\"00000000\"), First Of(Null)), Global.B)",
            "Array(Global.B, Global.B, Global.B, Global.B, Global.B, Global.B, Global.B, Global.B, Global.B)",
        )
        .replace(
            "String Split(Custom String(\"abc0def0ghi\"), First Of(Null))",
            "Array(Custom String(\"abc\"), Custom String(\"def\"), Custom String(\"ghi\"))",
        );
    assert_workshop_code_eq(&only_rules(&output, &compression), &expected);
}

// dicts
#[derive(GlobalVars)]
struct DictGlobals {
    #[ow(name = "A", index = 0)]
    a: Global<Num>,
    #[ow(name = "B", index = 1)]
    b: Global<Num>,
}

struct Dicts;

impl Workshop for Dicts {
    type Globals = DictGlobals;
    type Players = ();

    fn build(&self, ws: &mut Builder, g: &DictGlobals, (): &()) {
        ws.rule("dicts", OngoingGlobal, |r, _| {
            r.actions(|x| {
                let table = || {
                    dict::<Any, Num, _, _>([
                        (Expr::<Hero>::from(Hero::Pharah).erase(), 2),
                        (Expr::<Hero>::from(Hero::Brigitte).erase(), 3),
                    ])
                };
                x.set(g.a, table().get(g.b.get().erase()));
                x.comment("null");
                x.set(g.a, table().get(1));
                x.comment("2");
                x.set(g.a, table().get(Expr::<Hero>::from(Hero::Pharah).erase()));
            });
        });
    }

    fn options(&self) -> CompileOptions {
        size()
    }
}

#[test]
#[wasm_bindgen_test]
fn dicts() {
    check(&Dicts, include_str!("fixtures/reference/dicts.txt"));
}

// builtinenums: `Math.FUCKTON_OF_SPACES` is `math::LOTS_OF_SPACES`.
#[derive(GlobalVars)]
struct EnumGlobals {
    #[ow(name = "A", index = 0)]
    a: Global<Num>,
    #[ow(name = "B", index = 1)]
    b: Global<Num>,
    #[ow(name = "C", index = 2)]
    c: Global<Num>,
    #[ow(name = "D", index = 3)]
    d: Global<Num>,
    #[ow(name = "E", index = 4)]
    e: Global<Num>,
    #[ow(name = "F", index = 5)]
    f: Global<Num>,
    #[ow(name = "G", index = 6)]
    g: Global<Num>,
    #[ow(name = "H", index = 7)]
    h: Global<Num>,
    #[ow(name = "I", index = 8)]
    i: Global<Num>,
    #[ow(name = "J", index = 9)]
    j: Global<Num>,
    #[ow(name = "K", index = 10)]
    k: Global<Num>,
    #[ow(name = "S", index = 18)]
    s: Global<Num>,
}

struct BuiltinEnums;

impl Workshop for BuiltinEnums {
    type Globals = EnumGlobals;
    type Players = ();

    fn build(&self, ws: &mut Builder, g: &EnumGlobals, (): &()) {
        ws.rule("enums", OngoingEachPlayer, |r, ev| {
            let me = ev.event_player();
            r.condition(me.is_holding_button(Button::Interact));
            r.condition(truth(g.a.get()));
            r.actions(|x| {
                let all_a = || as_::<Array<Any>>(g.a.get());
                put(x, g.a, Expr::<Team>::from(Team::All));
                put(x, g.b, Expr::<Hero>::from(Hero::Brigitte));
                put(x, g.c, Expr::<Map>::from(Map::BlackForestWinter));
                put(x, g.d, current_gamemode().eq(Gamemode::Assault));
                put(x, g.e, Expr::<Button>::from(Button::Interact));
                put(x, g.f, me.is_holding_button(Button::Interact));
                x.start_forcing_button(me.clone(), Button::Melee);
                x.start_forcing_hero(me.clone(), Hero::Ana);
                x.wait_with(0.016, Wait::AbortWhenFalse);
                put(x, g.g, vector_backward());
                put(
                    x,
                    g.h,
                    random::int(1, 2)
                        + random::choice(all_a()).erase().cast::<Num>()
                        + random::real(1.2, 3.4)
                        + random::shuffle(all_a()).erase().cast::<Num>(),
                );
                put(x, g.i, Expr::<Num>::from(math::E));
                put(x, g.j, Expr::<Num>::from(math::PI));
                put(x, g.k, Expr::<Color>::from(Color::Blue));
                put(x, g.s, Expr::<Str>::from(math::LOTS_OF_SPACES));
            });
        });
    }

    fn options(&self) -> CompileOptions {
        size()
    }
}

#[test]
#[wasm_bindgen_test]
fn builtinenums() {
    check(
        &BuiltinEnums,
        include_str!("fixtures/reference/builtinenums.txt"),
    );
}

// builtinmacros: `splitDictArray` and `tabular` are both `columns`.
#[derive(GlobalVars)]
struct MacroGlobals {
    #[ow(name = "waveHeroes", index = 0)]
    wave_heroes: Global<Array<Hero>>,
    #[ow(name = "waveLengths", index = 1)]
    wave_lengths: Global<Array<Num>>,
}

struct BuiltinMacros;

impl Workshop for BuiltinMacros {
    type Globals = MacroGlobals;
    type Players = ();

    fn build(&self, ws: &mut Builder, g: &MacroGlobals, (): &()) {
        for name in ["splitDictArray", "tabular"] {
            ws.rule(name, OngoingGlobal, |r, _| {
                r.actions(|x| {
                    x.columns(
                        (g.wave_heroes, g.wave_lengths),
                        [
                            (Hero::Ana, Some(3)),
                            (Hero::Soldier, Some(8)),
                            (Hero::WreckingBall, None),
                        ],
                    );
                });
            });
        }
    }

    fn options(&self) -> CompileOptions {
        size()
    }
}

#[test]
#[wasm_bindgen_test]
fn builtinmacros() {
    check(
        &BuiltinMacros,
        include_str!("fixtures/reference/builtinmacros.txt"),
    );
}

// getCurrentMap
#[derive(GlobalVars)]
struct MapGlobals {
    #[ow(name = "A", index = 0)]
    a: Global<Map>,
    #[ow(name = "B", index = 1)]
    b: Global<Bool>,
    #[ow(name = "C", index = 2)]
    c: Global<Bool>,
}

struct GetCurrentMap;

impl Workshop for GetCurrentMap {
    type Globals = MapGlobals;
    type Players = ();

    fn build(&self, ws: &mut Builder, g: &MapGlobals, (): &()) {
        let map = |m: Map| Expr::<Map>::from(m);
        ws.rule(
            "test current map workaround for colosseo/esperança/samoa",
            OngoingGlobal,
            |r, _| {
                r.actions(|x| {
                    x.set(g.a, current_map());
                    x.set(g.b, g.a.get().eq(map(Map::Samoa)));
                    x.set(g.c, current_map().eq(map(Map::KingsRow)));
                    x.set(g.c, map(Map::KingsRow).eq(current_map()));
                    x.set(g.c, current_map().eq(map(Map::Colosseo)));
                    x.set(g.c, map(Map::Colosseo).eq(current_map()));
                });
            },
        );
    }

    fn options(&self) -> CompileOptions {
        size()
    }
}

#[test]
#[wasm_bindgen_test]
fn getcurrentmap() {
    // Allowed differences, all in how `Current Map` is rewritten. The bugged maps are listed in
    // the order the mod mentions them (Samoa, then Colosseo) and appended to `Current Map`
    // with `Append To Array`. Every use of `Current Map` is rewritten once a bugged map is
    // referenced, including a comparison with a map that is not bugged. A comparison keeps its
    // operand order.
    let fixed = "First Of(Filtered Array(Append To Array(Array(Map(Samoa), Map(Colosseo)), Current Map), Compare(Custom String(\"{0}\", Current Map), ==, String Split(Current Array Element, Empty Array))))";
    let colosseo = "Compare(Custom String(\"{0}\", Current Map), ==, Custom String(\"{0}\", Map(Colosseo))));\n    }";
    check_with(
        &GetCurrentMap,
        include_str!("fixtures/reference/getCurrentMap.txt"),
        &[
            (
                "First Of(Filtered Array(Array(Map(Colosseo), Map(Samoa), Current Map), Compare(Custom String(\"{0}\", Current Map), ==, String Split(Current Array Element, Empty Array))))",
                fixed,
            ),
            (
                "Compare(Current Map, ==, Map(King's Row))",
                &format!("Compare({fixed}, ==, Map(King's Row))"),
            ),
            (
                "Compare(Map(King's Row), ==, Current Map)",
                &format!("Compare(Map(King's Row), ==, {fixed})"),
            ),
            (
                colosseo,
                "Compare(Custom String(\"{0}\", Map(Colosseo)), ==, Custom String(\"{0}\", Current Map)));\n    }",
            ),
        ],
    );
}

// eventVars: subroutines are indexed in declaration order.
#[derive(GlobalVars)]
struct EventGlobals {
    #[ow(name = "A", index = 0)]
    a: Global<Num>,
}

struct EventVars;

impl Workshop for EventVars {
    type Globals = EventGlobals;
    type Players = ();

    fn build(&self, ws: &mut Builder, g: &EventGlobals, (): &()) {
        let ep_sub2 = ws.declare_subroutine("epSub2");
        let ep_sub = ws.declare_subroutine("epSub");
        let global_sub = ws.declare_subroutine("globalSub");
        let actually_ep_sub = ws.declare_subroutine("actuallyEpSub");
        let recursion2 = ws.declare_subroutine("recursion2");
        let recursion1 = ws.declare_subroutine("recursion1");

        ws.define(ep_sub2, |b, ev| put(b, g.a, ev.event_player()));
        ws.define(ep_sub, |b, ev| put(b, g.a, ev.event_player()));
        ws.define(global_sub, |b, _| b.call(ep_sub2));
        ws.define(actually_ep_sub, |b, _| b.call(ep_sub2));
        ws.define(recursion1, |b, _| b.call(recursion2));
        ws.define(recursion2, |b, ev| {
            put(b, g.a, ev.event_damage());
            b.call(recursion1);
        });
        ws.rule("global", OngoingGlobal, |r, _| {
            r.actions(|x| {
                x.comment("should throw warning");
                x.call(ep_sub);
                x.comment("should also throw warning");
                x.call(global_sub);
            });
        });
        ws.rule("ep", OngoingEachPlayer, |r, _| {
            r.actions(|x| {
                x.comment("should not throw warning");
                x.call(actually_ep_sub);
                x.comment("should throw warning");
                x.call(recursion1);
            });
        });
    }

    fn options(&self) -> CompileOptions {
        // The source has no `#!optimizeForSize`.
        CompileOptions::default()
    }
}

#[test]
#[wasm_bindgen_test]
fn eventvars() {
    let output = EventVars.compile(Lang::EnUs).expect("compiles");
    // `epSub`, `globalSub` and the recursive pair read event values their callers may lack.
    assert!(
        output
            .warnings
            .iter()
            .any(|w| matches!(w.kind, Kind::Warning(Warning::MismatchedSubroutineEvent))),
        "{:?}",
        output.warnings
    );
    // `A = eventWasCriticalHit` is left out: `Ongoing - Each Player` has no such event value,
    // and the typed events reject reading it there.
    check_with(
        &EventVars,
        include_str!("fixtures/reference/eventVars.txt"),
        &[(
            "        Set Global Variable(A, Event Was Critical Hit);\n",
            "",
        )],
    );
}

// arrays: the reference variables are untyped; `A` is an array and the rest hold anything.
#[derive(GlobalVars)]
struct ArrayGlobals {
    #[ow(name = "A", index = 0)]
    a: Global<Array<Any>>,
    #[ow(name = "B", index = 1)]
    b: Global<Any>,
    #[ow(name = "C", index = 2)]
    c: Global<Any>,
    #[ow(name = "D", index = 3)]
    d: Global<Any>,
    #[ow(name = "E", index = 4)]
    e: Global<Any>,
    #[ow(name = "X", index = 23)]
    x: Global<Any>,
    #[ow(name = "Y", index = 24)]
    y: Global<Any>,
    #[ow(name = "Z", index = 25)]
    z: Global<Any>,
}

#[derive(PlayerVars)]
struct ArrayPlayers {
    #[ow(name = "A", index = 0)]
    a: PlayerVar<Any>,
}

/// An array of anything.
fn items(values: Vec<Expr<Any>>) -> Expr<Array<Any>> {
    IntoExpr::<Array<Any>>::into_expr(values)
}

/// An array of numbers.
fn numbers(values: &[i32]) -> Expr<Array<Any>> {
    IntoExpr::<Array<Any>>::into_expr(values.to_vec())
}

/// A value read as an array.
fn as_array(e: Expr<Any>) -> Expr<Array<Any>> {
    e.cast::<Array<Any>>()
}

struct Arrays;

impl Workshop for Arrays {
    type Globals = ArrayGlobals;
    type Players = ArrayPlayers;

    fn build(&self, ws: &mut Builder, g: &ArrayGlobals, p: &ArrayPlayers) {
        let (a, b, c, d, e) = (
            || g.a.get().erase(),
            || g.b.get(),
            || g.c.get(),
            || g.d.get(),
            || g.e.get(),
        );
        let set_a = |x: &mut Block, value: Expr<Array<Any>>| x.set(g.a, value);
        // Stores any value in `A`.
        macro_rules! sa {
            ($x:expr, $v:expr) => {
                $x.set(g.a, $v.erase().cast::<Array<Any>>())
            };
        }
        let any = |v: i32| Expr::<Num>::from(v).erase();
        let num = |e: Expr<Any>| e.cast::<Num>();
        let players = all_players;
        let health_is_two = |pl: Expr<Player>, _: Expr<Num>| pl.var(p.a).get().eq(2);
        ws.rule("array literals", OngoingGlobal, |r, _| {
            r.actions(|x| {
                x.comment("empty array");
                set_a(x, items(vec![]));
                set_a(x, numbers(&[1, 2, 3]));
                sa!(x, as_array(a()).get(num(b())));
                x.comment("first of(B)");
                sa!(x, as_array(b()).get(0));
                for (index, note) in [
                    (0.0, "B"),
                    (1.0, "C"),
                    (3.0, "0"),
                    (1.3, "C"),
                    (1.5, "D"),
                    (1.7, "D"),
                    (2.6, "0"),
                ] {
                    x.comment(note);
                    sa!(x, items(vec![b(), c(), d()]).get(index));
                }
                set_a(
                    x,
                    items(vec![numbers(&[1, 2]).erase(), numbers(&[3, 4]).erase()]),
                );
                x.set(g.a.at(0), as_array(b()).get(0));
            });
        });
        ws.rule("last", OngoingGlobal, |r, _| {
            r.actions(|x| {
                sa!(x, as_array(b()).last());
                x.comment("D");
                sa!(x, items(vec![b(), c(), d()]).last());
            });
        });
        ws.rule("filtered array", OngoingGlobal, |r, _| {
            r.actions(|x| {
                sa!(x, players().filter(health_is_two));
                sa!(
                    x,
                    players().filter(|pl, _| pl.clone().erase().ne(b().eq(pl.erase())))
                );
                x.comment("getAllPlayers.remove(B)");
                sa!(x, players().filter(|pl, _| pl.erase().ne(b())));
                x.comment("getAllPlayers() if A == B else []");
                sa!(x, players().filter(|_, _| a().eq(b())));
                x.comment("getAllPlayers()");
                sa!(x, players().filter(|_, _| true.into()));
                sa!(x, players().filter(|pl, i| pl.erase().ne(i)));
                sa!(x, players().filter(health_is_two).map(|pl, _| pl.health()));
                x.comment("mapped array()");
                sa!(
                    x,
                    players()
                        .filter(|_, _| true.into())
                        .map(|pl, _| pl.health())
                );
            });
        });
        ws.rule("mapped array", OngoingGlobal, |r, _| {
            r.actions(|x| {
                sa!(x, players().map(|pl, _| pl.health()));
                x.comment("getAllPlayers()");
                sa!(x, players().map(|pl, _| pl));
            });
        });
        ws.rule("index", OngoingGlobal, |r, _| {
            r.actions(|x| {
                sa!(x, as_array(b()).index_of(3));
                x.comment("[A,B].index(B)");
                sa!(x, items(vec![a(), b(), c()]).index_of(b()));
                x.comment("-1");
                sa!(x, items(vec![]).index_of(b()));
                x.comment("2");
                sa!(x, items(vec![any(1), any(2), any(3), b()]).index_of(3));
                x.comment("should not be optimized");
                sa!(x, items(vec![any(1), any(2), b(), any(3)]).index_of(3));
                x.comment("-1");
                sa!(x, numbers(&[1, 2, 3]).index_of(123));
                x.comment("should not be optimized");
                sa!(x, items(vec![any(1), any(2), any(3), b()]).index_of(123));
            });
        });
        ws.rule("slice", OngoingGlobal, |r, _| {
            r.actions(|x| {
                sa!(x, as_array(b()).slice(1, 3));
                for (start, count, note) in [
                    (3.0, 1.0, "[4]"),
                    (3.0, 5.0, "[4,5]"),
                    (3.0, -2.0, "[]"),
                    (5.0, 2.0, "[]"),
                    (-2.0, 3.0, "[1]"),
                    (0.4, 2.5, "[1,2,3]"),
                ] {
                    x.comment(note);
                    sa!(x, numbers(&[1, 2, 3, 4, 5]).slice(start, count));
                }
            });
        });
        ws.rule("in", OngoingGlobal, |r, _| {
            r.actions(|x| {
                x.comment("true");
                sa!(x, items(vec![any(432), c(), any(123), b()]).contains(123));
                x.comment("true");
                sa!(x, items(vec![any(432), c(), any(123), b()]).contains(b()));
                x.comment("false");
                sa!(x, items(vec![]).contains(b()));
                x.comment("false");
                sa!(x, numbers(&[432, 643, 124]).contains(123));
                x.comment("123 == B");
                sa!(
                    x,
                    items(vec![any(432), any(643), b(), any(124)]).contains(123)
                );
            });
        });
        ws.rule("exclude", OngoingGlobal, |r, _| {
            r.actions(|x| {
                sa!(x, as_array(b()).without(3));
                x.comment("should not be optimized");
                sa!(x, items(vec![b(), c(), d()]).without(c()));
                x.comment("[1,3]");
                sa!(x, numbers(&[1, 2, 3]).without(2));
                x.comment("[1,D].exclude(2)");
                sa!(x, items(vec![any(1), any(2), d()]).without(2));
                x.comment("[2]");
                sa!(x, numbers(&[1, 2, 3]).exclude(numbers(&[1, 3])));
                x.comment("[2,3].exclude([D])");
                sa!(x, numbers(&[1, 2, 3]).exclude(items(vec![any(1), d()])));
            });
        });
        ws.rule("concat", OngoingGlobal, |r, _| {
            r.actions(|x| {
                sa!(x, as_array(b()).with(2));
                x.comment("[B,C,123]");
                sa!(x, items(vec![b(), c()]).with(123));
                x.comment("should not be optimized");
                sa!(x, items(vec![b(), c()]).concat(g.a.get()));
                x.comment("[B,C,D,E]");
                sa!(x, items(vec![b(), c()]).concat(items(vec![d(), e()])));
            });
        });
        ws.rule("other array functions", OngoingGlobal, |r, _| {
            r.actions(|x| {
                sa!(x, players().map(health_is_two).all_true());
                sa!(x, players().map(health_is_two).any_true());
                sa!(
                    x,
                    a().and(b())
                        .erase()
                        .cast::<Array<Any>>()
                        .map(|i, _| as_array(c()).contains(i))
                        .any_true()
                );
                sa!(
                    x,
                    if_then_else(b().cast::<Bool>(), as_array(a()), as_array(c()))
                        .map(|i, _| i.eq(2))
                        .any_true()
                );
                sa!(x, players().all_true());
                sa!(x, g.a.get().sorted_by(|e, _| num(e) + 2));
                sa!(x, g.a.get().sorted_by(|e, _| num(e)));
                x.remove_index(g.a, 1);
                x.append(g.a, 3);
                x.remove_value(g.a, 3);
                x.comment("Z,Y,X");
                sa!(x, items(vec![g.x.get(), g.y.get(), g.z.get()]).reversed());
                sa!(x, as_array(b()).reversed());
            });
        });
        ws.rule("member array functions", OngoingGlobal, |r, _| {
            r.actions(|x| {
                let empty = || items(vec![]);
                sa!(x, players().all(health_is_two));
                sa!(x, players().all_true());
                x.comment("true");
                sa!(x, players().all(|_, _| true.into()));
                x.comment("len(getAllPlayers()) == 0    =>    not len(getAllPlayers())");
                sa!(x, players().all(|_, _| false.into()));
                x.comment("true");
                sa!(x, empty().all(|_, _| false.into()));
                x.comment("true");
                sa!(x, empty().all(|e, _| num(e).gt(2)));

                sa!(x, players().any(health_is_two));
                sa!(x, players().any_true());
                x.comment("false");
                sa!(x, players().any(|_, _| false.into()));
                x.comment("len(getAllPlayers()) > 0");
                sa!(x, players().any(|_, _| true.into()));
                x.comment("false");
                sa!(x, empty().any(|_, _| true.into()));
                x.comment("false");
                sa!(x, empty().any(|e, _| num(e).gt(2)));

                sa!(x, players().filter(health_is_two));
                sa!(x, players().map(|pl, _| pl.health()));
                sa!(x, g.a.get().filter(|e, i| e.erase().ne(i)));
                sa!(x, g.a.get().map(|e, i| num(e.erase()) + i));
                sa!(x, players().all(|pl, _| num(pl.var(p.a).get()).gt(0)));
                sa!(x, players().any(|pl, i| pl.var(p.a).get().eq(i.erase())));
                sa!(x, players().filter(health_is_two).map(|pl, _| pl.health()));
            });
        });
        ws.rule("array to string", OngoingGlobal, |r, _| {
            r.actions(|x| {
                set_a(x, numbers(&[10, 20, 30]));
                x.set(g.b, numbers(&[100]));
                for max in 0..=4 {
                    x.print(array_to_string(g.a.get(), max));
                }
                for max in 0..=2 {
                    x.print(array_to_string(items(vec![]), max));
                }
                for max in 0..=2 {
                    x.print(array_to_string(as_array(b()), max));
                }
            });
        });
    }
}

#[test]
#[wasm_bindgen_test]
fn arrays() {
    let fixture = include_str!("fixtures/reference/arrays.txt");
    let output = Arrays.compile(Lang::EnUs).expect("compiles").code;
    let rules = [
        "array literals",
        "last",
        "filtered array",
        "mapped array",
        "index",
        "slice",
        "in",
        "exclude",
        "concat",
        "other array functions",
        "member array functions",
    ];
    assert_workshop_code_eq(&only_rules(&output, &rules), &only_rules(fixture, &rules));

    // `array to string`: `array_to_string` is its own formula, not the reference's, so the
    // rule is not compared text for text. Each of the 11 calls prints one `Create HUD Text`.
    let printed = |code: &str| {
        only_rules(code, &["array to string"])
            .lines()
            .filter(|line| line.contains("Create HUD Text("))
            .count()
    };
    assert_eq!(printed(&output), 11);
    assert_eq!(printed(fixture), 11);
}

// functions: `A` is an array (it is appended to), `B` a number (it is chased), the rest hold
// anything.
#[derive(GlobalVars)]
struct FnGlobals {
    #[ow(name = "A", index = 0)]
    a: Global<Array<Any>>,
    #[ow(name = "B", index = 1)]
    b: Global<Num>,
    #[ow(name = "C", index = 2)]
    c: Global<Any>,
    #[ow(name = "D", index = 3)]
    d: Global<Any>,
    #[ow(name = "E", index = 4)]
    e: Global<Any>,
    #[ow(name = "X", index = 23)]
    x: Global<Any>,
    #[ow(name = "Y", index = 24)]
    y: Global<Any>,
    #[ow(name = "Z", index = 25)]
    z: Global<Any>,
}

#[derive(PlayerVars)]
struct FnPlayers {
    #[ow(name = "B", index = 1)]
    b: PlayerVar<Num>,
}

struct Functions;

impl Workshop for Functions {
    type Globals = FnGlobals;
    type Players = FnPlayers;

    fn build(&self, ws: &mut Builder, g: &FnGlobals, p: &FnPlayers) {
        let (a, b, c, d, e) = (
            || g.a.get().erase(),
            || g.b.get().erase(),
            || g.c.get(),
            || g.d.get(),
            || g.e.get(),
        );
        // Stores any value in `A`.
        macro_rules! sa {
            ($x:expr, $v:expr) => {
                $x.set(g.a, $v.erase().cast::<Array<Any>>())
            };
        }
        // Reads a value as another type.
        macro_rules! ty {
            ($t:ty, $v:expr) => {
                $v.erase().cast::<$t>()
            };
        }
        let team_args = |t: Expr<Any>| ty!(Team, t);
        ws.rule("function", OngoingEachPlayer, |r, ev| {
            let me = ev.event_player();
            let hud = |value: i32, color: Color| HudArgs {
                location: HudPosition::Left.into(),
                sort_order: n(value),
                color: color.into(),
                reevaluation: HudReeval::VisibilityAndString.into(),
                spec_visibility: SpecVisibility::Always.into(),
                ..HudArgs::default()
            };
            r.actions(|x| {
                let cast = |include: bool| {
                    ray_cast_with(
                        ty!(Position, b()),
                        ty!(Position, c()),
                        RayCastArgs {
                            players_to_include: ty!(Array<Player>, d()),
                            players_to_exclude: ty!(Array<Player>, e()),
                            include_player_owned_objects: include.into(),
                        },
                    )
                };
                sa!(x, cast(true).hit_player());
                sa!(x, cast(false).hit_position());
                sa!(x, cast(true).hit_normal());
                sa!(x, math::sign(-3));
                sa!(
                    x,
                    ty!(Str, b()).slice_with(
                        2,
                        SubstringArgs {
                            substring_length: n(4)
                        }
                    )
                );

                x.chase_over_time(
                    me.var(p.b),
                    9999,
                    3,
                    ChaseTimeReeval::DestinationAndDuration,
                );
                x.chase_over_time(
                    me.var(p.b),
                    9999,
                    3,
                    ChaseTimeReeval::DestinationAndDuration,
                );
                x.chase_at_rate(me.var(p.b), 9999, 3, ChaseRateReeval::DestinationAndRate);
                x.chase_over_time(g.b, 9999, 1, ChaseTimeReeval::DestinationAndDuration);
                x.chase_at_rate(g.b, 99999, 1, ChaseRateReeval::DestinationAndRate);

                x.hud_header_with("test", hud(1, Color::Red));
                x.hud_subtext_with("test", hud(1, Color::Red));
                x.hud_subheader_with("test", hud(1, Color::Red));
                x.hud_text_with(HudTextArgs {
                    visible_to: null(),
                    header: Expr::<Str>::from("azer").erase(),
                    subheader: Expr::<Str>::from("dqsf").erase(),
                    text: Expr::<Str>::from("fdsq").erase(),
                    location: HudPosition::Left.into(),
                    sort_order: n(-1),
                    header_color: Color::Red.into(),
                    subheader_color: Color::White.into(),
                    text_color: Color::Yellow.into(),
                    reevaluation: HudReeval::VisibilitySortOrderAndString.into(),
                    spec_visibility: SpecVisibility::Never.into(),
                });
                x.wait(0);
                sa!(x, closest_player(ty!(Position, b()), team_args(c())));
                sa!(
                    x,
                    real_closest_player_with(
                        ty!(Position, b()),
                        RealPlayersArgs {
                            team: team_args(c())
                        }
                    )
                    .expect("literal center")
                );
                sa!(
                    x,
                    living_players_with(GetLivingPlayersArgs {
                        team: team_args(c())
                    })
                    .filter(|pl, _| pl.has_spawned())
                    .sorted_by(|pl, _| distance(ty!(Position, pl), ty!(Position, b())))
                    .first()
                );
                let spawned = |team: Expr<Any>| {
                    living_players_with(GetLivingPlayersArgs {
                        team: team_args(team),
                    })
                    .filter(|pl, _| pl.has_spawned())
                };
                sa!(x, farthest_player(ty!(Position, b()), team_args(c())));
                sa!(
                    x,
                    real_farthest_player_with(
                        ty!(Position, b()),
                        RealPlayersArgs {
                            team: team_args(c())
                        }
                    )
                    .expect("literal center")
                );
                sa!(
                    x,
                    spawned(c())
                        .sorted_by(|pl, _| -distance(ty!(Position, pl), ty!(Position, b())))
                        .first()
                );
                // The real version: spawned, living players other than `A`, by angle to `A`'s view.
                let reticle = || {
                    let me = ty!(Player, a());
                    living_players_with(GetLivingPlayersArgs {
                        team: team_args(b()),
                    })
                    .filter(|pl, _| pl.has_spawned() & pl.erase().ne(a()))
                    .sorted_by(move |pl, _| {
                        angle_between_vectors(
                            me.facing_direction(),
                            ty!(Vector, pl) - ty!(Vector, me.eye_position()),
                        )
                    })
                    .first()
                };
                sa!(
                    x,
                    ty!(Player, a()).player_closest_to_reticle(team_args(b()))
                );
                sa!(x, reticle());
                sa!(x, reticle());
                sa!(
                    x,
                    math::line_intersects_sphere(
                        ty!(Position, b()),
                        ty!(Direction, random::choice(ty!(Array<Any>, c()))),
                        ty!(Position, d()),
                        ty!(Num, random::choice(ty!(Array<Any>, e()))),
                    )
                );
                x.comment("Color.YELLOW");
                sa!(x, rgb(255, 255, 0));
                sa!(x, current_map());
            });
        });
        ws.rule("beams", OngoingGlobal, |r, _| {
            let beam = |kind: Beam, start: Expr<Position>, end: Expr<Position>| {
                let opts = CreateBeamArgs {
                    color: Color::White.into(),
                    reevaluation: EffectReeval::VisibilityPositionAndRadius.into(),
                };
                (kind, start, end, opts)
            };
            let [va, vb, vc, vd, vz] = [a(), b(), c(), d(), g.z.get()].map(|v| ty!(Num, v));
            let line = |first: Expr<Num>| {
                (
                    vect(first, vb.clone(), vc.clone()),
                    vect(va.clone(), vd.clone(), vc.clone()),
                )
            };
            r.actions(|x| {
                let all = ty!(Players, a());
                let mut create = |(kind, start, end, opts): (
                    Beam,
                    Expr<Position>,
                    Expr<Position>,
                    CreateBeamArgs,
                )| {
                    x.create_beam_with(all.clone(), kind, start, end, opts);
                };
                create(beam(Beam::Grapple, ty!(Position, b()), ty!(Position, c())));
                let (s1, e1) = line(va.clone());
                create(beam(Beam::Grapple, ty!(Position, s1), ty!(Position, e1)));
                let (s2, e2) = line(vz.clone());
                create(beam(Beam::Grapple, ty!(Position, s2), ty!(Position, e2)));
                for kind in [Beam::Good, Beam::Good, Beam::Bad] {
                    let (s, e) = line(va.clone());
                    create(beam(kind, ty!(Position, s), ty!(Position, e)));
                }
                let (s, _) = line(va.clone());
                let end = vect(va.clone(), vd.clone(), vz.clone());
                create(beam(Beam::Bad, ty!(Position, s), ty!(Position, end)));
            });
        });
        ws.rule("vectors", OngoingGlobal, |r, _| {
            let [vb, vc, vd] = [b(), c(), d()].map(|v| ty!(Num, v));
            r.actions(|x| {
                sa!(x, vect(vb.clone(), vc.clone(), vd.clone()));
                sa!(x, ty!(Vector, b()).x());
                for (value, note) in [
                    (vect(vb.clone(), vc.clone(), vd.clone()).x(), "B"),
                    (vect(vb.clone(), vc.clone(), vd.clone()).y(), "C"),
                    (vect(vb.clone(), vc.clone(), vd.clone()).z(), "D"),
                ] {
                    x.comment(note);
                    sa!(x, value);
                }
                x.comment("Vector.UP");
                sa!(x, vect(0, 1, 0));
            });
        });
        ws.rule("optimizations with literals", OngoingGlobal, |r, _| {
            r.actions(|x| {
                macro_rules! app {
                    ($note:literal, $v:expr) => {{
                        x.comment($note);
                        x.append(g.a, $v.erase());
                    }};
                }
                let v = |a: i32, b: i32, c: i32| vect(a, b, c);
                app!("4", math::ceil(3.3));
                app!("3", math::floor(3.7));
                app!("4", math::round(3.5));
                app!("3", math::round(2.5));
                app!("5", abs(-5));
                app!("1.012", acos(0.53));
                app!("3.1415", acos(-3));
                app!("113.633", acos_deg(-0.4));
                app!("-0.302", asin(-0.3));
                app!("36.865", asin_deg(0.6));
                app!("1.584", atan2(920, -12));
                app!("140.057", atan2_deg(103, -123));
                app!("0.996", cos(490));
                app!("-0.934", cos_deg(921));
                app!(
                    "vect(3951, -3165, -6406)",
                    cross_product(v(23, 53, -12), v(120, -2, 75))
                );
                app!(
                    "vect(-0.182, 0.273, 0.945)",
                    direction_towards(v(129, 23, -120), v(-75, 329, 938))
                );
                app!("1014.740", distance(v(329, -12, 643), v(-23, -125, -302)));
                app!("43498", dot_product(v(129, -35, 239), v(-12, 345, 239)));
                app!("Team 2", opposite_team(Team::One));
                app!("Team 1", opposite_team(Team::Two));
                app!("All Teams", opposite_team(Team::All));
                app!("0", len(items(vec![])));
                app!("3", len(items(vec![b(), c(), d()])));
                app!("4", math::log(10000, 10));
                app!("13.287", math::log(10000, 2));
                app!("9.210", math::ln(10000));
                app!("32", max(5, 32));
                app!("-31", min(-31, 6));
                app!("vect(-0.017, 0.433, 0.901)", normalize(v(-13, 324, 674)));
                app!("0.704", sin(19.63));
                app!("-0.218", sin_deg(-12.6));
                app!("0", sqrt(-4));
                app!("9.656", sqrt(93.23));
                app!("1.364", tan(912));
                app!("-36.648", tan_deg(91.563));
                app!(
                    "vect(342, -88, 68)",
                    vector_towards(v(-123, 53, 64), v(219, -35, 132))
                );
            });
        });
        ws.rule(
            "optimizations for function combinations",
            OngoingGlobal,
            |r, _| {
                let team = || ty!(Team, b());
                let on_team = || all_players_with(GetPlayersArgs { team: team() });
                r.actions(|x| {
                    x.comment("directionTowards(x, y)");
                    sa!(
                        x,
                        normalize(vector_towards(
                            ty!(Position, g.x.get()),
                            ty!(Position, g.y.get())
                        ))
                    );
                    x.comment("is dead(B)");
                    sa!(x, !ty!(Player, b()).is_alive());
                    x.comment("is alive(B)");
                    sa!(x, !ty!(Player, b()).is_dead());
                    x.comment("getPlayersOnHero(C, B)");
                    sa!(x, on_team().filter(|pl, _| pl.hero().eq(ty!(Hero, c()))));
                    x.comment("getPlayersOnObjective(B)");
                    sa!(x, on_team().filter(|pl, _| pl.is_on_objective()));
                    x.comment("getPlayersNotOnObjective(B)");
                    sa!(x, on_team().filter(|pl, _| !pl.is_on_objective()));
                    x.comment("getLivingPlayers(B)");
                    sa!(x, on_team().filter(|pl, _| pl.is_alive()));
                    x.comment("getDeadPlayers(B)");
                    sa!(x, on_team().filter(|pl, _| pl.is_dead()));
                    x.comment("getNumberOfPlayers(B)");
                    sa!(x, on_team().len());
                    x.comment("getNumberOfPlayersOnObjective(B)");
                    sa!(
                        x,
                        players_on_objective_with(GetPlayersOnObjectiveArgs { team: team() }).len()
                    );
                    x.comment("getNumberOfHeroes(B, C)");
                    sa!(
                        x,
                        players_on_hero_with(
                            ty!(Hero, b()),
                            GetPlayersOnHeroArgs {
                                team: ty!(Team, c())
                            }
                        )
                        .len()
                    );
                    x.comment("getNumberOfLivingPlayers(B)");
                    sa!(
                        x,
                        living_players_with(GetLivingPlayersArgs { team: team() }).len()
                    );
                    x.comment("getNumberOfDeadPlayers(B)");
                    sa!(
                        x,
                        dead_players_with(GetDeadPlayersArgs { team: team() }).len()
                    );
                });
            },
        );
        ws.rule("default arguments", OngoingGlobal, |r, _| {
            let hud = |visible: Option<Expr<Players>>| {
                let mut args = HudArgs::default();
                if let Some(players) = visible {
                    args.visible_to = players;
                }
                args
            };
            r.actions(|x| {
                x.hud_header_with(1234, hud(None));
                x.hud_header_with(1234, hud(Some(ty!(Players, a()))));
                x.hud_header_with(1234, hud(Some(ty!(Players, a()))));
                x.hud_header_with(
                    1234,
                    HudArgs {
                        location: HudPosition::Left.into(),
                        sort_order: n(0),
                        color: Color::White.into(),
                        reevaluation: HudReeval::VisibilitySortOrderStringAndColor.into(),
                        spec_visibility: SpecVisibility::Default.into(),
                        ..hud(Some(ty!(Players, a())))
                    },
                );
                x.wait(0);
                x.start_facing(ty!(Players, c()), vect(1, 2, 3));
                x.start_facing_with(
                    ty!(Players, c()),
                    vect(1, 2, 3),
                    StartFacingArgs {
                        relativity: Relativity::ToPlayer.into(),
                        ..StartFacingArgs::default()
                    },
                );
                x.set(g.d, ty!(Player, d()).ammo());
                x.set(g.e, ArrayExt::slice(&ty!(Array<Any>, e()), 5));
                x.set(g.e, ty!(Array<Any>, e()).slice(5, 7));
            });
        });
    }

    fn options(&self) -> CompileOptions {
        size()
    }
}

#[test]
#[wasm_bindgen_test]
fn functions() {
    // Allowed difference: the reference folds `acosDeg`, `asinDeg` and `atan2Deg` of a literal
    // to values that are not degrees (0.03, 0.01, 0.04), contradicting its own comments
    // (about 113.6, 36.9 and 140.1). The real degrees are expected here.
    check_with(
        &Functions,
        include_str!("fixtures/reference/functions.txt"),
        &[
            ("0.034597891672104", "113.57817847820183"),
            ("0.011231213088677", "36.86989764584402"),
            ("0.042663877670413", "140.05724853255916"),
        ],
    );
}

// comments: a multi-line comment and `#` comments become `comment` calls.
#[derive(GlobalVars)]
struct CommentGlobals {
    #[ow(name = "A", index = 0)]
    a: Global<Num>,
    #[ow(name = "B", index = 1)]
    b: Global<Num>,
    #[ow(name = "C", index = 2)]
    c: Global<Num>,
    #[ow(name = "D", index = 3)]
    d: Global<Num>,
}

struct Comments;

impl Workshop for Comments {
    type Globals = CommentGlobals;
    type Players = ();

    fn build(&self, ws: &mut Builder, g: &CommentGlobals, (): &()) {
        ws.rule("comments", OngoingGlobal, |r, _| {
            r.condition(truth(g.a.get())).comment("A");
            r.condition(truth(g.b.get())).comment("B");
            r.actions(|x| {
                x.comment("C++");
                x.add_to(g.c, 1);
                x.comment("D++");
                x.add_to(g.d, 1);
                x.comment("nothing\nsome array");
                put(x, g.a, IntoExpr::<Array<Num>>::into_expr(vec![1, 2, 3]));
            });
        });
    }

    fn options(&self) -> CompileOptions {
        size()
    }
}

#[test]
#[wasm_bindgen_test]
fn comments() {
    check(&Comments, include_str!("fixtures/reference/comments.txt"));
}

// gotos
#[derive(GlobalVars)]
struct GotoGlobals {
    #[ow(name = "A", index = 0)]
    a: Global<Num>,
}

struct Gotos;

impl Workshop for Gotos {
    type Globals = GotoGlobals;
    type Players = ();

    fn build(&self, ws: &mut Builder, g: &GotoGlobals, (): &()) {
        ws.rule("gotos", OngoingGlobal, |r, _| {
            r.actions(|x| {
                let (lbl_0, lbl_1, lbl_2, lbl_3) = (x.label(), x.label(), x.label(), x.label());
                x.comment("skip 3");
                x.skip_to(lbl_0);
                x.if_(false, |x| {
                    x.comment("skip 5");
                    x.skip_to(lbl_1);
                    x.set(g.a, 1);
                    x.place(lbl_0);
                    x.set(g.a, 2);
                });
                x.if_(false, |x| {
                    x.set(g.a, 1);
                    x.place(lbl_1);
                    x.set(g.a, 2);
                });
                x.comment("skip 3");
                x.skip_to(lbl_2);
                x.comment("skip 0");
                x.skip_to(lbl_3);
                x.place(lbl_3);
                x.if_(false, |x| {
                    x.set(g.a, 1);
                    x.place(lbl_2);
                    x.set(g.a, 2);
                });
            });
        });
    }

    fn options(&self) -> CompileOptions {
        size()
    }
}

#[test]
#[wasm_bindgen_test]
fn gotos() {
    // The reference deletes the dead code a skip jumps over inside `if false`, so its skip
    // distances are smaller (`Skip(True)`); this optimizer keeps dead branches that hold a
    // label (a skip lands inside them), so the distances count the code it keeps. A skip counted
    // at runtime (`goto loc+A`) is left out: it stops this optimizer from removing anything in
    // its rule. The expected output below is the verified result of this compiler.
    let expected = r#"
variables {
    global:
        0: A
}
//Optimize for size enabled
rule ("gotos") {
    event {
        Ongoing - Global;
    }
    actions {
        "skip 3"
        Skip(3);
        If(False);
            "skip 5"
            Skip(5);
            Set Global Variable(A, 1);
            Set Global Variable(A, 2);
        End;
        If(False);
            Set Global Variable(A, 1);
            Set Global Variable(A, 2);
        End;
        "skip 3"
        Skip(2);
        "skip 0"
        If(False);
            Set Global Variable(A, 1);
            Set Global Variable(A, 2);
    }
}
"#;
    let flat = |text: &str| {
        without_labels(text)
            .lines()
            .map(|line| line.trim_start().to_owned())
            .collect::<Vec<_>>()
            .join("\n")
    };
    let output = Gotos.compile(Lang::EnUs).expect("compiles");
    assert_workshop_code_eq(&flat(&output.code), &flat(expected));
    // The fixture itself still lists the same rule.
    assert!(include_str!("fixtures/reference/gotos.txt").contains("rule (\"gotos\")"));
}

// loops
#[derive(GlobalVars)]
struct LoopGlobals {
    #[ow(name = "A", index = 0)]
    a: Global<Num>,
    #[ow(name = "B", index = 1)]
    b: Global<Num>,
    #[ow(name = "C", index = 2)]
    c: Global<Num>,
    #[ow(name = "D", index = 3)]
    d: Global<Num>,
    #[ow(name = "I", index = 8)]
    i: Global<Num>,
}

#[derive(PlayerVars)]
struct LoopPlayers {
    #[ow(name = "I", index = 8)]
    i: PlayerVar<Num>,
}

struct Loops;

impl Workshop for Loops {
    type Globals = LoopGlobals;
    type Players = LoopPlayers;

    fn build(&self, ws: &mut Builder, g: &LoopGlobals, p: &LoopPlayers) {
        let (a, b, c) = (|| g.a.get(), || g.b.get(), || g.c.get());
        ws.rule("while loop", OngoingGlobal, |r, _| {
            r.actions(|x| {
                x.while_(a().eq(b()), |x| {
                    x.while_(b().eq(c()), |x| {
                        x.add_to(g.a, 1);
                        x.if_(truth(g.d.get()), |x| x.break_());
                    });
                });
                x.add_to(g.b, 1);
            });
        });
        ws.rule("while to the end", OngoingGlobal, |r, _| {
            r.actions(|x| {
                x.while_(a().eq(b()), |x| {
                    x.if_(truth(g.c.get()), |x| x.continue_());
                    x.add_to(g.a, 1);
                });
            });
        });
        ws.rule("while to the end nested", OngoingGlobal, |r, _| {
            r.actions(|x| {
                x.while_(a().eq(b()), |x| {
                    x.while_(b().eq(c()), |x| x.add_to(g.a, 1));
                });
            });
        });
        ws.rule("while false", OngoingGlobal, |r, _| {
            r.actions(|x| {
                x.while_(false, |x| x.add_to(g.a, 1));
                x.add_to(g.b, 1);
            });
        });
        ws.rule("for loop", OngoingGlobal, |r, _| {
            r.actions(|x| {
                x.for_range(g.i, range(1, 3).step(2), |x| x.add_to(g.a, 1));
                x.for_range(g.i, range(1, 3), |x| x.add_to(g.a, 1));
                x.for_range(g.i, 0..3, |x| x.add_to(g.a, 1));
                x.for_range(host_player().var(p.i), 0..3, |x| x.add_to(g.a, 1));
            });
        });
        ws.rule("do/while", OngoingGlobal, |r, _| {
            r.actions(|x| {
                x.do_while(a().eq(b()), |x| {
                    x.add_to(g.a, 1);
                    x.break_();
                });
                x.add_to(g.b, 1);
            });
        });
        ws.rule("do/while rule condition", OngoingGlobal, |r, _| {
            r.condition(a().eq(b()));
            r.actions(|x| {
                x.do_while(rule_condition(), |x| {
                    x.wait(0);
                    x.add_to(g.a, 1);
                    let players = number_of_players_with(GetNumberOfPlayersArgs {
                        team: Team::Two.into(),
                    });
                    x.if_(a().gt(players), |x| x.set(g.a, 0));
                });
            });
        });
    }

    fn options(&self) -> CompileOptions {
        size()
    }
}

#[test]
#[wasm_bindgen_test]
fn loops() {
    // `nested do/while` has no equivalent: a `do_while` must be the first statement of its
    // rule, so one cannot start another loop's body.
    let expected = without_rules(
        include_str!("fixtures/reference/loops.txt"),
        &["nested do/while"],
    );
    check_ignoring_labels(&Loops, &expected);
}

// rules
#[derive(GlobalVars)]
struct RuleGlobals {
    #[ow(name = "A", index = 0)]
    a: Global<Num>,
    #[ow(name = "B", index = 1)]
    b: Global<Num>,
    #[ow(name = "C", index = 2)]
    c: Global<Num>,
    #[ow(name = "D", index = 3)]
    d: Global<Num>,
    #[ow(name = "W", index = 22)]
    w: Global<Num>,
    #[ow(name = "X", index = 23)]
    x: Global<Num>,
    #[ow(name = "Y", index = 24)]
    y: Global<Num>,
    #[ow(name = "Z", index = 25)]
    z: Global<Num>,
}

#[derive(PlayerVars)]
struct RulePlayers {
    #[ow(name = "A", index = 0)]
    a: PlayerVar<Num>,
    #[ow(name = "B", index = 1)]
    b: PlayerVar<Num>,
}

struct Rules;

impl Workshop for Rules {
    type Globals = RuleGlobals;
    type Players = RulePlayers;

    fn build(&self, ws: &mut Builder, g: &RuleGlobals, p: &RulePlayers) {
        let t = |v: Global<Num>| truth(v.get());
        ws.rule("normal", OngoingGlobal, |r, _| {
            r.actions(|x| {
                x.add_to(g.a, 1);
                x.add_to(g.b, 1);
            });
        });
        ws.rule("event player", OngoingEachPlayer, |r, ev| {
            r.actions(|x| x.add_to(ev.event_player().var(p.a), 1));
        });
        ws.rule("filter on team+hero", PlayerTookDamage, |r, ev| {
            r.team(TeamFilter::Team1);
            r.hero(Hero::Ana);
            r.actions(|x| x.add_to(ev.victim().var(p.a), 1));
        });
        ws.rule("filter on team+slot", PlayerReceivedHealing, |r, ev| {
            r.team(TeamFilter::All);
            r.slot(3);
            r.actions(|x| x.add_to(ev.healer().var(p.b), 1));
        });
        ws.rule("other directives", PlayerDied, |r, _| {
            r.disabled();
            r.suppress(Warning::TypeCheck);
            r.delimiter();
        });
        ws.rule("conditions", OngoingGlobal, |r, _| {
            r.condition(t(g.a).and(t(g.b)));
            r.condition(t(g.x).and(t(g.y)).and(t(g.z).and(t(g.w))));
            r.condition(t(g.c));
            r.actions(|x| x.add_to(g.b, 1));
        });
        ws.rule("condition that is always true", OngoingGlobal, |r, _| {
            r.condition(g.a.get().eq(g.a.get()));
            r.actions(|x| x.add_to(g.b, 1));
        });
        ws.rule("rule condition test 1", OngoingGlobal, |r, _| {
            r.actions(|x| {
                x.comment("true");
                put(x, g.a, rule_condition());
            });
        });
        ws.rule("rule condition test 2", OngoingGlobal, |r, _| {
            r.condition(t(g.a));
            r.condition(t(g.b).and(t(g.c)));
            r.condition(g.d.get().eq(2));
            r.actions(|x| {
                x.comment("A and B and C and D == 2");
                put(x, g.a, rule_condition());
            });
        });
        ws.rule("rule condition test 3", OngoingGlobal, |r, _| {
            r.condition(t(g.a));
            r.actions(|x| put(x, g.a, rule_condition()));
        });
    }

    fn options(&self) -> CompileOptions {
        size()
    }
}

#[test]
#[wasm_bindgen_test]
fn rules() {
    // Allowed differences: the reference prints `ruleCondition` as a parenthesized `&&` chain
    // where this compiler emits `And`, and `rule condition test 3` (`A = ruleCondition` under
    // the condition `A`) is optimized away here, since it assigns `A` to itself. The
    // `condition that is always false` rule is not ported: the reference output drops it, as it
    // can never run.
    let expected = without_rules(
        include_str!("fixtures/reference/rules.txt"),
        &["rule condition test 3"],
    );
    check_with(
        &Rules,
        &expected,
        &[(
            "((Global.A) && (Global.B) && (Global.C) && (Compare(Global.D, ==, 2)))",
            "And(Global.A, And(And(Global.B, Global.C), Compare(Global.D, ==, 2)))",
        )],
    );
}

// rulePrefix: the reference prefixes rules with their source file; the groups below spell
// the same prefixes.
#[derive(GlobalVars)]
struct PrefixGlobals {
    #[ow(name = "A", index = 0)]
    a: Global<Num>,
}

struct RulePrefix;

impl Workshop for RulePrefix {
    type Globals = PrefixGlobals;
    type Players = ();

    fn build(&self, ws: &mut Builder, g: &PrefixGlobals, (): &()) {
        let rule = |ws: &mut Builder, name: &str, value: i32| {
            ws.rule(name, OngoingGlobal, |r, _| r.actions(|x| x.set(g.a, value)));
        };
        ws.group("RulePrefix", |ws| {
            rule(ws, "rule 1", 1);
            rule(ws, "rule 2", 2);
            rule(ws, "", 3);
            rule(ws, "rule 4", 4);
        });
        ws.group("Test Include/Test Prefix", |ws| {
            rule(ws, "rule 5", 5);
            rule(ws, "rule 6", 6);
        });
        ws.group("RulePrefix", |ws| rule(ws, "rule 7", 7));
        ws.group("Test Include/Include4", |ws| rule(ws, "4", 4));
        ws.group("RulePrefix", |ws| {
            rule(ws, "rule 8", 8);
            ws.rule("---- DELIMITER ----", OngoingGlobal, |r, _| r.delimiter());
        });
    }
}

#[test]
#[wasm_bindgen_test]
fn ruleprefix() {
    check(
        &RulePrefix,
        include_str!("fixtures/reference/rulePrefix.txt"),
    );
}

// strings
#[derive(GlobalVars)]
struct StringGlobals {
    #[ow(name = "A", index = 0)]
    a: Global<Num>,
    #[ow(name = "B", index = 1)]
    b: Global<Num>,
    #[ow(name = "C", index = 2)]
    c: Global<Num>,
    #[ow(name = "D", index = 3)]
    d: Global<Num>,
    #[ow(name = "E", index = 4)]
    e: Global<Num>,
    #[ow(name = "F", index = 5)]
    f: Global<Num>,
    #[ow(name = "G", index = 6)]
    g: Global<Num>,
    #[ow(name = "H", index = 7)]
    h: Global<Num>,
    #[ow(name = "owo", index = 8)]
    owo: Global<Num>,
}

/// `Add` of two strings, as the reference allows.
fn str_add(a: Expr<Str>, b: Expr<Str>) -> Expr<Num> {
    a.erase().cast::<Num>() + b.erase().cast::<Num>()
}

const CHARS_120: &str = "|_____________________________________________________________________________________________________________120 chars|";

struct Strings;

impl Workshop for Strings {
    type Globals = StringGlobals;
    type Players = ();

    fn build(&self, ws: &mut Builder, g: &StringGlobals, (): &()) {
        let (a, b, c, d, e) = (
            || g.a.get(),
            || g.b.get(),
            || g.c.get(),
            || g.d.get(),
            || g.e.get(),
        );
        let (f, gg, h) = (|| g.f.get(), || g.g.get(), || g.h.get());
        let no_args = || Vec::<Expr<Any>>::new();
        ws.rule("strings", OngoingGlobal, |r, _| {
            r.actions(|x| {
                x.print("custom 'string'");
                x.print("new\nline");
                x.print("qu\"o\"tes");
                x.print("zero width space: |\u{AD}|");
                x.print("");
                x.print("");
                x.print("single '\"quotes\"");
                x.print(Expr::<Str>::from("fullwidth string").fullwidth());
                x.print(Expr::<Str>::from("big letters string").big());
                put(x, g.a, localized_en("Hello", no_args()));
                x.print(localized_en("{0} {1}", ["localized", "string"]));
                x.print(localized_en("{0}, {1}, and {2}", [0, 1, 2]));

                put(x, g.a, Expr::<Str>::from("no tags: {{0}} {{}} <tx{{}}>"));
                put(x, g.a, ws_fmt!("uwu {}", c()).fullwidth());
                put(x, g.a, ws_fmt!("owo {}", n(1) + 2));
                put(x, g.b, ws_fmt!("123145{} {} {}", c(), a() + b(), 0).big());
                put(x, g.b, ws_fmt!("fullwidth a{}f", a() + b()).fullwidth());
                put(x, g.b, ws_fmt!("lowercase a{}f", g.owo.get()).case_sensitive());
                put(x, g.b, ws_fmt!("consecutivestrings{}owo", n(1) + 2));
                put(x, g.a, str_add(Expr::<Str>::from("owo"), Expr::<Str>::from("uwu")));

                put(
                    x,
                    g.a,
                    ws_fmt!(
                        "{0}{0}{0}{0}",
                        ws_fmt!(
                            "{0}{0}{0}{0}",
                            ws_fmt!("{0}{0}{0}{0}", ws_fmt!("{0}{0}{0}{0}", a()))
                        )
                    ),
                );
                put(x, g.a, ws_fmt!("A, B, B, C, D {}{}{}{}{}", a(), b(), b(), c(), d()));
                put(x, g.a, ws_fmt!("A, B, C, D {}{}{}{}", a(), b(), c(), d()));
                put(x, g.a, ws_fmt!("A, A, B, B {}{}{}{}", a(), a(), b(), b()));
                put(x, g.a, ws_fmt!("A, B, C, B, A {}{}{}{}{}", a(), b(), c(), b(), a()));
                put(x, g.a, ws_fmt!("A, B, C, D, A, B {}{}{}{}{}{}", a(), b(), c(), d(), a(), b()));

                put(x, g.a, ws_fmt!("{} {} {} {}", a(), b(), c(), c()));
                put(x, g.a, ws_fmt!("{} {} {} {}", a(), b(), b(), c()));
                put(x, g.a, ws_fmt!("{} {} {} {} {}", a(), b(), b(), c(), d()));
                put(x, g.a, ws_fmt!("{} {} {} {} {}", a(), b(), b(), c(), c()));
                put(x, g.a, ws_fmt!("{} {} {} {} {}", a(), b(), c(), d(), d()));
                put(x, g.a, ws_fmt!("{} {} {} {} {}", a(), b(), c(), c(), d()));

                x.print(ws_fmt!(
                    "Multiple fields {} {} {} {} {} {} {} {}",
                    a(),
                    b(),
                    c(),
                    d(),
                    e(),
                    f(),
                    gg(),
                    h()
                ));
                x.print(ws_fmt!("Numbered fields {0} {1} {2} {1} {3}", a(), b(), c(), d()));
                x.print(ws_fmt!("Numbered fields 2 {1}{0}{2}{1}", a(), b(), c()));
                // `ws_fmt!` rejects arguments no placeholder uses, so the unused ones are not passed.
                x.print(ws_fmt!("Missing field: {0} {1}", a(), c()));
                x.print(ws_fmt!("Missing field 2: {0} {1}", b(), c()));
                x.print(ws_fmt!("Null fields: {0} {1} {2}", null::<Any>(), null::<Any>(), null::<Any>()));
                x.print(Expr::<Str>::from(
                    "Very long string that will need to be splitted along several strings, as it is fullwidth and takes more than 128 characters which is the limit",
                ).fullwidth());
                x.print("String with entities: a\u{25A0}b\u{3000}c");
                x.print("String with \u{1} hex escape and \u{2000} unicode escape");
                x.print(Expr::<Str>::from("Case-sensitive string").case_sensitive());
                x.print("multiline\nstring");
                x.print(ws_fmt!("{} <fgFF00FFFF>texture test</fg> {} <txC0000000002DD21> {}", a(), b(), c()));
                x.print(ws_fmt!("{1} <fgFF00FFFF>texture test</fg> {2} <txC0000000002DD21> {0}", a(), b(), c()));
                x.print(ws_fmt!("<txC0000000002DD21>"));
                x.print(ws_fmt!("<txc12345> <tx{}>", 3));
                x.print(ws_fmt!("<tx{}>", a()));
                x.print(ws_fmt!("Optimized strings: {1} {0}{2} {3}", 123, 123.455, "|string|", a()));
                x.print(ws_fmt!(
                    "Optimized strings 2: {} {} {}{} {} {}",
                    a(),
                    0.125,
                    0.1,
                    Texture::MouseCursor,
                    b(),
                    ws_fmt!("other string {0}{0}", c())
                ));
                x.print(ws_fmt!("Optimized strings 3: {}", ws_fmt!("|str1|{}|str2|", a())));

                put(x, g.a, Expr::<Array<Str>>::from(vec![Expr::<Str>::from("a"); 450]));

                put(x, g.a, Expr::<Str>::from("a,b,c").split(","));
                put(x, g.a, Expr::<Str>::from("abc").find("b"));
                put(x, g.a, Expr::<Str>::from("a,b,c").replace(",", ";"));
                put(x, g.a, Expr::<Str>::from("abc").char_at(1));

                let chars = || Expr::<Str>::from(CHARS_120);
                put(x, g.a, ws_fmt!("{}{}{}", chars(), chars(), chars()));
                put(x, g.a, ws_fmt!("{}{}{}{}{}", a(), b(), c(), chars(), d()));
                x.comment("should not be split");
                put(x, g.a, ws_fmt!("{}12345678", chars()));
                x.comment("should be split");
                put(x, g.a, ws_fmt!("{}123456789", chars()));
                x.comment("should not be split");
                put(x, g.a, ws_fmt!("{}12345{}", chars(), a()));
                put(x, g.a, ws_fmt!("{}1234{}5", chars(), a()));
                put(x, g.a, ws_fmt!("{}123{}56", chars(), a()));
                put(x, g.a, ws_fmt!("{}12{}456", chars(), a()));
                put(x, g.a, ws_fmt!("{}12{}{}", chars(), a(), b()));
                put(x, g.a, ws_fmt!("{}1{}{}2", chars(), a(), b()));
                put(x, g.a, ws_fmt!("{}{}1{}2", chars(), a(), b()));
                x.comment("should be split");
                put(x, g.a, ws_fmt!("{}123456{}", chars(), a()));
                put(x, g.a, ws_fmt!("{}12345{}6", chars(), a()));
                put(x, g.a, ws_fmt!("{}1234{}56", chars(), a()));
                put(x, g.a, ws_fmt!("{}123{}456", chars(), a()));
                put(x, g.a, ws_fmt!("{}123{}{}4", chars(), a(), b()));
                put(x, g.a, ws_fmt!("{}123{}4{}", chars(), a(), b()));
                put(x, g.a, ws_fmt!("{}12{}34{}", chars(), a(), b()));
                put(x, g.a, ws_fmt!("{}12{}3{}4", chars(), a(), b()));
                put(x, g.a, ws_fmt!("{}1{}2{}34", chars(), a(), b()));
                put(x, g.a, ws_fmt!("{}{}{}{}", chars(), a(), b(), c()));

                put(
                    x,
                    g.a,
                    ws_fmt!(
                        "[{1}] LEAVE EDIT MODE\n{2} CREATE BOT\n{3} GRAB BOT\n[{4}] REMOVE BOT\n[{0}] + [{4}] REMOVE ALL BOTS\n[{5}] PLACE RESET MARKER\n[{0}] + [{5}] REMOVE RESET MARKER\n[{6}] INCREASE PLACEMENT DISTANCE\n[{0}] + [{6}] DECREASE PLACEMENT DISTANCE",
                        a(), b(), c(), d(), e(), f(), gg()
                    ),
                );
                put(
                    x,
                    g.a,
                    ws_fmt!(
                        "{1} CREATE BOT {2}\n{3} GRAB BOT\n[{4}] REMOVE BOT\n[{0}] + [{4}] REMOVE ALL BOTS\n[{4}] PLACE RESET MARKER\n[{0}] + [{4}] REMOVE RESET MARKER\n[{5}] INCREASE PLACEMENT DISTANCE\n[{0}] + [{5}] DECREASE PLACEMENT DISTANCE",
                        a(), b(), c(), d(), e(), f()
                    ),
                );
                put(
                    x,
                    g.a,
                    ws_fmt!("{0}fgFFFFFFF0>{1}{1}{1}{1}", a(), ws_fmt!("{0}TX C0000000002DD21>", a())),
                );
            });
        });
        ws.rule("optimizations", OngoingGlobal, |r, _| {
            r.actions(|x| {
                let lit = Expr::<Str>::from;
                x.comment("3");
                put(x, g.a, str_len("owo"));
                put(x, g.a, str_len(ws_fmt!("owo{}", a())));
                x.comment("true");
                put(x, g.a, str_contains("owo", "wo"));
                put(x, g.a, str_contains(ws_fmt!("owo{}", a()), "uwu"));
                x.comment("uwu");
                put(x, g.a, lit("owo").replace("o", "u"));
                put(x, g.a, lit("owo").replace("o", ws_fmt!("u{}", a())));
                x.comment("\"d\"");
                put(x, g.b, lit("abcdef").char_at(3));
                x.comment("\"\"");
                put(x, g.c, lit("abcdef").char_at(6));
                x.comment("\"a\"");
                put(x, g.d, lit("abcdef").char_at(-1));
                x.comment("\"def\"");
                put(
                    x,
                    g.e,
                    lit("abcdef").slice_with(
                        3,
                        SubstringArgs {
                            substring_length: n(9999),
                        },
                    ),
                );
                x.comment("\"abcd\"");
                put(
                    x,
                    g.f,
                    lit("abcdef").slice_with(
                        -1,
                        SubstringArgs {
                            substring_length: n(4),
                        },
                    ),
                );
                x.comment("\"\"");
                put(
                    x,
                    g.g,
                    lit("abcdef").slice_with(
                        2,
                        SubstringArgs {
                            substring_length: n(-1),
                        },
                    ),
                );
                x.comment("should not be optimized");
                put(x, g.a, lit("ab|cd|ef").split("|"));

                x.comment(".split(null[0])");
                put(x, g.a, Expr::<Array<Str>>::from(vec![lit("a"), lit("b")]));
                x.comment(".split(Vector.LEFT[0])");
                put(
                    x,
                    g.a,
                    Expr::<Array<Str>>::from(vec![lit("0"), lit("1"), lit("2")]),
                );
                x.comment(".split(\"\\uEC51\")");
                put(
                    x,
                    g.a,
                    Expr::<Array<Str>>::from(vec![lit("0(1.00, 0.00, 0.00)"), lit("123")]),
                );
            });
        });
    }

    fn options(&self) -> CompileOptions {
        let mut options = size();
        options.aggressive_size = true;
        options.tags = true;
        options
    }
}

#[test]
#[wasm_bindgen_test]
fn strings() {
    // Allowed differences:
    // - the generated tag setup rule is named `<tx> / <fg> setup code` here, without the two
    //   credit comments, and the size header comes before it;
    // - small string arrays stay arrays: this optimizer splits a string only when that is
    //   cheaper by its own cost model, where the reference always splits (the 450-item array
    //   is left out of the comparison for the same reason).
    let output = Strings.compile(Lang::EnUs).expect("compiles").code;
    let drop_array = |code: &str| {
        code.lines()
            .filter(|line| {
                !line.contains("Custom String(\"a\"), Custom String(\"a\"), Custom String(\"a\")")
                    && !line.contains("Custom String(\"a0a0a0a0a0a0")
            })
            .collect::<Vec<_>>()
            .join("\n")
    };
    let setup = "OverPy <\u{AD}tx> / <\u{AD}fg> setup code";
    let expected = include_str!("fixtures/reference/strings.txt")
        .replace(
            &format!("rule (\"{setup}\")"),
            "//Optimize for size enabled\nrule (\"<\u{AD}tx> / <\u{AD}fg> setup code\")",
        )
        .replace("        \"By Zezombye\"\n", "")
        .replace(
            "        \"More info: https://workshop.codes/wiki/articles/tx-reference-sheet\"\n",
            "",
        )
        .replacen("//Optimize for size enabled\nrule (\"strings\")", "rule (\"strings\")", 1)
        .replace("        //Optimize for size enabled\n", "")
        .replace(
            "String Split(Custom String(\"a0b\"), First Of(Null))",
            "Array(Custom String(\"a\"), Custom String(\"b\"))",
        )
        .replace(
            "String Split(Custom String(\"0(1.00, 0.00, 0.00)1(1.00, 0.00, 0.00)2\"), First Of(Left))",
            "Array(Custom String(\"0\"), Custom String(\"1\"), Custom String(\"2\"))",
        )
        .replace(
            "String Split(Custom String(\"0(1.00, 0.00, 0.00)\u{EC51}123\"), Custom String(\"\u{EC51}\"))",
            "Array(Custom String(\"0(1.00, 0.00, 0.00)\"), Custom String(\"123\"))",
        );
    assert_workshop_code_eq(&drop_array(&output), &drop_array(&expected));
}

// strings_args: every way to repeat up to six variables over 1 to 7 placeholders, in the order
// the reference's generator wrote them (restricted growth strings).
#[derive(GlobalVars)]
struct ArgsGlobals {
    #[ow(name = "A", index = 0)]
    a: Global<Num>,
    #[ow(name = "B", index = 1)]
    b: Global<Num>,
    #[ow(name = "C", index = 2)]
    c: Global<Num>,
    #[ow(name = "D", index = 3)]
    d: Global<Num>,
    #[ow(name = "E", index = 4)]
    e: Global<Num>,
    #[ow(name = "F", index = 5)]
    f: Global<Num>,
}

/// The patterns of `len` placeholders: each is a variable index that is at most one more than
/// the largest before it, with six variables at most.
fn arg_patterns(len: usize) -> Vec<Vec<usize>> {
    fn grow(prefix: &mut Vec<usize>, len: usize, out: &mut Vec<Vec<usize>>) {
        if prefix.len() == len {
            out.push(prefix.clone());
            return;
        }
        let next = prefix.iter().max().map_or(0, |m| m + 1);
        for value in 0..=next.min(5) {
            prefix.push(value);
            grow(prefix, len, out);
            prefix.pop();
        }
    }
    let mut out = Vec::new();
    grow(&mut Vec::new(), len, &mut out);
    out
}

struct StringsArgs;

impl Workshop for StringsArgs {
    type Globals = ArgsGlobals;
    type Players = ();

    fn build(&self, ws: &mut Builder, g: &ArgsGlobals, (): &()) {
        let vars = [g.a, g.b, g.c, g.d, g.e, g.f];
        for len in 1..=7 {
            let name = if len == 1 {
                "1 arg".to_owned()
            } else {
                format!("{len} args")
            };
            ws.rule(name, OngoingGlobal, |r, _| {
                r.actions(|x| {
                    for (i, pattern) in arg_patterns(len).iter().enumerate() {
                        let letters: Vec<String> = pattern
                            .iter()
                            .map(|v| char::from(b'A' + u8::try_from(*v).unwrap_or(0)).to_string())
                            .collect();
                        let mut pieces = vec![Piece::Text(Box::leak(
                            format!("{len}-{i} ({}): ", letters.join(",")).into_boxed_str(),
                        ))];
                        for (at, v) in pattern.iter().enumerate() {
                            if at > 0 {
                                pieces.push(Piece::Text(" "));
                            }
                            pieces.push(Piece::Arg(*v));
                        }
                        let args = (0..=*pattern.iter().max().unwrap_or(&0))
                            .map(|v| vars[v].get().erase())
                            .collect();
                        put(x, g.a, template(&pieces, args));
                    }
                });
            });
        }
    }
}

#[test]
#[wasm_bindgen_test]
fn strings_args() {
    check(
        &StringsArgs,
        include_str!("fixtures/reference/strings_args.txt"),
    );
}

// lowercase: `createCasedProgressBarIwt` is `cased_progress_bar_text_with`.
#[derive(GlobalVars)]
struct LowerGlobals {
    #[ow(name = "A", index = 0)]
    a: Global<Num>,
    #[ow(name = "B", index = 1)]
    b: Global<Num>,
}

struct Lowercase;

impl Workshop for Lowercase {
    type Globals = LowerGlobals;
    type Players = ();

    fn build(&self, ws: &mut Builder, g: &LowerGlobals, (): &()) {
        ws.rule("", OngoingGlobal, |r, _| {
            r.actions(|x| {
                x.set(g.a, 1234);
                x.cased_progress_bar_text_with(
                    3,
                    Expr::<Str>::from("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n<fg00FFFFFF><txb667> BREAKTHROUGH UPDATE 8.3.0 FOR OVERPY!</fg>\n\nFor the FIRST TIME EVER, you can use lowercase text\nin the Overwatch Workshop! No more shouting—just pure,\nclean typography exactly how you want it. But that’s not all...\n\n<fgFF00FFFF><txb666> DELVE INTO A NEW ERA OF OVERWATCH WORKSHOP TEXT</fg>\n\nFor the <fgFF0000FF>first time in history</fg>, the tapestry of text in Overwatch\nWorkshop expands beyond the rigid confines of ALL CAPS.\nWith the latest OverPy update, you can now weave\nlowercase letters seamlessly into your creations,\nunlocking a realm of expression never before possible.\n\n<fg80FF00FF><tx248B> FULL SUPPORT FOR COLOR TAGS & TEXTURES!</fg>\n\nCustomize your text like <fg00FF00FF>NEVER BEFORE</fg> with\nvibrant colors and eye-catching textures. Whether you’re\ndesigning a sleek UI, creating immersive story elements,\nor just flexing your creativity, this update changes everything.\n\n<fgFF8000FF><tx1f70d> The future of Workshop text starts NOW!</fg>\n\nThe ultimate workshop update: <fgFF0000FF>lowercase text</fg>."),
                    null::<PlayerOrPosition>(),
                    CasedTextArgs {
                        scale: Expr::from(1.5),
                        clipping: Clip::None.into(),
                        text_color: Color::White.into(),
                        reevaluation: ProgressWorldTextReeval::VisibilityPositionValuesAndColor
                            .into(),
                        spec_visibility: SpecVisibility::Always.into(),
                        ..CasedTextArgs::default()
                    },
                )
                .expect("a literal text");
                x.set(g.b, 567);
            });
        });
    }

    fn options(&self) -> CompileOptions {
        let mut options = CompileOptions::default();
        options.tags = true;
        options
    }
}

#[test]
#[wasm_bindgen_test]
fn lowercase() {
    // Allowed differences: the tag setup rule has its own name and no credit comments here (as
    // in `strings`), and the letters are dealt out to the overlaid progress-bar texts by this
    // implementation's own rule (and numbered 1 to 3, where the reference counts 0 to 2), so
    // the three texts are not compared text for text; `tests/cased.rs` covers them.
    let setup = "OverPy <\u{AD}tx> / <\u{AD}fg> setup code";
    let expected = include_str!("fixtures/reference/lowercase.txt")
        .replace(
            &format!("rule (\"{setup}\")"),
            "rule (\"<\u{AD}tx> / <\u{AD}fg> setup code\")",
        )
        .replace("        \"By Zezombye\"\n", "")
        .replace(
            "        \"More info: https://workshop.codes/wiki/articles/tx-reference-sheet\"\n",
            "",
        );
    let output = Lowercase.compile(Lang::EnUs).expect("compiles").code;
    let is_text = |line: &&str| line.contains("Create Progress Bar In-World Text(");
    let without = |code: &str| {
        code.lines()
            .filter(|line| !is_text(line))
            .collect::<Vec<_>>()
            .join("\n")
    };
    assert_workshop_code_eq(&without(&output), &without(&expected));
    assert_eq!(output.lines().filter(is_text).count(), 3);
    assert_eq!(expected.lines().filter(is_text).count(), 3);
}

// compression
#[derive(GlobalVars)]
struct CompressGlobals {
    #[ow(name = "B", index = 1)]
    b: Global<Array<Any>>,
    #[ow(name = "C", index = 2)]
    c: Global<Array<Any>>,
    #[ow(name = "D", index = 3)]
    d: Global<Array<Any>>,
    #[ow(name = "E", index = 4)]
    e: Global<Array<Any>>,
    #[ow(name = "F", index = 5)]
    f: Global<Array<Any>>,
    #[ow(name = "G", index = 6)]
    g: Global<Array<Any>>,
    #[ow(name = "H", index = 7)]
    h: Global<Array<Any>>,
}

struct Compression;

impl Workshop for Compression {
    type Globals = CompressGlobals;
    type Players = ();

    fn build(&self, ws: &mut Builder, g: &CompressGlobals, (): &()) {
        let packed = |v: Global<Array<Any>>| v.get().erase().cast::<Str>();
        // The reference variables are untyped; these hold a packed string, then an array.
        let store = |x: &mut Block, v: Global<Array<Any>>, value: Expr<Any>| {
            x.set(v, value.cast::<Array<Any>>());
        };
        ws.rule("compression", OngoingGlobal, |r, _| {
            r.actions(|x| {
                let numbers = || vec![1.0, 2.53, 3.0, -543.0];
                let vectors = || {
                    vec![
                        vect(1, 0, 3),
                        vect(-5, -7.342, 0),
                        vect(-53, 4998.999, -4998.999),
                    ]
                };
                store(x, g.b, (compress::<Num>(numbers()).expect("fits")).erase());
                store(x, g.b, (decompress_numbers(packed(g.b))).erase());
                store(
                    x,
                    g.c,
                    (compressed::<Num>(numbers()).expect("fits")).erase(),
                );
                store(
                    x,
                    g.d,
                    (compress::<Vector>(vectors()).expect("fits")).erase(),
                );
                store(x, g.d, (decompress_vectors(packed(g.d))).erase());
                store(
                    x,
                    g.e,
                    (compressed::<Vector>(vec![vect(1, 2, 3), vect(43, 65, 9)]).expect("fits"))
                        .erase(),
                );
                store(
                    x,
                    g.f,
                    (compress::<Num>(vec![123.0]).expect("fits")).erase(),
                );
                store(x, g.f, (decompress_numbers(packed(g.f))).erase());
                store(
                    x,
                    g.g,
                    (compress::<Vector>(vec![vect(0, 0, 0)]).expect("fits")).erase(),
                );
                store(x, g.g, (decompress_vectors(packed(g.g))).erase());
                store(
                    x,
                    g.h,
                    (compressed::<Vector>(vec![
                        vect(1, 2, 3),
                        vect(-5, 0, 0),
                        vect(-53, 4998.999, -4998.999),
                    ])
                    .expect("fits"))
                    .erase(),
                );
                for (name, var) in [
                    ("B", g.b),
                    ("C", g.c),
                    ("D", g.d),
                    ("E", g.e),
                    ("F", g.f),
                    ("G", g.g),
                    ("H", g.h),
                ] {
                    x.debug_value(name, var.get());
                }
            });
        });
    }
}

#[test]
#[wasm_bindgen_test]
fn compression() {
    // Allowed differences: `debug` pads its box with 170 ordinary spaces where the reference
    // used em spaces (split over the same two pieces here), and the optimizer reads
    // `x == []` as `Not(x)`, as in `arrays`.
    let em = |n: usize| "\u{2003}".repeat(n);
    let expected = include_str!("fixtures/reference/compression.txt")
        .replace(&em(118), &" ".repeat(118))
        .replace(&em(52), &" ".repeat(52))
        .replace(
            "And(Compare(Current Array Element, ==, Empty Array)",
            "And(Not(Current Array Element)",
        );
    check(&Compression, &expected);
}

// shionSettings: a mod with settings and no rules.
struct ShionSettings;

impl Workshop for ShionSettings {
    type Globals = ();
    type Players = ();

    fn build(&self, _: &mut Builder, (): &(), (): &()) {}

    fn settings(&self) -> Settings {
        let shion = Values::new()
            .with("secondaryFireCooldown%", SettingValue::Percent(113.0))
            .with("ability1Cooldown%", SettingValue::Percent(81.0))
            .with("ability1Distance%", SettingValue::Percent(110.0))
            .with("ability2Cooldown%", SettingValue::Percent(86.0))
            .with("ability2Kb%", SettingValue::Percent(119.0))
            .with("ability2Duration%", SettingValue::Percent(83.0))
            .with("ability2Speed%", SettingValue::Percent(109.0))
            .with("enableInfiniteUlt", true);
        Settings::default()
            .with_mode(ModeKey::Mode(Gamemode::Skirmish), ModeSettings::default())
            .with_heroes(
                HeroTeam::AllTeams,
                HeroSettings::default().hero(Hero::Shion, shion),
            )
    }
}

#[test]
#[wasm_bindgen_test]
fn shionsettings() {
    check(
        &ShionSettings,
        include_str!("fixtures/reference/shionSettings.txt"),
    );
}

// extensions
#[derive(GlobalVars)]
struct ExtGlobals {
    #[ow(name = "Afqdsfdsq", index = 0)]
    afqdsfdsq: Global<Num>,
}

struct Extensions;

impl Workshop for Extensions {
    type Globals = ExtGlobals;
    type Players = ();

    fn build(&self, ws: &mut Builder, g: &ExtGlobals, (): &()) {
        ws.rule("Rule 1", OngoingEachPlayer, |r, ev| {
            let me = ev.event_player();
            let as_position = || me.clone().erase().cast::<Position>();
            r.actions(|x| {
                x.create_beam_with(
                    all_players(),
                    Beam::BrigitteFlailChain,
                    as_position(),
                    as_position(),
                    CreateBeamArgs {
                        color: Color::White.into(),
                        reevaluation: EffectReeval::VisibilityPositionAndRadius.into(),
                    },
                );
                x.create_effect_with(
                    all_players(),
                    Effect::EchoFocusingBeamSound,
                    Color::White,
                    me.clone(),
                    g.afqdsfdsq.get(),
                    CreateEffectArgs {
                        reevaluation: EffectReeval::VisibilityPositionAndRadius.into(),
                    },
                );
                x.play_effect(
                    all_players(),
                    DynamicEffect::AnaBioticGrenadeIncreasedHealingSound,
                    Color::White,
                    as_position(),
                    1,
                );
            });
        });
    }

    fn settings(&self) -> Settings {
        Settings::default()
            .with_lobby(Lobby::default().team1_slots(0).team2_slots(0))
            .with_mode(
                ModeKey::Mode(Gamemode::TeamDeathmatch),
                ModeSettings::default().maps(MapSelection::Enabled(
                    [
                        Map::WorkshopChamber,
                        Map::WorkshopExpanse,
                        Map::WorkshopGreenScreen,
                        Map::WorkshopIsland,
                    ]
                    .map(MapRef::new)
                    .to_vec(),
                )),
            )
            .with_extension(Extension::BeamEffects)
            .with_extension(Extension::BeamSounds)
            .with_extension(Extension::BuffAndDebuffSounds)
            .with_extension(Extension::SpawnMoreDummyBots)
    }
}

#[test]
#[wasm_bindgen_test]
fn extensions() {
    check(
        &Extensions,
        include_str!("fixtures/reference/extensions.txt"),
    );
}

// workshopSettings: `A` takes any value, so it is untyped.
#[derive(GlobalVars)]
struct WsGlobals {
    #[ow(name = "A", index = 0)]
    a: Global<Num>,
}

struct WorkshopSettings;

impl Workshop for WorkshopSettings {
    type Globals = WsGlobals;
    type Players = ();

    fn build(&self, ws: &mut Builder, g: &WsGlobals, (): &()) {
        use workshop_setting as w;
        ws.rule("workshop settings", OngoingGlobal, |r, _| {
            r.actions(|x| {
                x.comment("bools");
                put(x, g.a, Expr::<Bool>::from(w::toggle("", "boolean", true)));
                x.comment("ints");
                put(x, g.a, Expr::<Num>::from(w::int("int", "int", 3)));
                put(
                    x,
                    g.a,
                    Expr::<Num>::from(w::int("int", "unsigned int", 3).unsigned()),
                );
                put(
                    x,
                    g.a,
                    Expr::<Num>::from(w::int("int", "signed int", 3).signed()),
                );
                put(
                    x,
                    g.a,
                    Expr::<Num>::from(w::int("int", "int<2;19>", 3).range(2..=19)),
                );
                put(
                    x,
                    g.a,
                    Expr::<Num>::from(w::int("int", "int<6;>", 3).range(6..=i32::MAX)),
                );
                put(
                    x,
                    g.a,
                    Expr::<Num>::from(w::int("int", "int<;6>", 3).range(i32::MIN..=6)),
                );
                x.comment("floats");
                put(x, g.a, Expr::<Num>::from(w::real("float", "float", 3.5)));
                put(
                    x,
                    g.a,
                    Expr::<Num>::from(w::real("float", "unsigned float", 3.5).unsigned()),
                );
                put(
                    x,
                    g.a,
                    Expr::<Num>::from(w::real("float", "signed float", 3.5).signed()),
                );
                put(
                    x,
                    g.a,
                    Expr::<Num>::from(w::real("float", "float<2.5;19>", 3.5).range(2.5..=19.0)),
                );
                put(
                    x,
                    g.a,
                    Expr::<Num>::from(
                        w::real("float", "float<6;>", 3.5).range(6.0..=9_999_999_999_999.0),
                    ),
                );
                put(
                    x,
                    g.a,
                    Expr::<Num>::from(
                        w::real("float", "float<;6>", 3.5).range(-9_999_999_999_999.0..=6.0),
                    ),
                );
                x.comment("heroes");
                put(
                    x,
                    g.a,
                    Expr::<Hero>::from(w::hero("hero", "hero", Hero::Ana)),
                );
                put(
                    x,
                    g.a,
                    Expr::<Hero>::from(w::hero("hero", "hero", Hero::WreckingBall).sort(3)),
                );
                x.comment("enums");
                put(
                    x,
                    g.a,
                    Expr::<Num>::from(w::combo("enum", "enum", ["a", "b\nc\nd"], 0)),
                );
                // The reference spells the option styles `p"a"`, `b"b"` and `w"c"`; a combo
                // option is a plain string here, so the styled text is written out.
                put(
                    x,
                    g.a,
                    Expr::<Num>::from(w::combo("enum", "enum", ["a", "\u{392}", "\u{FF43}"], 2)),
                );
                x.comment("texture test");
                put(
                    x,
                    g.a,
                    Expr::<Num>::from(w::combo(
                        "<txC00000000008E02>",
                        "<txC00000000008E02>",
                        ["<txC00000000008E02>"],
                        0,
                    )),
                );
                x.comment("char workaround");
                let special = "!\"#$%&'()*+,-./:\n;<=>?[\\]^_`{|}~";
                put(
                    x,
                    g.a,
                    Expr::<Num>::from(w::combo(
                        special,
                        special,
                        ["{test special chars:}", special],
                        0,
                    )),
                );
            });
        });
    }

    fn options(&self) -> CompileOptions {
        size()
    }
}

#[test]
#[wasm_bindgen_test]
fn workshopsettings() {
    // Allowed differences: a second hero setting with the same name gets a hidden suffix so the
    // two stay distinct, where the reference printed the name twice; and an integer range is
    // `i32`, so an open end is `i32::MAX` or `i32::MIN` rather than 13 nines.
    check_with(
        &WorkshopSettings,
        include_str!("fixtures/reference/workshopSettings.txt"),
        &[
            ("3, 6, 9999999999999, 0", "3, 6, 2147483647, 0"),
            ("3, -9999999999999, 6, 0", "3, -2147483648, 6, 0"),
            (
                "Custom String(\"hero\"), Custom String(\"hero\"), Wrecking Ball",
                "Custom String(\"hero\"), Custom String(\"hero\u{1}\"), Wrecking Ball",
            ),
        ],
    );
}

// customGameSettings: the settings are the whole mod. The `unknown gamemode` entry has no
// equivalent (a mode is a `Gamemode`), so it is dropped from the expected output.
struct CustomGameSettings;

impl Workshop for CustomGameSettings {
    type Globals = ();
    type Players = ();

    fn build(&self, _: &mut Builder, (): &(), (): &()) {}

    fn settings(&self) -> Settings {
        let pct = SettingValue::Percent;
        let choice = |text: &str| SettingValue::Choice(text.to_owned());
        let version = "1.4.3";
        let description = format!(
            "Tower Meifense - BETA do not copy! by Zezombye v{version}\n\nThis is an extremely long string that takes advantage of the fact that the description can have up to 512 characters (not bytes). \u{a4}\u{a7}\u{20ac} Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat naz|"
        );
        let hero = |values: Vec<(&str, SettingValue)>| {
            values
                .into_iter()
                .fold(Values::new(), |all, (key, value)| all.with(key, value))
        };
        Settings::default()
            .with_main(Main {
                mode_name: Some("\u{ff34}\u{ff4f}\u{ff57}\u{ff45}\u{ff52}\u{2001}\u{ff2d}\u{ff45}\u{ff49}\u{ff46}\u{ff45}\u{ff4e}\u{ff53}\u{ff45}".to_owned()),
                description: Some(description),
            })
            .with_lobby(
                Lobby::default()
                    .raw("mapRotation", choice("paused"))
                    .raw("spectatorSlots", 6)
                    .raw("returnToLobby", "unknown value")
                    .raw("swapTeamsAfterMatch", false)
                    .raw("unknownSetting", "abc"),
            )
            .with_mode(
                ModeKey::Mode(Gamemode::Assault),
                ModeSettings::default()
                    .maps(MapSelection::Enabled(vec![MapRef::new(Map::TempleOfAnubis)]))
                    .raw("captureSpeed%", pct(60.0))
                    .raw("unknown value", 60),
            )
            .with_mode(
                ModeKey::Mode(Gamemode::Escort),
                ModeSettings::default()
                    .maps(MapSelection::Enabled(vec![MapRef::new(Map::Dorado)]))
                    .raw("payloadSpeed%", pct(200.0)),
            )
            .with_mode(
                ModeKey::General,
                ModeSettings::default()
                    .raw("gamemodeStartTrigger", choice("manual"))
                    .raw("heroLimit", choice("off"))
                    .raw("respawnTime%", pct(100.0))
                    .raw("spawnHealthPacks", choice("disabled")),
            )
            .with_heroes(
                HeroTeam::Team1,
                HeroSettings::default().selection(HeroSelection::Enabled(vec![
                    Hero::Ashe,
                    Hero::Bastion,
                    Hero::Torbjorn,
                    Hero::Widowmaker,
                ])),
            )
            .with_heroes(
                HeroTeam::Team2,
                HeroSettings::default().hero(
                    Hero::Mei,
                    hero(vec![
                        ("ability1Cooldown%", pct(200.0)),
                        ("ability2Cooldown%", pct(200.0)),
                        ("enableSecondaryFire", false.into()),
                        ("primaryFireFreezeRate%", pct(50.0)),
                        ("unknown value", "123%".into()),
                    ]),
                ),
            )
            .with_heroes(
                HeroTeam::AllTeams,
                HeroSettings::default()
                    .hero(
                        Hero::Anran,
                        hero(vec![
                            ("ability1Cooldown%", pct(50.0)),
                            ("ability2Cooldown%", pct(50.0)),
                            ("damageDealt%", pct(200.0)),
                            ("ultGen%", pct(120.0)),
                        ]),
                    )
                    .hero(
                        Hero::Ashe,
                        hero(vec![("ammoClipSize%", pct(200.0)), ("ultDuration%", pct(400.0))]),
                    )
                    .hero(Hero::Bastion, hero(vec![("ammoClipSize%", pct(200.0))]))
                    .hero(
                        Hero::Domina,
                        hero(vec![
                            ("ability1KB%", pct(125.0)),
                            ("passiveHeal%", pct(125.0)),
                            ("ultBarrierHealth%", pct(200.0)),
                            ("ammoClipSize%", pct(200.0)),
                        ]),
                    )
                    .hero(
                        Hero::Emre,
                        hero(vec![
                            ("ability2Kb%", pct(50.0)),
                            ("ability1Duration%", pct(200.0)),
                            ("ability1Heat%", pct(125.0)),
                            ("ultKb%", pct(150.0)),
                            ("enableInfiniteUlt", true.into()),
                        ]),
                    )
                    .hero(
                        Hero::JetpackCat,
                        hero(vec![
                            ("primaryFireRange%", pct(150.0)),
                            ("secondaryFireMaximumTime%", pct(200.0)),
                            ("enablePassiveUnlimitedFuel", true.into()),
                        ]),
                    )
                    .hero(
                        Hero::Mizuki,
                        hero(vec![
                            ("ability1Duration%", pct(200.0)),
                            ("enableRolePassive", false.into()),
                        ]),
                    )
                    .hero(
                        Hero::Vendetta,
                        hero(vec![
                            ("secondaryFireRechargeRate%", pct(120.0)),
                            ("ability1Cooldown%", pct(10.0)),
                            ("ability1Distance%", pct(120.0)),
                            ("ability2Cooldown%", pct(10.0)),
                            ("ability2Distance%", pct(120.0)),
                            ("ultGen%", pct(120.0)),
                        ]),
                    )
                    .hero(
                        Hero::Widowmaker,
                        hero(vec![("damageDealt%", pct(120.0)), ("ability1Cooldown%", pct(10.0))]),
                    )
                    .hero(
                        Hero::JunkerQueen,
                        hero(vec![("secondaryFireCooldown%", pct(120.0))]),
                    ),
            )
    }
}

#[test]
#[wasm_bindgen_test]
fn customgamesettings() {
    // `unknown gamemode` (between Assault and Escort) is not expressible.
    let fixture = include_str!("fixtures/reference/customGameSettings.txt");
    let start = fixture
        .find("\t\tunknown gamemode\n")
        .expect("the unknown mode");
    let end = start + fixture[start..].find("\t\tEscort").expect("Escort follows");
    let expected = format!("{}{}", &fixture[..start], &fixture[end..]);
    // Allowed difference: Domina's `ability1KB%` (written with a capital B) is printed where it
    // was written, first, rather than after the keys spelled exactly like the schema.
    let knockback = "\t\t\t\tSonic Repulsors Knockback Scalar: 125%\n";
    let heal = "\t\t\t\tReconstruction Heal Scalar: 125%\n";
    let expected = expected
        .replace(knockback, "")
        .replace(heal, &format!("{knockback}{heal}"));
    check(&CustomGameSettings, &expected);
}

// translations
#[derive(GlobalVars)]
struct TrGlobals {
    #[ow(name = "A", index = 0)]
    a: Global<Any>,
    #[ow(name = "B", index = 1)]
    b: Global<Any>,
    #[ow(name = "C", index = 2)]
    c: Global<Any>,
    #[ow(name = "D", index = 3)]
    d: Global<Any>,
    #[ow(name = "E", index = 4)]
    e: Global<Any>,
    #[ow(name = "G", index = 6)]
    g: Global<Any>,
    #[ow(name = "H", index = 7)]
    h: Global<Any>,
    #[ow(name = "I", index = 8)]
    i: Global<Any>,
    #[ow(name = "K", index = 10)]
    k: Global<Any>,
    #[ow(name = "L", index = 11)]
    l: Global<Any>,
    #[ow(name = "N", index = 13)]
    n: Global<Any>,
    #[ow(name = "Y", index = 24)]
    y: Global<Any>,
    #[ow(name = "Z", index = 25)]
    z: Global<Any>,
}

fn translation_settings() -> Settings {
    Settings::default()
        .with_lobby(
            Lobby::default()
                .raw("allowPlayersInQueue", true)
                .raw("enableMatchVoiceChat", true),
        )
        .with_mode(
            ModeKey::Mode(Gamemode::Skirmish),
            ModeSettings::default().maps(MapSelection::Enabled(vec![MapRef::new(
                Map::WorkshopChamber,
            )])),
        )
}

struct Translated;

impl Workshop for Translated {
    type Globals = TrGlobals;
    type Players = ();

    fn build(&self, ws: &mut Builder, g: &TrGlobals, (): &()) {
        let (a, b, c, e) = (|| g.a.get(), || g.b.get(), || g.c.get(), || g.e.get());
        let (y, z) = (|| g.y.get().cast::<Num>(), || g.z.get().cast::<Num>());
        let source = |x: &mut Block, text: &str, value: Expr<Str>| x.debug_value(text, value);
        ws.rule("owo\nuwu", OngoingGlobal, |r, _| {
            r.actions(|x| {
                x.set(g.a, t!("some string"));
                source(x, "t\"some string\"", t!("some string"));
                x.set(g.c, t_ctx!("context\nowo", "some string"));
                x.set(g.d, t_ctx!("context\nowo", "some string"));
                x.set(g.e, t!("\u{AD}\n\nstring with whitespace \u{AD}"));
                source(x, "t\"line 1 \\n line 2   \\n line 3  \"", t!("line 1 \n line 2   \n line 3  "));
                x.set(g.g, t!("line 1 \n    line 2 \n line 3  "));
                x.set(
                    g.h,
                    t!("a very very long string that will be split dsqljmfk ljkfqsdjlk qjlk lkjfsdq lkqjs mqslkdj fqsmklj qsmlkfj qsklfj qslkfjm qsmljkdf qslkmjdf lkqmjsdf lmkqjsdf lkmjqsfd lkmjqsdlfmkj qlsmjkdf lkmjqsdf lkjmqsldkmfj qslmkjdf lmkjqsfd lkmjqsfldmkj qsjmlkfd lmqkjsdf mljkqsfml dkjqsmlkjfd qsmlkjfd mlkqjsfd jlmkqsfmlkj qslkmjf mljksf"),
                );
                x.set(g.z, 1234);
                x.set(g.y, 5432);
                x.if_(ws_fmt!("{}", host_player()).eq("Zezombye"), |x| x.set(g.k, 2));
                x.set(g.i, t!("some formatted string {}", z() + 1));
                x.set(
                    g.i,
                    t!("some formatted string {} which is relatively long qjdsfio piqsjf ioqpjsfd joiqsfd poijqsf pijo", z()),
                );
                x.big_message(t!("multiple formatters {} {} owo", z(), y()));
                x.big_message(t!("multiple formatters {0} {1} owo", z(), y()));
                x.big_message(t!("multiple formatters {1} {0} owo", y(), z()));
                x.big_message(t!("multiple formatters {0} {1} {2} {3} owo", a(), b(), c(), e()));
                x.big_message(t!(
                    "multiple formatters {} with a very long string ifdjopq ifjsqop idjfq spoifj poifjeqs iopj{} owo",
                    z(),
                    y()
                ));
                x.big_message(t!(
                    "multiple formatters {0} with a very long string ifdjopq ifjsqop idjfq spoifj poifjeqs iopj {1} owo",
                    z(),
                    y()
                ));
                x.big_message(t!(
                    "multiple formatters {1} with a very long string ifdjopq ifjsqop idjfq spoifj poifjeqs iopj {0} owo",
                    z(),
                    y()
                ));
                x.set(g.l, t!("653453[]|.<>$= \u{a4}$"));
                x.set(g.n, "untranslated string");
                source(x, "_(A)", tr(a()));
                x.hud_header_with(tr(a()), HudArgs { reevaluation: HudReeval::VisibilityAndString.into(), ..HudArgs::default() });
                x.hud_header_with(tr(g.h.get()), HudArgs { reevaluation: HudReeval::VisibilityAndString.into(), ..HudArgs::default() });
                x.hud_header_with(a(), HudArgs { reevaluation: HudReeval::VisibilityAndString.into(), ..HudArgs::default() });
                x.hud_header_with(tr(g.n.get()), HudArgs { reevaluation: HudReeval::VisibilityAndString.into(), ..HudArgs::default() });
                let strings = IntoExpr::<Array<Array<Str>>>::into_expr(vec![
                    tr_raw(t!("string 1")),
                    tr_raw(t!("string 2")),
                ]);
                x.big_message(tr(strings.get(z())));
            });
        });
        ws.rule("", OngoingEachPlayer, |r, ev| {
            r.condition(ev.event_player().is_holding_button(Button::Interact));
            r.actions(|x| {
                let text = || ws_fmt!("{}{}", tr(a()), input_binding_string(Button::Interact));
                x.big_message(text());
                x.small_message(text());
            });
        });
    }

    fn settings(&self) -> Settings {
        translation_settings()
    }

    fn translations(&self) -> Option<Translations> {
        let mut catalog = Catalog::default();
        for (lang, po) in [
            (
                TranslationLang::Fr,
                include_str!("fixtures/reference/translations.fr.po"),
            ),
            (
                TranslationLang::Es,
                include_str!("fixtures/reference/translations.es.po"),
            ),
            (
                TranslationLang::ZhCn,
                include_str!("fixtures/reference/translations.zh_cn.po"),
            ),
        ] {
            catalog.merge_po(lang, po).expect("parses");
        }
        Some(
            Translations::new([
                TranslationLang::En,
                TranslationLang::Fr,
                TranslationLang::Es,
                TranslationLang::ZhCn,
            ])
            .with_catalog(catalog)
            .keep_unused(true),
        )
    }

    fn options(&self) -> CompileOptions {
        let mut options = CompileOptions::default();
        options.tags = true;
        options
    }
}

#[test]
#[wasm_bindgen_test]
fn translations() {
    // Allowed differences:
    // - the tag setup rule has its own name and no credit comments, and the two internal
    //   variables take the last two slots in the other order (as in `strings`);
    // - `debug` pads with ordinary spaces where the reference used em spaces (as in
    //   `compression`);
    // - the formatter test `{0} {1} {2} {4}` is written `{0} {1} {2} {3}` here (an argument
    //   no placeholder uses is an error), which is a different translation key, so the one
    //   message line that uses it is not compared.
    let setup = "OverPy <\u{AD}tx> / <\u{AD}fg> setup code";
    let expected = include_str!("fixtures/reference/translations.txt")
        .replace(
            &format!("rule (\"{setup}\")"),
            "rule (\"<\u{AD}tx> / <\u{AD}fg> setup code\")",
        )
        .replace("        \"By Zezombye\"\n", "")
        .replace(
            "        \"More info: https://workshop.codes/wiki/articles/tx-reference-sheet\"\n",
            "",
        )
        .replace(
            "        126: __holygrail__\n        127: __overpyTranslationHelper__\n",
            "        126: __overpyTranslationHelper__\n        127: __holygrail__\n",
        )
        .replace('\u{2003}', " ");
    let output = Translated.compile(Lang::EnUs).expect("compiles").code;
    let four = "String Replace(String Replace(String Replace(String Replace(";
    let without = |code: &str| {
        code.lines()
            .filter(|line| !line.contains(four))
            .collect::<Vec<_>>()
            .join("\n")
    };
    assert_workshop_code_eq(&without(&output), &without(&expected));
}

// translations2: the language is kept in a player variable.
#[derive(GlobalVars)]
struct Tr2Globals {
    #[ow(name = "A", index = 0)]
    a: Global<Any>,
}

struct Translated2;

impl Workshop for Translated2 {
    type Globals = Tr2Globals;
    type Players = ();

    fn build(&self, ws: &mut Builder, g: &Tr2Globals, (): &()) {
        ws.rule("owo\nuwu", OngoingGlobal, |r, _| {
            r.actions(|x| {
                x.set(g.a, t!("host player: {}", host_player()));
                x.debug_value("t\"some string\"", t!("some string"));
            });
        });
        ws.rule("", OngoingEachPlayer, |r, ev| {
            r.condition(ev.event_player().is_holding_button(Button::Interact));
            r.actions(|x| {
                let text = || {
                    ws_fmt!(
                        "{}{}",
                        tr(g.a.get()),
                        input_binding_string(Button::Interact)
                    )
                };
                x.big_message(text());
                x.small_message(text());
            });
        });
    }

    fn settings(&self) -> Settings {
        translation_settings()
    }

    fn translations(&self) -> Option<Translations> {
        use TranslationLang::{De, En, EsEs, Fr, It, Ja, Ko, Pl, Pt, Ru, Th, Tr, Zh};
        let mut catalog = Catalog::default();
        for (lang, po) in [
            (En, include_str!("fixtures/reference/translations2.en.po")),
            (
                EsEs,
                include_str!("fixtures/reference/translations2.es_es.po"),
            ),
            (Fr, include_str!("fixtures/reference/translations2.fr.po")),
            (It, include_str!("fixtures/reference/translations2.it.po")),
            (Ja, include_str!("fixtures/reference/translations2.ja.po")),
            (Ko, include_str!("fixtures/reference/translations2.ko.po")),
            (Pl, include_str!("fixtures/reference/translations2.pl.po")),
            (Pt, include_str!("fixtures/reference/translations2.pt.po")),
            (Ru, include_str!("fixtures/reference/translations2.ru.po")),
            (Th, include_str!("fixtures/reference/translations2.th.po")),
            (Tr, include_str!("fixtures/reference/translations2.tr.po")),
            (Zh, include_str!("fixtures/reference/translations2.zh.po")),
        ] {
            catalog.merge_po(lang, po).expect("parses");
        }
        Some(
            Translations::new([De, Fr, Zh, EsEs, Tr, Th, Ja, Ko, Pl, Pt, Ru, It, En])
                .with_catalog(catalog)
                .with_player_var(PlayerVarMode::Default),
        )
    }

    fn options(&self) -> CompileOptions {
        let mut options = CompileOptions::default();
        options.tags = true;
        options
    }
}

#[test]
#[wasm_bindgen_test]
fn translations2() {
    // Allowed differences, as in `translations`: the tag setup rule's name and comments, the
    // order of the two internal variables, and `debug`'s padding; also the name of the rule
    // that detects the player's language, and one comment inside it.
    let setup = "OverPy <\u{AD}tx> / <\u{AD}fg> setup code";
    let expected = include_str!("fixtures/reference/translations2.txt")
        .replace(
            &format!("rule (\"{setup}\")"),
            "rule (\"<\u{AD}tx> / <\u{AD}fg> setup code\")",
        )
        .replace("        \"By Zezombye\"\n", "")
        .replace(
            "        \"More info: https://workshop.codes/wiki/articles/tx-reference-sheet\"\n",
            "",
        )
        .replace(
            "126: __holygrail__\n        127: __overpyTranslationHelper__\n",
            "126: __overpyTranslationHelper__\n        127: __holygrail__\n",
        )
        .replace(
            "OverPy translation setup - Determine the player's language",
            "Translation setup - detect player language",
        )
        .replace(
            "        \"Have a tolerance of 1/100 of a degree to account for precision errors\"\n",
            "",
        )
        .replace('\u{2003}', " ");
    check(&Translated2, &expected);
}
