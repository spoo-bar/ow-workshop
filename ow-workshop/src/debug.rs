//! `debug!`: shows an expression's source and its value on the HUD.

use ow_workshop_data::char_width;

use crate::array_to_string::array_to_string;
use crate::block::Block;
use crate::expr::Expr;
use crate::hud::HudArgs;
use crate::ir::{self, ExprKind, StrExpr, StrToken};
use crate::ty::{Any, Array, Str, ValueType};
use crate::vars::{Global, Indexed, Indexed2, Indexed3, PlayerVarRef};

/// Widest source text shown, in font units.
const MAX_SOURCE_WIDTH: u32 = 20_000;

/// Items of an array shown.
const ARRAY_ITEMS: u32 = 6;

/// Keeps the box wide enough that the text does not move when the value changes.
const PADDING: usize = 170;

/// A value `debug!` can show: an expression or a variable.
pub trait DebugValue {
    #[doc(hidden)]
    fn into_debug(self) -> Expr<Any>;
}

/// Arrays show only their first element, so they are spelled out.
fn shown<T: ValueType>(value: Expr<T>) -> Expr<Any> {
    if T::IS_ARRAY {
        array_to_string(value.erase().cast::<Array<Any>>(), ARRAY_ITEMS).erase()
    } else {
        value.erase()
    }
}

impl<T: ValueType> DebugValue for Expr<T> {
    fn into_debug(self) -> Expr<Any> {
        shown(self)
    }
}

impl<T: ValueType> DebugValue for &Expr<T> {
    fn into_debug(self) -> Expr<Any> {
        shown(self.clone())
    }
}

macro_rules! variables {
    ($($name:ident),*) => {$(
        impl<T: ValueType> DebugValue for $name<T> {
            fn into_debug(self) -> Expr<Any> {
                shown(self.get())
            }
        }
    )*};
}

variables!(Global, PlayerVarRef, Indexed, Indexed2, Indexed3);

/// The source text as shown: braces would read as placeholders, so they become the tag
/// characters that look like them; it is cut to a width the HUD can hold.
fn source_text(source: &str, tags: bool) -> String {
    let source: String = source
        .chars()
        .filter(|c| *c != '\r')
        .map(|c| match c {
            '{' => '\u{E007B}',
            '}' => '\u{E007D}',
            '\n' => ' ',
            other => other,
        })
        .collect();
    let ending = if tags { "</FG>…" } else { "…" };
    let width = |text: &str| text.chars().map(char_width).sum::<u32>();
    if width(&source) <= MAX_SOURCE_WIDTH {
        return source;
    }
    let mut room = MAX_SOURCE_WIDTH.saturating_sub(width(ending));
    let mut cut: String = source
        .chars()
        .take_while(|c| {
            room = room.saturating_sub(char_width(*c));
            room > 0
        })
        .collect();
    cut.push_str(ending);
    cut
}

/// `source`, a line break, `= ` and a placeholder for the value, padded to a fixed width.
fn template(source: &str, tags: bool) -> Vec<StrToken> {
    let padding = " ".repeat(PADDING);
    let (head, equals) = if tags {
        (
            format!("<FGD0FF00FF>{}", source_text(source, true)),
            "\u{2028}<FG99F3FFFF>= <FGFFFF00FF>",
        )
    } else {
        (source_text(source, false), "\u{2028}= ")
    };
    vec![
        StrToken::Text(format!("{head}{equals}")),
        StrToken::Arg(0),
        StrToken::Text(padding),
    ]
}

impl Block {
    /// Shows `source` and the current value of `value` on everyone's HUD; use through
    /// [`debug!`](crate::debug).
    ///
    /// Arrays are spelled out with `array_to_string`. With text tags on, the source and the
    /// value are colored.
    ///
    /// A debug-only call: a release build leaves it out (see [`Block::debug_only`]).
    #[track_caller]
    pub fn debug_value(&mut self, source: &str, value: impl DebugValue) {
        let start = self.recorded();
        let text: Expr<Str> = Expr::from_ir(ir::Expr::here(ExprKind::Str(StrExpr::Custom {
            tokens: template(source, self.tags),
            args: vec![value.into_debug().into_ir()],
        })));
        let opts = HudArgs {
            sort_order: (-9999).into(),
            ..HudArgs::default()
        };
        self.hud_subheader_with(text.case_sensitive(), opts);
        self.mark_debug_only(start);
    }
}

/// Shows an expression's source and its value on the HUD: `debug!(b, a.get() + 2)` shows
/// `a.get() + 2` and what it is now.
#[macro_export]
macro_rules! debug {
    ($block:expr, $value:expr $(,)?) => {
        $block.debug_value(::core::stringify!($value), $value)
    };
}

// Not in a `debug/` directory: the repository's .gitignore ignores that name.
#[cfg(test)]
#[path = "debug_tests.rs"]
mod tests;
