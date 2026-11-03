//! Source locations attached to IR nodes for diagnostics.

/// A position in the user's Rust source.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Location {
    /// Source file path.
    pub(crate) file: &'static str,
    /// 1-based line.
    pub(crate) line: u32,
    /// 1-based column.
    pub(crate) column: u32,
}
