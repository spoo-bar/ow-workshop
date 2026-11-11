//! Compressed arrays compile end to end, with and without the alphabet variable.

use ow_workshop::prelude::*;

#[derive(GlobalVars)]
struct Globals {
    points: Global<Array<Vector>>,
}

struct Mode {
    alphabet_variable: bool,
}

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
            r.actions(|b| {
                let points = compressed::<Vector>(vec![
                    vect(1, 2, 3),
                    vect(-4.5, 0, 250.25),
                    vect(10, 20, 30),
                ])
                .expect("compresses");
                b.set(g.points, points);
            });
        });
    }

    fn options(&self) -> CompileOptions {
        let mut options = CompileOptions::default();
        options.compression_alphabet_variable = self.alphabet_variable;
        options
    }
}

fn compile(alphabet_variable: bool) -> String {
    Mode { alphabet_variable }
        .compile_with_debug(Lang::EnUs, Some(DebugMode::Off))
        .expect("compiles")
        .code
}

#[test]
fn the_alphabet_is_repeated_without_the_variable() {
    let code = compile(false);
    assert!(!code.contains("__compressionAlphabet__"), "{code}");
    assert!(
        code.contains("Append To Array(Current Array Element"),
        "{code}"
    );
    assert!(code.contains("Last Of(Current Array Element)"), "{code}");
    assert!(!code.contains("Decompress"), "{code}");
}

#[test]
fn the_variable_holds_the_alphabet_when_asked() {
    let code = compile(true);
    assert!(code.contains("__compressionAlphabet__"), "{code}");
    assert!(!code.contains("Last Of(Current Array Element)"), "{code}");
    // Set by the global init rule, and read by the decoder.
    assert!(
        code.contains("Set Global Variable(__compressionAlphabet__, Custom String(\""),
        "{code}"
    );
    assert!(
        code.contains("Index Of String Char(Global.__compressionAlphabet__"),
        "{code}"
    );
}

#[test]
fn the_variable_is_internal_so_it_takes_the_highest_index() {
    let code = compile(true);
    assert!(code.contains("127: __compressionAlphabet__"), "{code}");
}

#[test]
fn the_alphabet_keeps_its_carriage_return_digit() {
    // A carriage return is one of the 100 digits; string normalization must not drop it.
    let code = Mode {
        alphabet_variable: false,
    }
    .compile_with_debug(Lang::EnUs, Some(DebugMode::Off))
    .expect("compiles")
    .code;
    // The text prints a carriage return as `\r`, between the form feed and shift-out digits.
    assert!(
        code.contains("\u{c}\\r\u{e}"),
        "the alphabet lost its carriage return"
    );
}
