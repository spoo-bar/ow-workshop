//! `array_to_string`: shows `[a, b, c, …+N]`, where the game shows only an array's first element.

use crate::expr::{Expr, null};
use crate::fmt::{Piece, template};
use crate::generated::api::{StrExt, SubstringArgs};
use crate::ops::if_then_else;
use crate::ty::{Any, Array, Bool, IntoExpr, Num, Str};

/// Most items shown.
const MAX_SHOWN: u32 = 1000;

/// Marks the end of the text we may cut, so only our own `…` is ever replaced.
const END: &str = "…\u{1}";

fn empty() -> Expr<Array<Any>> {
    Vec::<Expr<Any>>::new().into()
}

fn as_array(value: &Expr<Any>) -> Expr<Array<Any>> {
    value.clone().cast()
}

fn as_bool(value: Expr<Num>) -> Expr<Bool> {
    value.erase().cast()
}

/// How one item of the array is displayed: nested arrays show their first element and a count.
fn item(e: &Expr<Any>) -> Expr<Any> {
    let count = as_array(e).len();
    let single = count.eq(1) | (e.eq(empty()) & e.ne(null::<Any>()));
    let wrapped: Expr<Str> = template(
        &[Piece::Text("["), Piece::Arg(0), Piece::Text("]")],
        vec![e.clone()],
    );
    let more: Expr<Str> = template(
        &[
            Piece::Text("["),
            Piece::Arg(0),
            Piece::Text(", …+"),
            Piece::Arg(1),
            Piece::Text("]"),
        ],
        vec![e.clone(), (as_array(e).len() - 1).erase()],
    );
    if_then_else(
        single,
        wrapped.erase(),
        if_then_else(as_bool(count), more.erase(), e.clone()),
    )
}

/// `[is_array, 3 * length, items…]` of the mapped array `e`.
fn describe(e: &Expr<Any>) -> Expr<Any> {
    let count = as_array(e).len();
    let size = if_then_else(
        !as_bool(count.clone()) & e.ne(empty()),
        Expr::<Num>::from(3),
        count * 3,
    );
    as_bool_array(is_array_flag(e))
        .with(size)
        .concat(as_array(e))
        .erase()
}

fn is_array_flag(e: &Expr<Any>) -> Expr<Bool> {
    as_bool(as_array(e).len()) | (e.eq(empty()) & e.ne(null::<Any>()))
}

fn as_bool_array(flag: Expr<Bool>) -> Expr<Array<Any>> {
    flag.erase().cast()
}

/// Shows an array as `[a, b, c, …+N]`, at most `max_len` items (1 to 1000).
///
/// `max_len` is a literal because it sets how many placeholders the text has.
#[track_caller]
#[must_use]
pub fn array_to_string(arr: impl IntoExpr<Array<Any>>, max_len: u32) -> Expr<Str> {
    let shown = max_len.clamp(1, MAX_SHOWN);
    let items = arr.into_expr().map(|e, _| item(&e));
    let described = Expr::<Array<Any>>::from(vec![items.erase()]).map(|e, _| describe(&e));
    described.map(|x, _| render(&x, shown)).erase().cast()
}

/// The text of one `[is_array, 3 * length, items…]` description.
#[track_caller]
fn render(x: &Expr<Any>, shown: u32) -> Expr<Any> {
    let parts = as_array(x);
    let size: Expr<Num> = parts.get(1).cast();
    let max_chars = shown * 3;

    // "{}, " once per shown item, formatted with the items; absent items read as 0.
    let mut pieces = Vec::new();
    let mut args = Vec::new();
    for slot in 0..shown {
        pieces.push(Piece::Arg(slot as usize));
        pieces.push(Piece::Text(", "));
        args.push(parts.get(slot + 2));
    }
    pieces.push(Piece::Text(END));
    let display = template(&pieces, args);

    // What the display shows when it holds fewer items than slots; cut it off.
    let placeholder: Expr<Str> = format!("{}{END}", "0, ".repeat(shown as usize)).into();
    let unused = placeholder.slice_with(
        Expr::<Num>::from(-2) + &size,
        SubstringArgs {
            substring_length: Expr::<Num>::from(max_chars + 4) - &size,
        },
    );
    let shown_items = display.replace(unused, empty().erase().cast::<Str>());

    let overflow = if_then_else(
        size.gt(max_chars),
        template(
            &[Piece::Text("+"), Piece::Arg(0)],
            vec![(size.clone() / 3 - shown).erase()],
        )
        .erase(),
        empty().erase(),
    );
    let whole = template(
        &[
            Piece::Text("["),
            Piece::Arg(0),
            Piece::Arg(1),
            Piece::Text("]"),
        ],
        vec![shown_items.erase(), overflow],
    );
    let scalar = parts
        .get(2)
        .cast::<Str>()
        .split(empty().erase().cast::<Str>());
    if_then_else(parts.first().cast::<Bool>(), whole.erase(), scalar.erase())
}

#[cfg(test)]
mod tests;
