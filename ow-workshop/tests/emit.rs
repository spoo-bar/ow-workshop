//! End-to-end: a mod compiles to workshop text.

mod common;

use common::assert_workshop_code_eq;
use ow_workshop::prelude::*;

#[derive(GlobalVars)]
struct Globals {
    round: Global<Num>,
}

struct Mode;

impl Workshop for Mode {
    fn mode_name(&self) -> &'static str {
        "Test"
    }

    fn description(&self) -> &'static str {
        "Test mode"
    }

    fn settings(&self) -> ow_workshop::Settings {
        ow_workshop::Settings::unchecked()
    }

    type Globals = Globals;
    type Players = ();

    fn build(&self, ws: &mut Builder, g: &Globals, (): &()) {
        ws.rule("Init", OngoingGlobal, |r, _| {
            r.actions(|b| b.set(g.round, 1));
        });
    }
}

#[test]
fn compiles_variables_and_a_rule() {
    let code = Mode
        .compile_with_debug(Lang::EnUs, Some(DebugMode::Off))
        .expect("compiles")
        .code;
    let expected = r#"
variables {
    global:
        0: round
}

rule ("Init") {
    event {
        Ongoing - Global;
    }
    actions {
        Set Global Variable(round, 1);
    }
}
"#;
    assert_workshop_code_eq(&code, expected);
}

struct Breaks;

impl Workshop for Breaks {
    fn mode_name(&self) -> &'static str {
        "Test"
    }

    fn description(&self) -> &'static str {
        "Test mode"
    }

    fn settings(&self) -> ow_workshop::Settings {
        ow_workshop::Settings::unchecked()
    }

    type Globals = Globals;
    type Players = ();

    fn build(&self, ws: &mut Builder, g: &Globals, (): &()) {
        ws.rule("Loop", OngoingGlobal, |r, _| {
            r.actions(|b| {
                b.while_(true, |b| {
                    b.add_to(g.round, 1);
                    b.if_(g.round.get().gt(3), |b| b.break_());
                });
            });
        });
    }
}

#[test]
fn a_break_in_a_loop_is_printed_and_counted() {
    let output = Breaks
        .compile_with_debug(Lang::EnUs, Some(DebugMode::Off))
        .expect("compiles");
    assert!(output.code.contains("Break;"), "{}", output.code);
}
