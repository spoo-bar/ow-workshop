//! Prints the final IR as workshop code.
//!
//! Everything is written through [`core::fmt::Write`], so printing allocates nothing per node
//! and works on WASM.

// Used by the passes that call the printer in the following changes.
#![allow(dead_code, unused_imports)]

mod censor;
mod constant;
mod expr;
mod keyword;
mod number;
mod rule;
mod stmt;
mod string;
mod summary;
#[cfg(test)]
mod testutil;
mod variables;

use core::fmt::{self, Write};

use ow_workshop_data::Lang;

use crate::count::Counts;
use crate::ir::Program;
use crate::options::CompileOptions;

pub(crate) use number::write_number;
pub(crate) use rule::write_rules;
pub(crate) use string::{write_censored_string, write_string};

/// What the printers need to know about the program being printed.
#[derive(Clone, Copy)]
pub(crate) struct Cx<'a> {
    /// The language to print in.
    pub(crate) lang: Lang,
    /// The program.
    pub(crate) program: &'a Program,
    /// The compile options.
    pub(crate) options: &'a CompileOptions,
}

/// Writes four spaces per level.
fn indent(out: &mut impl Write, depth: usize) -> fmt::Result {
    for _ in 0..depth {
        out.write_str("    ")?;
    }
    Ok(())
}

/// Prints the whole program: the element-count summary (with `counts`), the variables and
/// subroutines blocks, then the rules.
pub(crate) fn emit(
    program: &Program,
    lang: Lang,
    counts: Option<&Counts>,
) -> Result<String, fmt::Error> {
    let cx = Cx {
        lang,
        program,
        options: &program.options,
    };
    let mut out = String::new();
    if let Some(counts) = counts {
        summary::write_summary(&mut out, cx, counts)?;
    }
    if !program.options.exclude_variables_block {
        variables::write_variables(&mut out, cx)?;
        variables::write_subroutines(&mut out, cx)?;
    }
    write_rules(&mut out, cx, counts)?;
    Ok(out)
}
