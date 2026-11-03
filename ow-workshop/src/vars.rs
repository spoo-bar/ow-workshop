//! Variable declaration traits.

use crate::ir::Vars;

/// Collects variable declarations while the handles are created.
#[derive(Debug, Default)]
pub struct VarRegistry {
    pub(crate) vars: Vars,
}

/// A struct of global variable handles.
pub trait GlobalVars: Sized {
    /// Registers the variables and returns their handles.
    fn declare(reg: &mut VarRegistry) -> Self;
}

/// A struct of player variable handles.
pub trait PlayerVars: Sized {
    /// Registers the variables and returns their handles.
    fn declare(reg: &mut VarRegistry) -> Self;
}

impl GlobalVars for () {
    fn declare(_: &mut VarRegistry) -> Self {}
}

impl PlayerVars for () {
    fn declare(_: &mut VarRegistry) -> Self {}
}
