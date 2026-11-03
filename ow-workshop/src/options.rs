//! Compile options.

use crate::ir::WarningSet;

/// How hard the optimizer works.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum OptLevel {
    /// No optimization.
    Off,
    /// Optimize for fewer runtime operations.
    #[default]
    Speed,
    /// Optimize for fewer elements.
    Size,
}

/// Options controlling a compilation.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
#[allow(clippy::struct_excessive_bools)]
pub struct CompileOptions {
    /// Optimizer level.
    pub optimize: OptLevel,
    /// Leave out the `variables {}` block.
    pub exclude_variables_block: bool,
    /// Print the element count in a summary comment.
    pub debug_element_count: bool,
    /// Disable the workshop inspector in the emitted settings.
    pub disable_inspector: bool,
    /// Name of the rule that initializes global variables.
    pub global_init_rule_name: String,
    /// Name of the rule that initializes player variables.
    pub player_init_rule_name: String,
    /// Warnings to suppress everywhere.
    pub suppress: WarningSet,
    /// Emit tags around generated rules.
    pub tags: bool,
}

impl Default for CompileOptions {
    fn default() -> Self {
        Self {
            optimize: OptLevel::default(),
            exclude_variables_block: false,
            debug_element_count: false,
            disable_inspector: false,
            global_init_rule_name: "Initialize global variables".to_owned(),
            player_init_rule_name: "Initialize player variables".to_owned(),
            suppress: WarningSet::default(),
            tags: false,
        }
    }
}
