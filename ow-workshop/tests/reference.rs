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
use ow_workshop::Kind;
use ow_workshop::prelude::*;
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

stub!(comments, "comments", "#101");
stub!(compression, "compression", "#101");
stub!(customgamesettings, "customGameSettings", "#101");
stub!(enums, "enums", "#101");
stub!(extensions, "extensions", "#101");
stub!(functions, "functions", "#101");
stub!(gotos, "gotos", "#101");
stub!(loops, "loops", "#101");
stub!(lowercase, "lowercase", "#101");
stub!(ruleprefix, "rulePrefix", "#101");
stub!(rules, "rules", "#101");
stub!(shionsettings, "shionSettings", "#101");
stub!(strings, "strings", "#101");
stub!(strings_args, "strings_args", "#101");
stub!(translations, "translations", "#101");
stub!(translations2, "translations2", "#101");
stub!(workshopsettings, "workshopSettings", "#101");

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
