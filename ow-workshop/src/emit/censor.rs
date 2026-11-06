//! Works around words the game's paste censor rejects, without the `regex` crate.
//!
//! A soft hyphen (`U+00AD`, invisible) goes inside each word. The rewrites are the ones the
//! reference compiler applies, case-insensitively and in the same order.

/// The soft hyphen.
const SHY: char = '\u{AD}';

/// Characters removed first: they trigger the censor themselves.
const ZERO_WIDTH: [char; 5] = ['\u{200B}', '\u{200E}', '\u{200F}', '\u{FEFF}', '\u{061C}'];

/// `\b(left)(right)\b`, with each `\b` optional.
struct Word {
    start: bool,
    left: &'static str,
    right: &'static str,
    end: bool,
}

const fn word(start: bool, left: &'static str, right: &'static str, end: bool) -> Word {
    Word {
        start,
        left,
        right,
        end,
    }
}

/// In application order.
const WORDS: [Word; 28] = [
    word(false, "1", "488", false),
    word(false, "a", "ccount", false),
    word(false, "a", "dmin", false),
    word(true, "a", "ss", true),
    word(false, "b", "attlenet", false),
    word(true, "b", "liz", true),
    word(true, "b", "lizzaard", true),
    word(false, "b", "lizzard", false),
    word(true, "b", "low", true),
    word(true, "bn", "et", true),
    word(true, "b", "razil", true),
    word(true, "c", "anada", true),
    word(true, "d", "enmark", true),
    word(true, "e", "ngland", true),
    word(true, "f", "inland", true),
    word(false, "f", "uck", false),
    word(false, "g", "oddamn", false),
    word(true, "i", "reland", true),
    word(true, "n", "etherlands", true),
    word(true, "n", "orway", true),
    word(true, "p", "oland", true),
    word(true, "p", "olish", true),
    word(true, "s", "anctuary", true),
    word(true, "s", "atan", true),
    word(true, "s", "ingapore", true),
    word(false, "s", "hit", false),
    word(true, "s", "weden", true),
    word(true, "s", "witzerland", true),
];

/// Applies the censor workarounds to an already escaped string.
pub(crate) fn avoid_censor(text: &str) -> String {
    let mut chars: Vec<char> = text.chars().filter(|c| !ZERO_WIDTH.contains(c)).collect();
    for word in &WORDS {
        chars = rewrite(&chars, |at| match_word(word, at));
    }
    chars = rewrite(&chars, match_rigger);
    chars.into_iter().collect()
}

/// Whether `\w` matches: letters, digits, marks and connectors.
fn is_word(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '\u{200C}' || c == '\u{200D}' || is_mark(c)
}

/// Combining marks of the common blocks (`\p{M}`).
fn is_mark(c: char) -> bool {
    matches!(c,
        '\u{0300}'..='\u{036F}' | '\u{1AB0}'..='\u{1AFF}' | '\u{1DC0}'..='\u{1DFF}'
        | '\u{20D0}'..='\u{20FF}' | '\u{FE20}'..='\u{FE2F}' | '\u{0E31}' | '\u{0E34}'..='\u{0E3A}'
        | '\u{0E47}'..='\u{0E4E}' | '\u{3099}' | '\u{309A}')
}

/// Whether there is a word boundary between `before` and `after`.
fn boundary(before: Option<char>, after: Option<char>) -> bool {
    before.is_some_and(is_word) != after.is_some_and(is_word)
}

/// What a matcher found at one position: its length and where the soft hyphen goes.
struct Hit {
    len: usize,
    split: usize,
}

/// Replaces every non-overlapping match (leftmost first) by inserting a soft hyphen.
fn rewrite(chars: &[char], matcher: impl Fn((&[char], usize)) -> Option<Hit>) -> Vec<char> {
    let mut out = Vec::with_capacity(chars.len() + 2);
    let mut i = 0;
    while i < chars.len() {
        if let Some(hit) = matcher((chars, i)) {
            out.extend(&chars[i..i + hit.split]);
            out.push(SHY);
            out.extend(&chars[i + hit.split..i + hit.len]);
            i += hit.len.max(1);
        } else {
            out.push(chars[i]);
            i += 1;
        }
    }
    out
}

/// Matches `lit` case-insensitively at `at`; returns the position after it.
fn lit(chars: &[char], at: usize, lit: &str) -> Option<usize> {
    let mut pos = at;
    for expected in lit.chars() {
        if !chars.get(pos)?.eq_ignore_ascii_case(&expected) {
            return None;
        }
        pos += 1;
    }
    Some(pos)
}

fn match_word(word: &Word, (chars, at): (&[char], usize)) -> Option<Hit> {
    if word.start && !boundary(at.checked_sub(1).map(|i| chars[i]), chars.get(at).copied()) {
        return None;
    }
    let mid = lit(chars, at, word.left)?;
    let end = lit(chars, mid, word.right)?;
    if word.end && !boundary(Some(chars[end - 1]), chars.get(end).copied()) {
        return None;
    }
    Some(Hit {
        len: end - at,
        split: mid - at,
    })
}

/// `(r\s*i\s*)(gg\s*e\s*r)\b`
fn match_rigger((chars, at): (&[char], usize)) -> Option<Hit> {
    let spaces = |mut pos: usize| {
        while chars.get(pos).is_some_and(|c| c.is_whitespace()) {
            pos += 1;
        }
        pos
    };
    let pos = spaces(lit(chars, at, "r")?);
    let pos = lit(chars, pos, "i")?;
    let split = spaces(pos);
    let pos = spaces(lit(chars, split, "gg")?);
    let pos = spaces(lit(chars, pos, "e")?);
    let end = lit(chars, pos, "r")?;
    if !boundary(Some(chars[end - 1]), chars.get(end).copied()) {
        return None;
    }
    Some(Hit {
        len: end - at,
        split: split - at,
    })
}

#[cfg(test)]
mod tests;
