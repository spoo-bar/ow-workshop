//! The element-count summary at the top of the output.

use core::fmt::{self, Write};

use super::Cx;
use crate::count::Counts;
use crate::rule_name::module_path;

/// Writes `/* Element count: … */`, the rules with more than one element, largest first.
pub(super) fn write_summary(out: &mut impl Write, cx: Cx<'_>, counts: &Counts) -> fmt::Result {
    writeln!(out, "/* Element count: (total {})\n", counts.total)?;
    let mut rules: Vec<(usize, &crate::ir::Rule)> = counts
        .rules
        .iter()
        .zip(&cx.program.rules)
        .filter(|(count, _)| count.total > 1)
        .map(|(count, rule)| (count.total, rule))
        .collect();
    // Stable, so equal counts keep their output order.
    rules.sort_by_key(|&(total, _)| std::cmp::Reverse(total));
    for (total, rule) in rules {
        // The name must not end the comment.
        let name = rule.name.replace("*/", "* /");
        write!(out, "{total:>5}: rule \"{name}\"")?;
        if let Some(loc) = rule.location {
            write!(out, " ({})", module_path(loc.file))?;
        }
        out.write_char('\n')?;
    }
    out.write_str("\n*/\n\n")
}
