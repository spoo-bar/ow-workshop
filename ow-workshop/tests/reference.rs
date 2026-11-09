//! Ports of the reference fixtures (`tests/fixtures/reference/`) to `Workshop` impls.
//!
//! Each test builds the equivalent mod in Rust and compares its output to the fixture's expected
//! `.txt`, ignoring whitespace-only lines and CRLF. Until a fixture is ported its test is an
//! ignored stub (#99); #100 and #101 replace the stubs, and #109 then moves the expected output
//! into `insta` snapshots and deletes the fixtures.

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
stub!(subroutines, "subroutines", "#100");
stub!(switches, "switches", "#100");
stub!(translations, "translations", "#101");
stub!(translations2, "translations2", "#101");
stub!(variables, "variables", "#100");
stub!(workshopsettings, "workshopSettings", "#101");
