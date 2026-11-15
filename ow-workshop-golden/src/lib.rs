//! Compares a generated release script to a reference script.
//!
//! Reference scripts are normalized first (see [`normalize`]), so the
//! comparison is "same script", not "same bytes".

mod diff;
mod normalize;
mod reference;

pub use normalize::{Options, normalize, normalize_with};
pub use reference::{REFERENCE_DIR_VAR, reference_file};

/// Panics with a line diff unless `generated` is the same script as `reference`.
#[track_caller]
pub fn assert_same_script(generated: &str, reference: &str) {
    assert_same_script_with(generated, reference, &Options::default());
}

/// Like [`assert_same_script`], with explicit normalization options.
#[track_caller]
pub fn assert_same_script_with(generated: &str, reference: &str, options: &Options) {
    let generated = normalize_with(generated, options);
    let reference = normalize_with(reference, options);
    if generated != reference {
        let report = diff::lines(&reference, &generated);
        panic!("generated script differs from the reference (- reference, + generated):\n{report}");
    }
}
