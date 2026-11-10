//! Localized strings print in the output language.

use ow_workshop::prelude::*;

struct Mode;

impl Workshop for Mode {
    type Globals = ();
    type Players = ();

    fn build(&self, ws: &mut Builder, (): &(), (): &()) {
        ws.rule("Greet", OngoingGlobal, |r, _| {
            r.actions(|b| {
                b.big_message(localized_en("Hello", [0; 0]));
                b.big_message(localized_en("{0} - {1}", [1, 2]));
            });
        });
    }
}

#[test]
fn the_phrase_is_printed_in_the_output_language() {
    let en = Mode.compile(Lang::EnUs).expect("compiles").code;
    assert!(en.contains("String(\"Hello\")"), "{en}");
    let de = Mode.compile(Lang::DeDe).expect("compiles").code;
    assert!(de.contains("(\"Hallo\")"), "{de}");
}

#[test]
fn language_neutral_phrases_become_custom_strings() {
    let en = Mode.compile(Lang::EnUs).expect("compiles").code;
    // The literal arguments fold into the text.
    assert!(en.contains("Custom String(\"1 - 2\")"), "{en}");
}
