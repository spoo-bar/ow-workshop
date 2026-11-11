//! Cased progress-bar text expands to overlaid progress-bar in-world texts.

use ow_workshop::prelude::*;

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

    type Globals = ();
    type Players = ();

    fn build(&self, ws: &mut Builder, (): &(), (): &()) {
        ws.rule("Cased", OngoingGlobal, |r, _| {
            r.actions(|b| {
                b.cased_progress_bar_text(
                    2,
                    Expr::<Str>::from("ab"),
                    vect(0, 0, 0).erase().cast::<Position>(),
                )
                .expect("valid");
            });
        });
    }
}

#[test]
fn expands_to_one_action_per_overlaid_text() {
    let code = Mode
        .compile_with_debug(Lang::EnUs, Some(DebugMode::Off))
        .expect("compiles")
        .code;
    let actions: Vec<&str> = code
        .lines()
        .filter(|l| l.contains("Create Progress Bar In-World Text("))
        .collect();
    assert_eq!(actions.len(), 2, "{code}");
    // Text k has percentage k and a white progress bar.
    assert!(actions[0].contains(", 1, Custom String("), "{}", actions[0]);
    assert!(actions[1].contains(", 2, Custom String("), "{}", actions[1]);
    assert!(actions.iter().all(|a| a.contains("Color(White)")), "{code}");
    // Each letter shows once, as its lowercase lookalike.
    assert_eq!(code.matches('\u{FF41}').count(), 1, "{code}");
    assert_eq!(code.matches('\u{FF42}').count(), 1, "{code}");
}
