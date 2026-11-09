//! Ports of the reference fixtures (`tests/fixtures/reference/`) to `Workshop` impls.
//!
//! Each test builds the equivalent mod in Rust and compares its output to the fixture's expected
//! `.txt`, ignoring whitespace-only lines and CRLF. Until a fixture is ported its test is an
//! ignored stub (#99); #100 and #101 replace the stubs, and #109 then moves the expected output
//! into `insta` snapshots and deletes the fixtures.

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

stub!(arrays, "arrays", "#101");
stub!(builtinenums, "builtinenums", "#101");
stub!(builtinmacros, "builtinmacros", "#101");
stub!(comments, "comments", "#101");
stub!(compression, "compression", "#101");
stub!(conditions, "conditions", "#101");
stub!(customgamesettings, "customGameSettings", "#101");
stub!(dicts, "dicts", "#101");
stub!(elementsaving, "elementSaving", "#101");
stub!(enums, "enums", "#101");
stub!(eventvars, "eventVars", "#101");
stub!(extensions, "extensions", "#101");
stub!(functions, "functions", "#101");
stub!(getcurrentmap, "getCurrentMap", "#101");
stub!(gotos, "gotos", "#101");
stub!(loops, "loops", "#101");
stub!(lowercase, "lowercase", "#101");
stub!(operators, "operators", "#101");
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
fn check_ignoring_labels(mode: &impl Workshop, expected: &str) {
    let output = mode.compile(Lang::EnUs).expect("compiles");
    // Also allowed here: indentation after a label, and `0` rather than `Null` as the table filler.
    let flat = |text: &str| {
        without_labels(text)
            .lines()
            .map(|line| line.trim_start().replace(", Null,", ", 0,"))
            .collect::<Vec<_>>()
            .join("\n")
    };
    assert_workshop_code_eq(&flat(&output.code), &flat(expected));
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
