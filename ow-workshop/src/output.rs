//! The result of a compilation.

use crate::catalog::Catalog;
use crate::ir::Diagnostic;

/// The result of a successful compilation.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct CompileOutput {
    /// The workshop code to paste into the game.
    pub code: String,
    /// Non-fatal problems found.
    pub warnings: Vec<Diagnostic>,
    /// Number of elements the mod uses.
    pub element_count: usize,
    /// The updated translation catalog, for the caller to write out.
    pub catalog: Option<Catalog>,
}
