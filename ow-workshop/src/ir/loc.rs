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

impl Location {
    /// The location of the caller; use inside `#[track_caller]` functions.
    #[track_caller]
    pub(crate) fn caller() -> Self {
        Self::from(std::panic::Location::caller())
    }
}

impl From<&'static std::panic::Location<'static>> for Location {
    fn from(loc: &'static std::panic::Location<'static>) -> Self {
        Self {
            file: loc.file(),
            line: loc.line(),
            column: loc.column(),
        }
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.column)
    }
}
