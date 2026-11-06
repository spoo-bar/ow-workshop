//! The `variables` and `subroutines` blocks.

use core::fmt::{self, Write};

use ow_workshop_data::Keyword;

use super::{Cx, indent};
use crate::ir::{Program, Vars};

/// Writes the `variables` block; nothing if no variable is declared.
pub(super) fn write_variables(out: &mut impl Write, cx: Cx<'_>) -> fmt::Result {
    let globals = indexed(
        cx.program
            .globals
            .entries
            .iter()
            .map(|d| (d.index, &d.name)),
    )?;
    let players = indexed(
        cx.program
            .players
            .entries
            .iter()
            .map(|d| (d.index, &d.name)),
    )?;
    if globals.is_empty() && players.is_empty() {
        return Ok(());
    }
    writeln!(out, "{} {{", Keyword::Variables.name(cx.lang))?;
    for (keyword, entries) in [(Keyword::Global, globals), (Keyword::Player, players)] {
        if entries.is_empty() {
            continue;
        }
        indent(out, 1)?;
        writeln!(out, "{}:", keyword.name(cx.lang))?;
        write_entries(out, 2, &entries)?;
    }
    out.write_str("}\n\n")
}

/// Writes the `subroutines` block; nothing if no subroutine is declared.
pub(super) fn write_subroutines(out: &mut impl Write, cx: Cx<'_>) -> fmt::Result {
    let entries = indexed(
        cx.program
            .subroutines
            .entries
            .iter()
            .map(|d| (d.index, &d.name)),
    )?;
    if entries.is_empty() {
        return Ok(());
    }
    writeln!(out, "{} {{", Keyword::Subroutines.name(cx.lang))?;
    write_entries(out, 1, &entries)?;
    out.write_str("}\n\n")
}

/// The entries sorted by index. Indices must have been allocated.
fn indexed<'a>(
    entries: impl Iterator<Item = (Option<u8>, &'a String)>,
) -> Result<Vec<(u8, &'a str)>, fmt::Error> {
    let mut sorted = entries
        .map(|(index, name)| index.map(|i| (i, name.as_str())).ok_or(fmt::Error))
        .collect::<Result<Vec<_>, _>>()?;
    sorted.sort_unstable_by_key(|&(index, _)| index);
    Ok(sorted)
}

fn write_entries(out: &mut impl Write, depth: usize, entries: &[(u8, &str)]) -> fmt::Result {
    for (index, name) in entries {
        indent(out, depth)?;
        writeln!(out, "{index}: {name}")?;
    }
    Ok(())
}

#[cfg(test)]
mod tests;
