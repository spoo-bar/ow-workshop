//! String literal escaping.

use core::fmt::{self, Write};

use super::censor::avoid_censor;

/// Writes `text` as a double-quoted workshop string literal.
pub(crate) fn write_string(out: &mut impl Write, text: &str) -> fmt::Result {
    out.write_char('"')?;
    write_escaped(out, text)?;
    out.write_char('"')
}

/// Like [`write_string`], for rule names and settings strings, which the game's paste censor
/// checks: a soft hyphen goes into the words it rejects.
pub(crate) fn write_censored_string(out: &mut impl Write, text: &str) -> fmt::Result {
    let mut escaped = String::with_capacity(text.len());
    write_escaped(&mut escaped, text)?;
    out.write_char('"')?;
    out.write_str(&avoid_censor(&escaped))?;
    out.write_char('"')
}

/// Escapes `\`, `"`, LF and CR; nothing else.
fn write_escaped(out: &mut impl Write, text: &str) -> fmt::Result {
    for c in text.chars() {
        match c {
            '\\' => out.write_str("\\\\")?,
            '"' => out.write_str("\\\"")?,
            '\n' => out.write_str("\\n")?,
            '\r' => out.write_str("\\r")?,
            c => out.write_char(c)?,
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
