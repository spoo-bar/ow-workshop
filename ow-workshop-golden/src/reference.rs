//! Locates reference scripts that cannot be vendored in this repository.

use std::path::PathBuf;

/// Environment variable naming a checkout of the Marblr reference scripts.
pub const REFERENCE_DIR_VAR: &str = "OW_REFERENCE_DIR";

/// Reads `relative` (e.g. `Fall Damage/FallDamage.txt`) from the directory in
/// [`REFERENCE_DIR_VAR`]. Returns `None`, after saying so on stderr, when the
/// variable is unset, so the calling test can skip.
pub fn reference_file(relative: &str) -> Option<String> {
    let Some(dir) = std::env::var_os(REFERENCE_DIR_VAR) else {
        eprintln!("skipped: {REFERENCE_DIR_VAR} is not set");
        return None;
    };
    let path = PathBuf::from(dir).join(relative);
    match std::fs::read_to_string(&path) {
        Ok(text) => Some(text),
        Err(err) => panic!("cannot read reference {}: {err}", path.display()),
    }
}
