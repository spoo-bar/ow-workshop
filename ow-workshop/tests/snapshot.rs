//! Sanity checks for the snapshot-testing helper.

mod common;

use common::assert_workshop_eq;
use wasm_bindgen_test::wasm_bindgen_test;

// `include_str!`: there is no filesystem at test runtime on WASM.
const HELLO: &str = include_str!("fixtures/hello.txt");

#[test]
#[wasm_bindgen_test]
fn ignores_crlf_and_blank_lines() {
    let actual = "rule (\"hello\") {\r\n\r\n    event {\r\n   \r\n        Ongoing - Global;\r\n    }\r\n}\r\n";
    assert_workshop_eq(actual, HELLO);
}

#[test]
#[wasm_bindgen_test]
#[should_panic(expected = "workshop output differs")]
fn reports_differences() {
    assert_workshop_eq("rule (\"other\") {}", HELLO);
}
