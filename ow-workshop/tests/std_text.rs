//! The string helpers compile end to end.

use ow_workshop::prelude::*;

#[derive(GlobalVars)]
struct Globals {
    text: Global<Str>,
}

struct Mode;

impl Workshop for Mode {
    type Globals = Globals;
    type Players = ();

    fn build(&self, ws: &mut Builder, g: &Globals, (): &()) {
        ws.rule("Show", OngoingGlobal, |r, _| {
            r.actions(|b| {
                b.set(g.text, array_to_string([1, 2, 3], 3));
                b.set(g.text, time_to_string(match_time()));
                b.set(g.text, button_to_string(Button::Jump));
            });
        });
    }
}

#[test]
fn compiles_without_errors() {
    let out = Mode
        .compile_with_debug(Lang::EnUs, Some(DebugMode::Off))
        .expect("compiles");
    assert!(out.code.contains("Mapped Array("), "{}", out.code);
    assert!(
        out.code.contains("Input Binding String(Button(Jump))"),
        "{}",
        out.code
    );
}
