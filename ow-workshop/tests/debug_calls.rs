//! Debug-only calls (`debug!`, `print`, `debug_only`): in a release build the mod is what it
//! would be without them.
#![cfg(not(target_arch = "wasm32"))]

use ow_workshop::prelude::*;

#[derive(GlobalVars)]
struct Globals {
    flag: Global<Num>,
}

/// A mod that skips over a stretch of code; `calls` adds debug-only calls to that stretch and
/// to the rule around it.
struct Mode {
    calls: bool,
}

impl Workshop for Mode {
    const NAME: &'static str = "Test";
    const DESCRIPTION: &'static str = "Test mode";

    fn settings(&self) -> ow_workshop::Settings {
        ow_workshop::Settings::unchecked()
    }

    type Globals = Globals;
    type Players = ();

    fn build(&self, ws: &mut Builder, g: &Globals, (): &()) {
        ws.rule("Run", OngoingGlobal, |r, _| {
            r.actions(|b| {
                if self.calls {
                    b.print("starting");
                }
                let label = b.label();
                b.skip_to_if(g.flag.get().eq(1), label);
                if self.calls {
                    debug!(b, g.flag.get());
                    b.debug_only(|b| {
                        b.set(g.flag, 5);
                        b.big_message("test teleport");
                    });
                    b.if_(true, |b| b.print("nested"));
                }
                b.set(g.flag, 2);
                b.place(label);
                b.set(g.flag, 3);
                if self.calls {
                    b.print("done");
                }
            });
        });
        if self.calls {
            ws.rule("Only debug", OngoingGlobal, |r, _| {
                r.actions(|b| b.debug_only(|b| b.small_message("hello")));
            });
        }
    }
}

fn code(calls: bool, debug: DebugMode) -> String {
    Mode { calls }
        .compile_with_debug(Lang::EnUs, Some(debug))
        .expect("compiles")
        .code
}

fn on() -> DebugOptions {
    DebugOptions::default()
}

#[test]
fn a_release_build_equals_the_mod_without_the_calls() {
    let with = code(true, DebugMode::Off);
    let without = code(false, DebugMode::Off);
    // The rule that held nothing but a debug-only call is empty, and an empty rule goes.
    assert_eq!(with, without);
    assert!(!with.contains("Create HUD Text"), "{with}");
    assert!(!with.contains("test teleport"), "{with}");
    assert!(!with.contains("Only debug"), "{with}");
}

#[test]
fn skip_distances_do_not_count_removed_calls() {
    let release = code(true, DebugMode::Off);
    assert!(
        release.contains("Skip If(Compare(Global.flag, ==, 1), 1);"),
        "{release}"
    );
    let debug = code(true, DebugMode::On(on()));
    assert!(
        !debug.contains("Skip If(Compare(Global.flag, ==, 1), 1);"),
        "{debug}"
    );
}

#[test]
fn a_debug_build_keeps_the_calls() {
    let debug = code(true, DebugMode::On(on()));
    assert!(debug.contains("test teleport"), "{debug}");
    assert!(debug.contains("Custom String(\"starting\")"), "{debug}");
    assert!(debug.contains("Custom String(\"hello\")"), "{debug}");
    assert_eq!(debug.matches("\"nested\"").count(), 1, "{debug}");
}

#[test]
fn keep_debug_calls_off_strips_them_but_keeps_the_panels() {
    let mut options = on();
    options.keep_debug_calls = false;
    let stripped = code(true, DebugMode::On(options));
    assert!(!stripped.contains("test teleport"), "{stripped}");
    assert!(!stripped.contains("starting"), "{stripped}");
    assert!(stripped.contains("Debug: globals"), "{stripped}");
    // What follows the panels is the release mod's rules.
    let release = code(true, DebugMode::Off);
    let rules = |code: &str| code[code.find("rule (\"Run\")").expect("a rule")..].to_owned();
    assert_eq!(rules(&stripped), rules(&release));
}

#[test]
fn stripped_calls_leave_no_variable_behind() {
    struct OnlyDebug;

    impl Workshop for OnlyDebug {
        const NAME: &'static str = "Test";
        const DESCRIPTION: &'static str = "Test mode";

        fn settings(&self) -> ow_workshop::Settings {
            ow_workshop::Settings::unchecked()
        }

        type Globals = Globals;
        type Players = ();

        fn build(&self, ws: &mut Builder, g: &Globals, (): &()) {
            ws.rule("Run", OngoingGlobal, |r, _| {
                r.actions(|b| b.debug_only(|b| b.set(g.flag, 1)));
            });
        }
    }

    let release = OnlyDebug
        .compile_with_debug(Lang::EnUs, Some(DebugMode::Off))
        .expect("compiles");
    assert!(
        !release.code.contains("Set Global Variable"),
        "{}",
        release.code
    );
}
