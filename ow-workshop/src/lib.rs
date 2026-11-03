//! Write Overwatch Workshop mods in Rust.

mod builder;
mod expr;
mod ir;
mod options;
mod output;
mod pipeline;
pub mod prelude;
mod translations;
mod ty;
mod vars;
mod workshop;

pub use builder::Builder;
pub use expr::{Expr, null, vect};
pub use ir::{Diagnostic, Diagnostics, Location, Settings, Severity, WarningSet};
pub use options::{CompileOptions, OptLevel};
pub use output::{Catalog, CompileOutput};
pub use ow_workshop_data::Lang;
pub use translations::Translations;
pub use ty::*;
pub use vars::{GlobalVars, PlayerVars, VarRegistry};
pub use workshop::{Workshop, WorkshopExt};
