//! Prints the final IR as workshop code.
//!
//! Everything is written through [`core::fmt::Write`], so printing allocates nothing per node
//! and works on WASM.

// Used by the printers added in the following changes.
#![allow(dead_code, unused_imports)]

mod censor;
mod number;
mod string;

pub(crate) use number::write_number;
pub(crate) use string::{write_censored_string, write_string};
