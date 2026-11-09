// A throwaway evaluator: casts between small integers and floats are exact here.
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::many_single_char_names
)]

use ow_workshop_data::ValueId;
use proptest::prelude::*;

use super::*;
use crate::expr::vect;
use crate::ir::VarId;
use crate::lower::compression::decode;

/// A value of the tiny evaluator below.
#[derive(Clone, Debug, PartialEq)]
enum Val {
    Num(f64),
    Text(String),
    List(Vec<Val>),
    Vec3([f64; 3]),
}

/// Evaluates the expanded decoding expression.
///
/// Models the workshop facts the decoder relies on: `Null` is the text `0`, a character past the
/// end of a string is empty and sits at position 0 of any string, and a list used as a string
/// is its first item.
struct Eval {
    alphabet: Vec<char>,
    elem: Option<Val>,
}

impl Eval {
    fn text(val: &Val) -> String {
        match val {
            Val::Text(t) => t.clone(),
            Val::List(items) => items.first().map(Self::text).unwrap_or_default(),
            Val::Num(n) => n.to_string(),
            Val::Vec3(_) => String::new(),
        }
    }

    fn num(val: &Val) -> f64 {
        match val {
            Val::Num(n) => *n,
            _ => f64::NAN,
        }
    }

    fn eval(&mut self, e: &ir::Expr) -> Val {
        match e.kind() {
            ExprKind::Num(n) => Val::Num(*n),
            ExprKind::Null => Val::Text("0".to_owned()),
            ExprKind::Str(StrExpr::Custom { tokens, .. }) => Val::Text(
                tokens
                    .iter()
                    .map(|t| match t {
                        StrToken::Text(t) => t.as_str(),
                        StrToken::Arg(_) => "",
                    })
                    .collect(),
            ),
            ExprKind::GlobalVar(_) => Val::Text(self.alphabet.iter().collect()),
            ExprKind::ArrayElem => self.elem.clone().expect("inside a lambda"),
            ExprKind::Vector([x, y, z]) => Val::Vec3([x, y, z].map(|p| Self::num(&self.eval(p)))),
            ExprKind::Add(a, b) => self.arith(a, b, |a, b| a + b),
            ExprKind::Sub(a, b) => self.arith(a, b, |a, b| a - b),
            ExprKind::Mul(a, b) => self.arith(a, b, |a, b| a * b),
            ExprKind::Pow(a, b) => self.arith(a, b, f64::powf),
            ExprKind::Value { id, args } => self.call(*id, args),
            other => panic!("the decoder does not use {other:?}"),
        }
    }

    fn arith(&mut self, a: &ir::Expr, b: &ir::Expr, f: impl Fn(f64, f64) -> f64) -> Val {
        match (self.eval(a), self.eval(b)) {
            (Val::Num(a), Val::Num(b)) => Val::Num(f(a, b)),
            (Val::Vec3(v), Val::Num(n)) | (Val::Num(n), Val::Vec3(v)) => {
                Val::Vec3(v.map(|c| f(c, n)))
            }
            (Val::Vec3(a), Val::Vec3(b)) => Val::Vec3([0, 1, 2].map(|i| f(a[i], b[i]))),
            (a, b) => panic!("cannot combine {a:?} and {b:?}"),
        }
    }

