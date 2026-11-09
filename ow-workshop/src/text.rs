//! String helpers: `time_to_string` and `button_to_string`.

use crate::expr::{Expr, null};
use crate::fmt::{Piece, template};
use crate::generated::api::{StrExt, SubstringArgs, abs, input_binding_string, str_len, vector_up};
use crate::math::floor;
use crate::ty::{Any, Array, Button, IntoExpr, Num, Str};

/// `H:MM:SS.ss` for a time in seconds.
///
/// Adding 100 and slicing from the second character zero-pads the minutes and seconds; numbers
/// convert to strings inside `String Slice`.
#[track_caller]
#[must_use]
pub fn time_to_string(t: impl IntoExpr<Num>) -> Expr<Str> {
    let t = t.into_expr();
    let digits = |value: Expr<Num>, count: u32| {
        let text: Expr<Str> = (value + 100).erase().cast();
        text.slice_with(
            1,
            SubstringArgs {
                substring_length: count.into(),
            },
        )
    };
    let hours = floor(t.clone() / 3600);
    let minutes = digits(t.clone() % 3600 / 60, 2);
    let seconds = digits(t % 60, 9999);
    template(
        &[
            Piece::Arg(0),
            Piece::Text(":"),
            Piece::Arg(1),
            Piece::Text(":"),
            Piece::Arg(2),
        ],
        vec![hours.erase(), minutes.erase(), seconds.erase()],
    )
}

/// The key bound to `button`, as `[KEY]` text instead of a texture icon; LSHIFT, LCONTROL and
/// LALT are shortened to SHIFT, CTRL and ALT.
///
/// Like `Input Binding String`, but keys are wrapped in brackets. The binding string is bound
/// once, as the element of a one-element `Mapped Array`.
#[track_caller]
#[must_use]
pub fn button_to_string(button: impl IntoExpr<Button>) -> Expr<Str> {
    let binding: Expr<Array<Str>> = input_binding_string(button).erase().cast();
    binding
        .map(|key, _| {
            let key = key.erase();
            // `First Of(Up)` is the text "(0.00, 1.00, 0.00)", which separates the parts.
            let separator: Expr<Str> = vector_up().erase().cast::<Array<Any>>().first().cast();
            let parts = template(
                &[
                    Piece::Arg(0),
                    Piece::Text("(0.00, 1.00, 0.00)["),
                    Piece::Arg(0),
                    Piece::Text(
                        "](0.00, 1.00, 0.00)[SHIFT](0.00, 1.00, 0.00)[CTRL](0.00, 1.00, 0.00)[ALT]",
                    ),
                ],
                vec![key.clone()],
            )
            .split(separator);

            // A texture icon is one character, so the seven-fold repeat has length 1 mod 7.
            let seven = [Piece::Text("\\")]
                .into_iter()
                .chain((0..7).map(|_| Piece::Arg(0)))
                .collect::<Vec<_>>();
            let is_icon = (str_len(template(&seven, vec![key.clone()])) % 7).eq(1);

            let modifier_names = template(
                &[Piece::Text("\u{EC47}0\u{EC47}0LSHIFT0LCONTROL0LALT")],
                vec![],
            );
            let delimiter: Expr<Str> = null::<Array<Any>>().first().cast();
            let modifier_index = abs(modifier_names.split(delimiter).index_of(key.cast::<Str>()));
            let pick = is_icon.and(modifier_index).erase().cast::<Num>();
            parts.get(pick)
        })
        .erase()
        .cast()
}

#[cfg(test)]
mod tests;
