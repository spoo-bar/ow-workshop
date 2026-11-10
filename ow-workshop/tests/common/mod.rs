//! Shared test helpers.

use similar::TextDiff;

/// Asserts two workshop code texts are equal, ignoring CRLF and blank or whitespace-only lines.
#[track_caller]
#[allow(dead_code)]
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

/// Strips the metadata header of an `insta` snapshot file.
#[cfg(target_arch = "wasm32")]
#[allow(dead_code)]
pub fn snapshot_body(snap: &str) -> &str {
    let rest = snap.strip_prefix("---\n").expect("a snapshot header");
    &rest[rest.find("---\n").expect("the end of the header") + 4..]
}

/// Asserts a mod's code equals `tests/snapshots/<file>__<name>.snap` (`file` is the test file's stem).
///
/// Natively this is an `insta` snapshot (`cargo insta review` accepts changes). `insta` reads
/// and writes files, which WASM does not have, so there the snapshot file is compiled in and
/// compared, ignoring whitespace-only lines and CRLF as [`assert_workshop_code_eq`] does.
#[allow(unused_macros)]
macro_rules! assert_code_snapshot {
    ($file:literal, $name:literal, $code:expr) => {{
        #[cfg(not(target_arch = "wasm32"))]
        insta::assert_snapshot!($name, $code);
        #[cfg(target_arch = "wasm32")]
        $crate::common::assert_workshop_code_eq(
            $code,
            $crate::common::snapshot_body(include_str!(concat!(
                "snapshots/",
                $file,
                "__",
                $name,
                ".snap"
            ))),
        );
    }};
}
#[allow(unused_imports)]
pub(crate) use assert_code_snapshot;
