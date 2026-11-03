//! Shared test helpers.

use similar::TextDiff;

/// Asserts two workshop code texts are equal, ignoring CRLF and blank or whitespace-only lines.
#[track_caller]
pub fn assert_workshop_code_eq(actual: &str, expected: &str) {
    let normalize = |text: &str| {
        text.replace("\r\n", "\n")
            .lines()
            .filter(|line| !line.trim().is_empty())
            .collect::<Vec<_>>()
            .join("\n")
    };
    let (actual, expected) = (normalize(actual), normalize(expected));
    assert!(
        actual == expected,
        "workshop output differs (- expected, + actual):\n{}",
        TextDiff::from_lines(&expected, &actual)
            .unified_diff()
            .header("expected", "actual")
    );
}
