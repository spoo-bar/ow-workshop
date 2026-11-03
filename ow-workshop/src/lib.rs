//! Write Overwatch Workshop mods in Rust.

// Lets the derives refer to `::ow_workshop` from inside this crate's own tests.
extern crate self as ow_workshop;

mod block;
mod builder;
mod expr;
mod ir;
mod ops;
mod options;
mod output;
mod pipeline;
pub mod prelude;
mod translations;
mod ty;
mod vars;
mod workshop;

pub use block::Block;
pub use builder::Builder;
pub use expr::{Expr, null, vect};
pub use ir::{Diagnostic, Diagnostics, Location, Settings, Severity, WarningSet};
pub use ops::{Apply, NumLike, Operand, Ordered, VecLike, if_then_else};
pub use options::{CompileOptions, OptLevel};
pub use output::{Catalog, CompileOutput};
pub use ow_workshop_data::Lang;
pub use ow_workshop_macros::{GlobalVars, PlayerVars};
pub use translations::Translations;
pub use ty::*;
pub use vars::{
    Global, GlobalHandle, GlobalVars, Indexed, Place, Plain, PlayerHandle, PlayerVar, PlayerVarRef,
    PlayerVars, RawPlace, VarRegistry,
};
pub use workshop::{Workshop, WorkshopExt};