    fn call(&mut self, id: ValueId, args: &[ir::Expr]) -> Val {
        match id {
            ValueId::FirstOf => {
                let first = self.eval(&args[0]);
                match first {
                    Val::List(items) => {
                        items.into_iter().next().unwrap_or(Val::Text(String::new()))
                    }
                    other => other,
                }
            }
            ValueId::Last => match self.eval(&args[0]) {
                Val::List(items) => items.last().cloned().expect("not empty"),
                other => other,
            },
            ValueId::Split => {
                let text = Self::text(&self.eval(&args[0]));
                let separator = Self::text(&self.eval(&args[1]));
                Val::List(
                    text.split(&separator)
                        .map(|p| Val::Text(p.to_owned()))
                        .collect(),
                )
            }
            ValueId::Concat => {
                let mut items = match self.eval(&args[0]) {
                    Val::List(items) => items,
                    single => vec![single],
                };
                items.push(self.eval(&args[1]));
                Val::List(items)
            }
            ValueId::MappedArray => {
                let Val::List(items) = self.eval(&args[0]) else {
                    panic!("mapping a non-list");
                };
                let mapped = items
                    .into_iter()
                    .map(|item| {
                        let outer = self.elem.replace(item);
                        let out = self.eval(&args[1]);
                        self.elem = outer;
                        out
                    })
                    .collect();
                Val::List(mapped)
            }
            ValueId::CharAt => {
                let text = Self::text(&self.eval(&args[0]));
                let index = Self::num(&self.eval(&args[1]));
                let ch = text.chars().nth(index as usize).map(String::from);
                Val::Text(ch.unwrap_or_default())
            }
            ValueId::StrIndex => {
                let symbols: Vec<char> = Self::text(&self.eval(&args[0])).chars().collect();
                let wanted = Self::text(&self.eval(&args[1]));
                let position = match wanted.chars().next() {
                    None => 0.0,
                    Some(c) => symbols
                        .iter()
                        .position(|s| *s == c)
                        .map_or(-1.0, |p| p as f64),
                };
                Val::Num(position)
            }
            other => panic!("the decoder does not call {other:?}"),
        }
    }
}

fn run(d: &Decompress, with_variable: bool) -> Vec<Val> {
    let var = with_variable.then_some(VarId(0));
    let expanded = decode(d, None, var);
    let mut eval = Eval {
        alphabet: alphabet::alphabet(),
        elem: None,
    };
    match eval.eval(&expanded) {
        Val::List(items) => items,
        other => panic!("not a list: {other:?}"),
    }
}

fn decompress_of(e: &Expr<impl Sized>) -> &Decompress {
    match e.ir().kind() {
        ExprKind::Decompress(d) => d,
        other => panic!("not a decompress node: {other:?}"),
    }
}

fn numbers(values: &[f64]) -> Expr<Array<Num>> {
    values
        .iter()
        .map(|&v| Expr::<Num>::from(v))
        .collect::<Vec<_>>()
        .into()
}

fn vectors(values: &[[f64; 3]]) -> Expr<Array<Vector>> {
    values
        .iter()
        .map(|v| vect(v[0], v[1], v[2]))
        .collect::<Vec<_>>()
        .into()
}

fn assert_numbers(got: &[Val], expected: &[f64], tolerance: f64) {
    assert_eq!(got.len(), expected.len());
    for (val, want) in got.iter().zip(expected) {
        let Val::Num(n) = val else {
            panic!("not a number: {val:?}")
        };
        assert!(
            (n - want).abs() <= tolerance,
            "{n} is not within {tolerance} of {want}"
        );
    }
}

fn assert_vectors(got: &[Val], expected: &[[f64; 3]], tolerance: f64) {
    assert_eq!(got.len(), expected.len());
    for (val, want) in got.iter().zip(expected) {
        let Val::Vec3(v) = val else {
            panic!("not a vector: {val:?}")
        };
        for i in 0..3 {
            assert!(
                (v[i] - want[i]).abs() <= tolerance,
                "{v:?} is not within {tolerance} of {want:?}"
            );
        }
    }
}

#[test]
fn a_small_number_array_round_trips() {
    let values = [3.0, -1.5, 0.0, 12.25, 100.0];
    let array = compressed::<Num>(numbers(&values)).unwrap();
    for with_variable in [false, true] {
        assert_numbers(&run(decompress_of(&array), with_variable), &values, 1e-3);
    }
}

#[test]
fn a_small_vector_array_round_trips() {
    let values = [[1.0, 2.0, 3.0], [-4.5, 0.0, 250.25], [0.0, -10.0, 7.0]];
    let array = compressed::<Vector>(vectors(&values)).unwrap();
    for with_variable in [false, true] {
        assert_vectors(&run(decompress_of(&array), with_variable), &values, 1e-2);
    }
}

