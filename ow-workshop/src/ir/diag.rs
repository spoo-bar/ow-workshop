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

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Warning => "warning",
            Self::Error => "error",
        })
    }
}

/// `file:line:col: warning[code]: message`, without the location when it is unknown.
impl std::fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(loc) = &self.location {
            write!(f, "{loc}: ")?;
        }
        write!(f, "{}[{}]: {}", self.severity, self.code, self.message)
    }
}

/// One diagnostic per line.
impl std::fmt::Display for Diagnostics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, diagnostic) in self.0.iter().enumerate() {
            if i > 0 {
                f.write_str("\n")?;
            }
            write!(f, "{diagnostic}")?;
        }
        Ok(())
    }
}

impl std::error::Error for Diagnostics {}
