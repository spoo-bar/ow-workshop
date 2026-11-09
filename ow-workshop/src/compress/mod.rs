//! Array compression: large literal arrays packed into one string and decoded at run time.
//!
//! An array of numbers or vectors costs one element per value; as a single string with a short
//! decoding formula it costs far fewer.

mod alphabet;
mod encode;

use crate::expr::Expr;
use crate::ir::{self, Decompress, ExprKind, StrExpr, StrToken};
use crate::ty::{Array, IntoExpr, Num, Str, ValueType, Vector};

pub(crate) use alphabet::alphabet_text;
use encode::{Values, pack};

/// Why an array could not be compressed.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum CompressError {
    /// The array has no items.
    Empty,
    /// The array is not a literal array of numbers (or vectors of numbers); `Null` counts as 0.
    NotLiteral,
    /// A value is too large: numbers must be below 49999 and vector components below 4999.
    OutOfRange {
        /// The offending value.
        value: f64,
        /// The limit it must stay under, in magnitude.
        limit: f64,
    },
}

impl std::fmt::Display for CompressError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str("cannot compress an empty array"),
            Self::NotLiteral => {
                f.write_str("only literal arrays of numbers or vectors can be compressed")
            }
            Self::OutOfRange { value, limit } => {
                write!(
                    f,
                    "{value} is out of range: values must be below {limit} in magnitude"
                )
            }
        }
    }
}

impl std::error::Error for CompressError {}

/// Items that can be compressed: numbers and vectors.
pub trait Compressible: ValueType {
    #[doc(hidden)]
    const VECTORS: bool;
}

impl Compressible for Num {
    const VECTORS: bool = false;
}

impl Compressible for Vector {
    const VECTORS: bool = true;
}

/// The array, decoded at run time from a string with just enough digits for its values.
///
/// Numbers keep up to three decimals and vector components two; the decoded values are within
/// that precision of the originals. The array must be a literal of numbers (or of vectors of
/// number literals); `Null` stands for 0. Name the item type: `compressed::<Num>([1.5, 2.0])`.
#[track_caller]
pub fn compressed<T: Compressible>(
    array: impl IntoExpr<Array<T>>,
) -> Result<Expr<Array<T>>, CompressError> {
    let values = literal_values::<T>(&array.into_expr())?;
    let packed = pack(&values, false)?;
    Ok(decode::<T>(
        &packed.text,
        T::VECTORS,
        packed.min_place,
        packed.max_place,
        packed.offset,
    ))
}

/// The array as a string, for [`decompress_numbers`] or [`decompress_vectors`].
///
/// Uses fixed places, so the text is longer than [`compressed`] needs but the decoder is the
/// same everywhere: numbers must be below 49999 and vector components below 4999.
#[track_caller]
pub fn compress<T: Compressible>(
    array: impl IntoExpr<Array<T>>,
) -> Result<Expr<Str>, CompressError> {
    let values = literal_values::<T>(&array.into_expr())?;
    Ok(text(&pack(&values, true)?.text))
}

/// Decodes a string made by [`compress`] from numbers.
#[track_caller]
#[must_use]
pub fn decompress_numbers(packed: impl IntoExpr<Str>) -> Expr<Array<Num>> {
    fixed(packed.into_expr(), false, -3, 5, 50_000.0)
}

/// Decodes a string made by [`compress`] from vectors.
#[track_caller]
#[must_use]
pub fn decompress_vectors(packed: impl IntoExpr<Str>) -> Expr<Array<Vector>> {
    fixed(packed.into_expr(), true, -2, 4, 5_000.0)
}

#[track_caller]
fn fixed<T>(
    packed: Expr<Str>,
    vectors: bool,
    min_place: i32,
    max_place: i32,
    offset: f64,
) -> Expr<Array<T>> {
    Expr::from_ir(ir::Expr::here(ExprKind::Decompress(Box::new(Decompress {
        packed: packed.into_ir(),
        vectors,
        min_place,
        max_place,
        offset,
    }))))
}

#[track_caller]
fn decode<T>(
    packed: &str,
    vectors: bool,
    min_place: i32,
    max_place: i32,
    offset: f64,
) -> Expr<Array<T>> {
    fixed(text(packed), vectors, min_place, max_place, offset)
}

/// A string literal that keeps every character: `Expr<Str>::from` would drop carriage returns,
/// which are digits of the alphabet.
#[track_caller]
fn text(text: &str) -> Expr<Str> {
    Expr::from_ir(ir::Expr::here(ExprKind::Str(StrExpr::Custom {
        tokens: vec![StrToken::Text(text.to_owned())],
        args: Vec::new(),
    })))
}

fn literal_values<T: Compressible>(array: &Expr<Array<T>>) -> Result<Values, CompressError> {
    let ExprKind::Array(items) = array.ir().kind() else {
        return Err(CompressError::NotLiteral);
    };
    if T::VECTORS {
        items
            .iter()
            .map(vector)
            .collect::<Result<_, _>>()
            .map(Values::Vectors)
    } else {
        items
            .iter()
            .map(number)
            .collect::<Result<_, _>>()
            .map(Values::Numbers)
    }
}

fn number(item: &ir::Expr) -> Result<f64, CompressError> {
    match item.kind() {
        ExprKind::Num(n) => Ok(*n),
        ExprKind::Neg(inner) => number(inner).map(|n| -n),
        ExprKind::Null => Ok(0.0),
        _ => Err(CompressError::NotLiteral),
    }
}

fn vector(item: &ir::Expr) -> Result<[f64; 3], CompressError> {
    match item.kind() {
        ExprKind::Vector([x, y, z]) => Ok([number(x)?, number(y)?, number(z)?]),
        ExprKind::Null => Ok([0.0; 3]),
        _ => Err(CompressError::NotLiteral),
    }
}

#[cfg(test)]
mod tests;