#[test]
fn null_items_are_zero() {
    let array: Expr<Array<Num>> = vec![
        Expr::<Num>::from(5),
        crate::expr::null(),
        Expr::<Num>::from(-2),
    ]
    .into();
    let got = run(decompress_of(&compressed::<Num>(array).unwrap()), false);
    assert_numbers(&got, &[5.0, 0.0, -2.0], 1e-3);
}

#[test]
fn fixed_mode_round_trips_through_the_fixed_decoders() {
    let values = [49_000.123, -49_000.123, 0.0, 1.0];
    let packed = compress::<Num>(numbers(&values)).unwrap();
    let decoded = decompress_numbers(packed);
    assert_numbers(&run(decompress_of(&decoded), false), &values, 1e-3);

    let points = [[4_000.5, -4_000.5, 0.0], [1.0, 2.0, 3.0]];
    let packed = compress::<Vector>(vectors(&points)).unwrap();
    let decoded = decompress_vectors(packed);
    assert_vectors(&run(decompress_of(&decoded), true), &points, 1e-2);
}

#[test]
fn auto_mode_is_shorter_than_fixed_mode() {
    let values = numbers(&[1.0, 2.0, 3.0]);
    let auto = compressed::<Num>(values.clone()).unwrap();
    let ExprKind::Str(StrExpr::Custom { tokens: auto, .. }) =
        decompress_of(&auto).packed.kind().clone()
    else {
        panic!("not a string");
    };
    let ExprKind::Str(StrExpr::Custom { tokens: fixed, .. }) =
        compress::<Num>(values).unwrap().ir().kind().clone()
    else {
        panic!("not a string");
    };
    let len = |t: &[StrToken]| {
        t.iter()
            .map(|t| match t {
                StrToken::Text(s) => s.chars().count(),
                StrToken::Arg(_) => 0,
            })
            .sum::<usize>()
    };
    assert!(len(&auto) < len(&fixed));
}

#[test]
fn carriage_returns_in_the_text_survive() {
    // Every symbol is a possible digit, including U+000D, which `Expr<Str>::from` would drop.
    let digits: Vec<f64> = (0..100).map(f64::from).collect();
    let array = compressed::<Num>(numbers(&digits)).unwrap();
    assert_numbers(&run(decompress_of(&array), false), &digits, 1e-3);
}

#[test]
fn errors() {
    assert_eq!(
        compressed::<Num>(numbers(&[])).unwrap_err(),
        CompressError::Empty
    );
    assert_eq!(
        compressed::<Num>(numbers(&[1e6])).unwrap_err(),
        CompressError::OutOfRange {
            value: 1e6,
            limit: 49_999.0
        }
    );
    let not_literal: Expr<Array<Num>> = vec![crate::generated::api::match_time()].into();
    assert_eq!(
        compressed::<Num>(not_literal).unwrap_err(),
        CompressError::NotLiteral
    );
    assert_eq!(
        compress::<Num>(crate::expr::null::<Array<Num>>()).unwrap_err(),
        CompressError::NotLiteral
    );
}

proptest! {
    #[test]
    fn numbers_decode_within_a_thousandth(values in prop::collection::vec(-49_000.0f64..49_000.0, 1..40)) {
        let array = compressed::<Num>(numbers(&values)).unwrap();
        for with_variable in [false, true] {
            assert_numbers(&run(decompress_of(&array), with_variable), &values, 1e-3);
        }
        let fixed = decompress_numbers(compress::<Num>(numbers(&values)).unwrap());
        assert_numbers(&run(decompress_of(&fixed), false), &values, 1e-3);
    }

    #[test]
    fn vectors_decode_within_a_hundredth(values in prop::collection::vec(prop::array::uniform3(-4_900.0f64..4_900.0), 1..30)) {
        let array = compressed::<Vector>(vectors(&values)).unwrap();
        for with_variable in [false, true] {
            assert_vectors(&run(decompress_of(&array), with_variable), &values, 1e-2);
        }
        let fixed = decompress_vectors(compress::<Vector>(vectors(&values)).unwrap());
        assert_vectors(&run(decompress_of(&fixed), false), &values, 1e-2);
    }
}
