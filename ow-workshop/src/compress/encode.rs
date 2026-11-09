//! Turns literal numbers or vectors into packed text.
//!
//! Each value gets a fixed number of decimals and a non-negative offset, then its digits are
//! written least significant first and paired into base-100 symbols of the alphabet.

use super::CompressError;
use super::alphabet::alphabet;

/// The literals to pack.
pub(crate) enum Values {
    /// Numbers.
    Numbers(Vec<f64>),
    /// Vectors, as `[x, y, z]`.
    Vectors(Vec<[f64; 3]>),
}

/// Packed text and what is needed to read it back.
#[derive(Debug, PartialEq)]
pub(crate) struct Packed {
    /// Items separated by `"0"`.
    pub(crate) text: String,
    /// Decimal place of the first digit.
    pub(crate) min_place: i32,
    /// One past the decimal place of the last digit.
    pub(crate) max_place: i32,
    /// Added to every value before packing.
    pub(crate) offset: f64,
}

/// Decimals, magnitude limit, fixed offset and fixed places of numbers and of vectors.
struct Shape {
    decimals: usize,
    limit: f64,
    offset: f64,
    min_place: i32,
    max_place: i32,
}

const NUMBERS: Shape = Shape {
    decimals: 3,
    limit: 49_999.0,
    offset: 50_000.0,
    min_place: -3,
    max_place: 5,
};

const VECTORS: Shape = Shape {
    decimals: 2,
    limit: 4_999.0,
    offset: 5_000.0,
    min_place: -2,
    max_place: 4,
};

/// Packs `values`. `fixed` uses the places and offset that `decompress_*` knows by heart;
/// otherwise they are chosen to fit the values, which makes the text shorter.
pub(crate) fn pack(values: &Values, fixed: bool) -> Result<Packed, CompressError> {
    let (shape, items): (&Shape, Vec<Vec<f64>>) = match values {
        Values::Numbers(numbers) => (&NUMBERS, numbers.iter().map(|&n| vec![n]).collect()),
        // The text holds x, then z, then y.
        Values::Vectors(vectors) => (
            &VECTORS,
            vectors.iter().map(|v| vec![v[0], v[2], v[1]]).collect(),
        ),
    };
    if items.is_empty() {
        return Err(CompressError::Empty);
    }
    for &value in items.iter().flatten() {
        if !value.is_finite() || value.abs() >= shape.limit {
            return Err(CompressError::OutOfRange {
                value,
                limit: shape.limit,
            });
        }
    }
    let lowest = items
        .iter()
        .flatten()
        .copied()
        .fold(f64::INFINITY, f64::min);
    let offset = if fixed {
        shape.offset
    } else {
        (-lowest).max(0.0)
    };

    let mut digits: Vec<Vec<String>> = items
        .iter()
        .map(|item| {
            item.iter()
                .map(|&v| reversed_digits(v + offset, shape.decimals))
                .collect()
        })
        .collect();
    let (min_place, max_place) = if fixed {
        (shape.min_place, shape.max_place)
    } else {
        let mut min_place = -i32::try_from(shape.decimals).unwrap_or(0);
        // Drop the zeros every value ends in, keeping at least one digit.
        while digits
            .iter()
            .flatten()
            .all(|d| d.len() > 1 && d.starts_with('0'))
        {
            digits.iter_mut().flatten().for_each(|d| {
                d.remove(0);
            });
            min_place += 1;
        }
        let longest = digits.iter().flatten().map(String::len).max().unwrap_or(1);
        (min_place, min_place + i32::try_from(longest).unwrap_or(0))
    };

    let symbols = alphabet();
    let width = usize::try_from(max_place - min_place)
        .unwrap_or(0)
        .div_ceil(2)
        * 2;
    let text = digits
        .iter()
        .map(|item| {
            let stripped: Vec<String> = item.iter().map(|d| strip_zeros(d)).collect();
            let joined = match stripped.as_slice() {
                [number] => number.clone(),
                [x, z, y] => format!("{x:0<width$}{z:0<width$}{y}"),
                parts => parts.concat(),
            };
            pack_digits(&joined, &symbols)
        })
        .collect::<Vec<_>>()
        .join("0");
    Ok(Packed {
        text,
        min_place,
        max_place,
        offset,
    })
}

/// `value` with `decimals` decimals, without the point, least significant digit first.
fn reversed_digits(value: f64, decimals: usize) -> String {
    format!("{value:.decimals$}")
        .chars()
        .filter(|c| *c != '.')
        .rev()
        .collect()
}

/// Trailing zeros are implied; an all-zero value keeps one digit.
fn strip_zeros(digits: &str) -> String {
    match digits.trim_end_matches('0') {
        "" => "0".to_owned(),
        kept => kept.to_owned(),
    }
}

/// One symbol per pair of digits (the last pair may be a single digit).
fn pack_digits(digits: &str, symbols: &[char]) -> String {
    digits
        .as_bytes()
        .chunks(2)
        .map(|pair| {
            let low = usize::from(pair[0] - b'0');
            let high = pair.get(1).map_or(0, |d| usize::from(d - b'0'));
            symbols[high * 10 + low]
        })
        .collect()
}

#[cfg(test)]
mod tests;
