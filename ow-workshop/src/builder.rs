//! The builder a mod records its rules and subroutines on.

use crate::ir::{Diagnostics, Program, Rule};

/// Records the rules and subroutines of a mod in call order.
#[derive(Debug, Default)]
pub struct Builder {
    rules: Vec<Rule>,
    diagnostics: Diagnostics,
}

// Recording methods are used as the API grows.
#[allow(dead_code)]
impl Builder {
    /// An empty builder.
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Appends a finished rule.
    pub(crate) fn push_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    /// Diagnostics found while recording.
    pub(crate) fn diagnostics_mut(&mut self) -> &mut Diagnostics {
        &mut self.diagnostics
    }

    /// Finishes recording, returning the program and the diagnostics found so far.
    pub(crate) fn finish(self, program: Program) -> (Program, Diagnostics) {
        let program = Program {
            rules: self.rules,
            ..program
        };
        (program, self.diagnostics)
    }
}
