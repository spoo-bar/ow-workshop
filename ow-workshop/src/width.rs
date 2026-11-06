//! Text width helpers for the Blizzard Global font (used by `.big()` and progress-bar text).

use ow_workshop_data::{SPACES, char_width};

/// Why a width could not be computed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum TextWidthError {
    /// The text has a newline, so it has no single width.
    Newline,
}

impl std::fmt::Display for TextWidthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Newline => f.write_str("text with a newline has no single width"),
        }
    }
}

impl std::error::Error for TextWidthError {}

/// The width of `text` in the font: the sum of its characters' widths (unlisted ones are 512).
///
/// Build-time only, for literal strings.
///
/// # Errors
/// When `text` contains a newline.
pub fn visual_len(text: &str) -> Result<u32, TextWidthError> {
    if text.contains('\n') {
        return Err(TextWidthError::Newline);
    }
    Ok(text.chars().map(char_width).sum())
}

/// The fewest space characters that add up to exactly `width` (coin change over the space widths).
///
/// Ties go to the smallest first coin, so the result is stable.
#[must_use]
pub fn spaces_for(width: u32) -> String {
    let target = width as usize;
    // `cost[w]` is the fewest characters for width `w`; `coin[w]` the first of them.
    let mut cost = vec![0_u32; target + 1];
    let mut coin = vec![0_usize; target + 1];
    for w in 1..=target {
        let mut best = u32::MAX;
        for (i, (size, _)) in SPACES.iter().enumerate() {
            let size = *size as usize;
            if size > w {
                break;
            }
            let candidate = 1 + cost[w - size];
            if candidate < best {
                best = candidate;
                coin[w] = i;
            }
        }
        cost[w] = best;
    }
    let mut out = String::new();
    let mut left = target;
    while left > 0 {
        let (size, c) = SPACES[coin[left]];
        out.push(c);
        left -= size as usize;
    }
    out
}

/// Spaces as wide as `text`.
///
/// # Errors
/// When `text` contains a newline.
pub fn spaces_for_str(text: &str) -> Result<String, TextWidthError> {
    Ok(spaces_for(visual_len(text)?))
}

#[cfg(test)]
mod tests;
