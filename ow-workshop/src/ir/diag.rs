//! Minimal diagnostics for IR passes; superseded by the diagnostics epic.

use super::Location;

/// How serious a diagnostic is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Severity {
    /// Compilation can continue.
    Warning,
    /// Compilation fails.
    Error,
}

/// One problem found in the user's mod.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diagnostic {
    /// Warning or error.
    pub severity: Severity,
    /// Stable kebab-case identifier, e.g. `duplicate-name`.
    pub code: &'static str,
    /// Human-readable description.
    pub message: String,
    /// Where in the user's source, if known.
    pub location: Option<Location>,
}

/// A list of diagnostics.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Diagnostics(pub Vec<Diagnostic>);

impl Diagnostics {
    /// Records a diagnostic.
    pub fn push(
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
    pub fn has_errors(&self) -> bool {
        self.0.iter().any(|d| d.severity == Severity::Error)
    }
}
