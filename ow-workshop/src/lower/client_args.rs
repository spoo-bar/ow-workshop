//! Which arguments of which actions the client reevaluates.
//!
//! Translations only resolve correctly in arguments that the *client* reevaluates, so the
//! translation lowering asks [`client_side_args`] where it may put a translated string.

use ow_workshop_data::ActionId;
use ow_workshop_data::signature::Signature;

use super::functions::arg_index;
use crate::ir::{Expr, ExprKind};

/// A set of argument positions; an action has far fewer than 64 arguments.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct SmallBitSet(u64);

impl SmallBitSet {
    /// Adds the argument at `index`.
    pub(crate) fn insert(&mut self, index: usize) {
        if index < 64 {
            self.0 |= 1 << index;
        }
    }

    /// Whether the argument at `index` is in the set.
    pub(crate) fn contains(self, index: usize) -> bool {
        index < 64 && self.0 & (1 << index) != 0
    }

    /// Whether no argument is in the set.
    pub(crate) fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// The positions in the set, ascending.
    pub(crate) fn iter(self) -> impl Iterator<Item = usize> {
        (0..64).filter(move |&i| self.contains(i))
    }
}

impl FromIterator<usize> for SmallBitSet {
    fn from_iter<I: IntoIterator<Item = usize>>(iter: I) -> Self {
        let mut set = Self::default();
        for index in iter {
            set.insert(index);
        }
        set
    }
}

/// The argument names a token of a reevaluation constant makes client-side.
fn token_args(token: &str) -> &'static [&'static str] {
    match token {
        "VISIBILITY" => &["visibleTo"],
        "SORT_ORDER" => &["sortOrder"],
        "STRING" => &["text", "header", "subheader"],
        "COLOR" => &[
            "color",
            "textColor",
            "headerColor",
            "subheaderColor",
            "progressBarColor",
        ],
        "POSITION" => &["startPosition", "endPosition", "position"],
        "RADIUS" => &["radius"],
        "VALUES" => &["value", "values", "text"],
        "DIRECTION" => &["direction"],
        "TURN_RATE" => &["turnRate"],
        "MAGNITUDE" => &["magnitude"],
        "SIZE" => &["oversize"],
        "FRIENDLINESS" => &["friendlyTo"],
        _ => &[],
    }
}

/// Splits a reevaluation key such as `SORT_ORDER_STRING_AND_COLOR` into its tokens.
fn tokens(key: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut words = key.split('_').filter(|w| *w != "AND").peekable();
    while let Some(word) = words.next() {
        // `SORT_ORDER` and `TURN_RATE` are the two tokens of two words.
        let second = match word {
            "SORT" => Some("ORDER"),
            "TURN" => Some("RATE"),
            _ => None,
        };
        if second.is_some_and(|s| words.peek() == Some(&s)) {
            words.next();
            tokens.push(format!("{word}_{}", second.unwrap_or_default()));
        } else {
            tokens.push(word.to_owned());
        }
    }
    tokens
}

/// The `Always client-side` arguments: `debug`'s value and the message texts.
fn always_client_side(action: ActionId) -> &'static [&'static str] {
    match action {
        ActionId::LogToInspector | ActionId::BigMessage | ActionId::SmallMessage => &["text"],
        _ => &[],
    }
}

fn names_to_positions(sig: &Signature, names: &[&str]) -> impl Iterator<Item = usize> {
    names
        .iter()
        .filter_map(|name| arg_index(sig, name))
        .collect::<Vec<_>>()
        .into_iter()
}

/// The positions of the arguments of `action(args)` that the client reevaluates.
///
/// For actions with a reevaluation argument the answer depends on that constant; if it is not
/// a constant, nothing is assumed to be client-side.
pub(crate) fn client_side_args(action: ActionId, args: &[Expr]) -> SmallBitSet {
    let sig = action.signature();
    let mut set: SmallBitSet = names_to_positions(sig, always_client_side(action)).collect();
    let Some(reeval) = arg_index(sig, "reevaluation").and_then(|i| args.get(i)) else {
        return set;
    };
    let ExprKind::Const(constant) = reeval.kind() else {
        return set;
    };
    if !constant.enum_name.ends_with("Reeval") {
        return set;
    }
    for token in tokens(constant.key) {
        for position in names_to_positions(sig, token_args(&token)) {
            set.insert(position);
        }
    }
    set
}

#[cfg(test)]
mod tests;
