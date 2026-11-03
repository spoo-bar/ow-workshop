//! Minimal diagnostics for IR passes; superseded by the diagnostics epic.

use super::Location;

/// How serious a diagnostic is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Severity {
    /// Compilation can continue.
    Warning,
    /// Compilation fails.
    Error,
}

/// One problem found in the user's mod.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Diagnostic {
    /// Warning or error.
    pub(crate) severity: Severity,
    /// Stable kebab-case identifier, e.g. `duplicate-name`.
    pub(crate) code: &'static str,
    /// Human-readable description.
    pub(crate) message: String,
    /// Where in the user's source, if known.
    pub(crate) location: Option<Location>,
}

/// A list of diagnostics.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Diagnostics(pub(crate) Vec<Diagnostic>);

impl Diagnostics {
    /// Records a diagnostic.
    pub(crate) fn push(
        &mut self,
        severity: Severity,
        code: &'static str,
        message: String,
        location: Option<Location>,
    ) {
        self.0.push(Diagnostic {
            severity,
            code,
            message,
            location,
        });
    }

    /// Whether any diagnostic is an error.
    pub(crate) fn has_errors(&self) -> bool {
        self.0.iter().any(|d| d.severity == Severity::Error)
    }
}
