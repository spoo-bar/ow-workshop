//! Shared test helpers.

use similar::TextDiff;

/// Drops CR and whitespace-only lines so comparisons ignore CRLF and blank-line noise.
fn normalize(text: &str) -> String {
    text.replace("\r\n", "\n")
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

/// Asserts two workshop texts are equal after normalization, showing a line diff on failure.
#[track_caller]
pub fn assert_workshop_eq(actual: &str, expected: &str) {
    let (actual, expected) = (normalize(actual), normalize(expected));
    assert!(
        actual == expected,
        "workshop output differs (- expected, + actual):\n{}",
        TextDiff::from_lines(&expected, &actual)
            .unified_diff()
            .header("expected", "actual")
    );
}
