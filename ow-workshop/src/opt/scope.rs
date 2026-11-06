//! Optimizer settings in effect for a block.

use crate::ir::Block;
use crate::options::{CompileOptions, OptLevel};

/// The four optimizer switches, resolved for the block being walked.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(clippy::struct_excessive_bools)]
pub(crate) struct Scope {
    enabled: bool,
    size: bool,
    aggressive_size: bool,
    strict: bool,
}

impl Scope {
    /// The program-wide defaults.
    pub(crate) fn new(options: &CompileOptions) -> Self {
        Self {
            enabled: options.optimize != OptLevel::Off,
            size: options.optimize == OptLevel::Size,
            aggressive_size: options.aggressive_size,
            strict: options.strict,
        }
    }

    /// The settings inside `block`: its override replaces the parent's.
    ///
    /// `aggressive_size` is program-wide and never overridden.
    #[must_use]
    pub(crate) fn enter(self, block: &Block) -> Self {
        let Some(over) = block.opt else { return self };
        Self {
            enabled: over.enabled,
            size: over.enabled && over.size,
            strict: over.strict.unwrap_or(self.strict),
            ..self
        }
    }

    /// Whether any rewrite runs.
    pub(crate) fn enabled(self) -> bool {
        self.enabled
    }

    /// Whether to prefer fewer elements over runtime speed.
    pub(crate) fn size(self) -> bool {
        self.enabled && self.size
    }

    /// Whether the whole-program size rewrites run; needs `size`.
    pub(crate) fn aggressive_size(self) -> bool {
        self.size() && self.aggressive_size
    }

    /// Whether rewrites that change behaviour under workshop type coercion are off.
    pub(crate) fn strict(self) -> bool {
        self.strict
    }
}
