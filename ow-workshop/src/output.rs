//! The result of a compilation.

use crate::ir::Diagnostic;

/// A listing of the strings and elements of a mod; filled in by the catalog epic.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct Catalog;

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
    /// Optional catalog of the mod's contents.
    pub catalog: Option<Catalog>,
}
