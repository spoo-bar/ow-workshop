//! The 100 symbols that act as base-100 digits.
//!
//! Letters and digits are left out (except `2`, `Q` and `q`) so the game's censor cannot
//! trigger, and `{` is left out so no placeholder can form.

/// The alphabet, in digit order.
pub(crate) fn alphabet() -> Vec<char> {
    let mut symbols: Vec<char> = (0x01..=0x1F).filter_map(char::from_u32).collect();
    symbols.extend(" !\"#$%&'()*+,-./".chars());
    symbols.push('2');
    symbols.extend(":;<=>?@".chars());
    symbols.push('Q');
    symbols.extend("[\\]^_`".chars());
    symbols.push('q');
    symbols.extend("|}~".chars());
    symbols.push('\u{7F}');
    symbols.extend((0x80..=0x9F).filter_map(char::from_u32));
    symbols.push('\u{A1}');
    symbols
}

/// The alphabet as one string.
pub(crate) fn alphabet_text() -> String {
    alphabet().into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// One symbol per pair of decimal digits.
    const SIZE: usize = 100;

    #[test]
    fn has_one_hundred_distinct_symbols() {
        let symbols = alphabet();
        assert_eq!(symbols.len(), SIZE);
        let mut sorted = symbols.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), SIZE);
    }

    #[test]
    fn avoids_what_the_game_would_mangle() {
        let symbols = alphabet();
        assert!(!symbols.contains(&'{'));
        assert!(!symbols.contains(&'0'), "0 separates the items");
        let letters: Vec<_> = symbols.iter().filter(|c| c.is_ascii_alphabetic()).collect();
        assert_eq!(letters, [&'Q', &'q']);
    }

    #[test]
    fn starts_and_ends_where_the_issue_says() {
        let symbols = alphabet();
        assert_eq!(symbols[0], '\u{1}');
        assert_eq!(symbols[31], ' ');
        assert_eq!(symbols[99], '\u{A1}');
    }
}
