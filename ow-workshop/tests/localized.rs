//! Localized strings print in the output language.

use ow_workshop::prelude::*;

struct Mode;

impl Workshop for Mode {
    const NAME: &'static str = "Test";
    const DESCRIPTION: &'static str = "Test mode";

    fn settings(&self) -> ow_workshop::Settings {
        ow_workshop::Settings::unchecked()
    }

    type Globals = ();
    type Players = ();

    fn rules<'a>(&'a self, (): &'a (), (): &'a ()) -> Vec<RuleDef<'a>> {
        vec![rule("Greet", OngoingGlobal, |r, _| {
            r.actions(|b| {
                b.big_message(localized_en("Hello", [0; 0]));
                b.big_message(localized_en("{0} - {1}", [1, 2]));
            });
        })]
    }
}

#[test]
fn the_phrase_is_printed_in_the_output_language() {
    let en = Mode
        .compile_with_debug(Lang::EnUs, Some(DebugMode::Off))
        .expect("compiles")
        .code;
    assert!(en.contains("String(\"Hello\")"), "{en}");
    let de = Mode
        .compile_with_debug(Lang::DeDe, Some(DebugMode::Off))
        .expect("compiles")
        .code;
    assert!(de.contains("(\"Hallo\")"), "{de}");
}

#[test]
fn language_neutral_phrases_become_custom_strings() {
    let en = Mode
        .compile_with_debug(Lang::EnUs, Some(DebugMode::Off))
        .expect("compiles")
        .code;
    // The literal arguments fold into the text.
    assert!(en.contains("Custom String(\"1 - 2\")"), "{en}");
}
