use super::write_number;

fn fmt(n: f64) -> String {
    let mut out = String::new();
    write_number(&mut out, n).unwrap();
    out
}

#[test]
fn formats_like_javascript() {
    let table = [
        (74940.0, "74940"),
        (-292.0, "-292"),
        (0.016, "0.016"),
        (0.1, "0.1"),
        (1e21, "1e+21"),
        (1.5, "1.5"),
        (100.0, "100"),
        (-0.5, "-0.5"),
    ];
    for (n, expected) in table {
        assert_eq!(fmt(n), expected, "{n}");
    }
}

#[test]
fn truncates_the_fraction_without_rounding() {
    assert_eq!(fmt(0.123_456_789_012_345_68), "0.123456789012345");
    assert_eq!(fmt(0.999_999_999_999_999_9), "0.999999999999999");
}

#[test]
fn keeps_the_exponent_when_truncating() {
    assert_eq!(fmt(1.224_646_799_147_353_2e-16), "1.224646799147353e-16");
}

#[test]
fn short_fractions_with_exponent_are_unchanged() {
    assert_eq!(fmt(1.5e-7), "1.5e-7");
}

#[test]
fn zero_and_negative_zero_print_as_zero() {
    assert_eq!(fmt(0.0), "0");
    assert_eq!(fmt(-0.0), "0");
}

#[test]
fn infinity_prints_as_a_large_number() {
    assert_eq!(fmt(f64::INFINITY), "999999999999");
    assert_eq!(fmt(f64::NEG_INFINITY), "-999999999999");
}
