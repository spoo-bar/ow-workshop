use super::*;

fn numbers(values: &[f64]) -> Values {
    Values::Numbers(values.to_vec())
}

#[test]
fn digits_are_written_least_significant_first() {
    assert_eq!(reversed_digits(12.5, 3), "00521");
    assert_eq!(reversed_digits(0.0, 2), "000");
}

#[test]
fn trailing_zeros_are_implied() {
    assert_eq!(strip_zeros("12300"), "123");
    assert_eq!(strip_zeros("000"), "0");
    assert_eq!(strip_zeros("1"), "1");
}

#[test]
fn digit_pairs_become_symbols() {
    let symbols = alphabet();
    // "21" is read back to front: 1 + 10 * 2 = 12.
    assert_eq!(pack_digits("21", &symbols), symbols[12].to_string());
    assert_eq!(pack_digits("5", &symbols), symbols[5].to_string());
    assert_eq!(
        pack_digits("0021", &symbols),
        format!("{}{}", symbols[0], symbols[12])
    );
}

#[test]
fn auto_mode_fits_the_places_to_the_values() {
    let packed = pack(&numbers(&[3.0, 5.0, 7.0]), false).unwrap();
    // All integers: the three fractional zeros are dropped, one digit remains.
    assert_eq!(
        (packed.min_place, packed.max_place, packed.offset),
        (0, 1, 0.0)
    );
    let symbols = alphabet();
    let expected: String = ["3", "5", "7"]
        .into_iter()
        .map(|d| symbols[d.parse::<usize>().unwrap()].to_string())
        .collect::<Vec<_>>()
        .join("0");
    assert_eq!(packed.text, expected);
}

#[test]
fn negative_values_are_offset_by_the_minimum() {
    let packed = pack(&numbers(&[-2.5, 1.0]), false).unwrap();
    assert_eq!(packed.offset, 2.5);
}

#[test]
fn fixed_mode_uses_the_places_the_decoder_knows() {
    let numbers = pack(&numbers(&[1.5]), true).unwrap();
    assert_eq!(
        (numbers.min_place, numbers.max_place, numbers.offset),
        (-3, 5, 50_000.0)
    );
    let vectors = pack(&Values::Vectors(vec![[1.0, 2.0, 3.0]]), true).unwrap();
    assert_eq!(
        (vectors.min_place, vectors.max_place, vectors.offset),
        (-2, 4, 5_000.0)
    );
}

#[test]
fn vectors_pad_x_and_z_but_not_y() {
    // Fixed vectors: three symbols per component. 5001.00 -> "100500" -> x is 3 symbols,
    // z is 3 symbols, y (5002.00 -> "2005" after stripping) is 2.
    let packed = pack(&Values::Vectors(vec![[1.0, 2.0, 3.0]]), true).unwrap();
    assert_eq!(packed.text.chars().count(), 3 + 3 + 3);
    let y_stripped = pack(&Values::Vectors(vec![[1.0, 0.0, 3.0]]), true).unwrap();
    assert_eq!(y_stripped.text.chars().count(), 3 + 3 + 3);
}

#[test]
fn items_are_separated_by_a_zero() {
    let packed = pack(&numbers(&[1.0, 2.0, 3.0]), false).unwrap();
    assert_eq!(packed.text.matches('0').count(), 2);
}

#[test]
fn all_zeros_keep_one_digit() {
    let packed = pack(&numbers(&[0.0, 0.0]), false).unwrap();
    assert_eq!((packed.min_place, packed.max_place), (0, 1));
    assert_eq!(packed.text.chars().count(), 3);
}

#[test]
fn empty_and_out_of_range_inputs_are_errors() {
    assert_eq!(
        pack(&numbers(&[]), false).unwrap_err(),
        CompressError::Empty
    );
    assert_eq!(
        pack(&numbers(&[50_000.0]), false).unwrap_err(),
        CompressError::OutOfRange {
            value: 50_000.0,
            limit: 49_999.0
        }
    );
    assert_eq!(
        pack(&Values::Vectors(vec![[0.0, 5_000.0, 0.0]]), false).unwrap_err(),
        CompressError::OutOfRange {
            value: 5_000.0,
            limit: 4_999.0
        }
    );
    assert!(pack(&numbers(&[f64::NAN]), false).is_err());
}
