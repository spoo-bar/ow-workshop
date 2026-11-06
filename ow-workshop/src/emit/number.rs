//! Number formatting.

use core::fmt::{self, Write};

/// Longest fraction the workshop editor accepts.
const MAX_FRACTION: usize = 15;
/// What infinity prints as.
const INFINITY: &str = "999999999999";

/// Writes `n` like JavaScript's `Number.prototype.toString()`, with the fraction cut to
/// [`MAX_FRACTION`] digits (truncated, not rounded) because the editor rejects longer ones.
pub(crate) fn write_number(out: &mut impl Write, n: f64) -> fmt::Result {
    if n.is_nan() || n == 0.0 {
        // Also covers negative zero.
        return out.write_char('0');
    }
    if n.is_infinite() {
        if n < 0.0 {
            out.write_char('-')?;
        }
        return out.write_str(INFINITY);
    }
    let mut buffer = ryu_js::Buffer::new();
    let text = buffer.format_finite(n);
    let (mantissa, exponent) = match text.split_once('e') {
        Some((mantissa, exponent)) => (mantissa, Some(exponent)),
        None => (text, None),
    };
    match mantissa.split_once('.') {
        Some((whole, fraction)) => {
            let fraction = fraction.get(..MAX_FRACTION).unwrap_or(fraction);
            write!(out, "{whole}.{fraction}")?;
        }
        None => out.write_str(mantissa)?,
    }
    if let Some(exponent) = exponent {
        write!(out, "e{exponent}")?;
    }
    Ok(())
}

#[cfg(test)]
mod tests;
