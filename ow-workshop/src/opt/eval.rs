//! A test-only evaluator of workshop expressions, so property tests can compare an expression
//! before and after optimization.
//!
//! Workshop facts it models:
//! - falsy is `0`, `False`, `Null`, an empty array or an empty string; an array is as truthy as
//!   its first element;
//! - indexing out of range gives `0`, and `Index Of Array Value` gives `-1` when not found;
//! - comparing values of different types coerces: `True == 1`, `Null == 0`, `Array(x) == x`;
//! - division and modulo by zero give `0`.

// A throwaway evaluator: casts between small integers and floats are exact here.
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::float_cmp,
    clippy::many_single_char_names,
    clippy::too_many_lines
)]

use std::collections::HashMap;

use ow_workshop_data::ValueId as V;

use crate::ir::{CmpOp, ConstRef, Expr, ExprKind, StrExpr, StrToken, VarId};

/// A workshop value.
#[derive(Clone, Debug)]
pub(crate) enum Value {
    Num(f64),
    Bool(bool),
    Null,
    Str(String),
    Vector([f64; 3]),
    Array(Vec<Value>),
    /// A constant the evaluator knows nothing about, equal only to itself.
    Opaque(&'static str, &'static str),
}

impl Value {
    pub(crate) fn truthy(&self) -> bool {
        match self {
            Self::Num(n) => *n != 0.0,
            Self::Bool(b) => *b,
            Self::Null => false,
            Self::Str(s) => !s.is_empty(),
            Self::Vector(v) => v.iter().any(|c| *c != 0.0),
            Self::Array(items) => items.first().is_some_and(Self::truthy),
            Self::Opaque(..) => true,
        }
    }

    /// The value as a number: an array is its first element.
    pub(crate) fn num(&self) -> f64 {
        match self {
            Self::Num(n) => *n,
            Self::Bool(b) => f64::from(u8::from(*b)),
            Self::Array(items) => items.first().map_or(0.0, Self::num),
            Self::Null | Self::Str(_) | Self::Vector(_) | Self::Opaque(..) => 0.0,
        }
    }

    fn vector(&self) -> [f64; 3] {
        match self {
            Self::Vector(v) => *v,
            other => [other.num(); 3],
        }
    }

    fn is_vector(&self) -> bool {
        matches!(self, Self::Vector(_))
    }

    fn items(&self) -> Vec<Self> {
        match self {
            Self::Array(items) => items.clone(),
            other => vec![other.clone()],
        }
    }

    fn text(&self) -> String {
        match self {
            Self::Str(s) => s.clone(),
            Self::Num(n) => format!("{n}"),
            Self::Bool(b) => if *b { "1" } else { "0" }.to_owned(),
            Self::Null => "0".to_owned(),
            Self::Array(items) => items.first().map_or_else(String::new, Self::text),
            Self::Vector(_) | Self::Opaque(..) => String::new(),
        }
    }
}

/// Floats are equal exactly in the game; `tolerant` allows for folding that associates
/// operations differently, and is for comparing results across the optimizer only.
fn close(a: f64, b: f64, tolerant: bool) -> bool {
    a == b
        || (tolerant
            && ((a.is_nan() && b.is_nan())
                || (a - b).abs() <= 1e-9 * a.abs().max(b.abs()).max(1.0)))
}

/// Equality as the workshop's `==` computes it.
pub(crate) fn loose_eq(a: &Value, b: &Value) -> bool {
    coerced_eq(a, b, false)
}

/// [`loose_eq`] with floats compared to a tolerance.
pub(crate) fn approx_eq(a: &Value, b: &Value) -> bool {
    coerced_eq(a, b, true)
}

fn coerced_eq(a: &Value, b: &Value, tolerant: bool) -> bool {
    let loose_eq = |x: &Value, y: &Value| coerced_eq(x, y, tolerant);
    match (a, b) {
        (Value::Opaque(x, y), Value::Opaque(p, q)) => (x, y) == (p, q),
        (Value::Opaque(..), _) | (_, Value::Opaque(..)) => false,
        (Value::Str(x), Value::Str(y)) => x == y,
        (Value::Array(x), Value::Array(y)) => {
            x.len() == y.len() && x.iter().zip(y).all(|(p, q)| loose_eq(p, q))
        }
        (Value::Array(x), other) | (other, Value::Array(x)) => {
            // An empty array reads as `0` at index 0, like any out-of-range index.
            loose_eq(&at(x, 0.0), other)
        }
        (x, y) if x.is_vector() || y.is_vector() => {
            let (p, q) = (x.vector(), y.vector());
            (0..3).all(|i| close(p[i], q[i], tolerant))
        }
        (Value::Str(s), other) | (other, Value::Str(s)) => *s == other.text(),
        (x, y) => close(x.num(), y.num(), tolerant),
    }
}

/// Values of the variables an expression may read, and the current array element.
#[derive(Default)]
pub(crate) struct Scope {
    pub(crate) globals: HashMap<u32, Value>,
    elem: Option<(Value, usize)>,
}

/// Evaluates `e`; `None` for anything the evaluator does not model.
pub(crate) fn eval(e: &Expr, scope: &mut Scope) -> Option<Value> {
    use ExprKind as K;
    let mut sub = |x: &Expr| eval(x, scope);
    Some(match e.kind() {
        K::Num(n) => Value::Num(*n),
        K::Bool(b) => Value::Bool(*b),
        K::Null => Value::Null,
        K::Const(ConstRef { enum_name, key }) => Value::Opaque(enum_name, key),
        K::Str(StrExpr::Custom { tokens, args }) => {
            let args = args.iter().map(&mut sub).collect::<Option<Vec<_>>>()?;
            let mut out = String::new();
            for token in tokens {
                match token {
                    StrToken::Text(t) => out.push_str(t),
                    StrToken::Arg(n) => out.push_str(&args.get(*n)?.text()),
                }
            }
            Value::Str(out)
        }
        K::Vector(parts) => {
            let mut v = [0.0; 3];
            for (slot, part) in v.iter_mut().zip(parts) {
                *slot = sub(part)?.num();
            }
            Value::Vector(v)
        }
        K::Array(items) => Value::Array(items.iter().map(sub).collect::<Option<_>>()?),
        K::GlobalVar(VarId(n)) => scope.globals.get(n)?.clone(),
        K::Index { array, index } => {
            let (a, i) = (sub(array)?, sub(index)?);
            at(&a.items(), i.num())
        }
        K::ArrayElem => scope.elem.clone()?.0,
        K::ArrayIndex => Value::Num(scope.elem.clone()?.1 as f64),
        K::Compare { op, lhs, rhs } => {
            let (a, b) = (sub(lhs)?, sub(rhs)?);
            Value::Bool(compare(*op, &a, &b))
        }
        K::And(a, b) => Value::Bool(sub(a)?.truthy() && sub(b)?.truthy()),
        K::Or(a, b) => Value::Bool(sub(a)?.truthy() || sub(b)?.truthy()),
        K::Not(a) => Value::Bool(!sub(a)?.truthy()),
        K::IfThenElse { cond, then, else_ } => {
            if sub(cond)?.truthy() {
                sub(then)?
            } else {
                sub(else_)?
            }
        }
        K::Neg(a) => arith(&sub(a)?, &Value::Num(-1.0), |x, y| x * y),
        K::Add(a, b) => arith(&sub(a)?, &sub(b)?, |x, y| x + y),
        K::Sub(a, b) => arith(&sub(a)?, &sub(b)?, |x, y| x - y),
        K::Mul(a, b) => arith(&sub(a)?, &sub(b)?, |x, y| x * y),
        K::Div(a, b) => arith(
            &sub(a)?,
            &sub(b)?,
            |x, y| if y == 0.0 { 0.0 } else { x / y },
        ),
        K::Mod(a, b) => arith(
            &sub(a)?,
            &sub(b)?,
            |x, y| if y == 0.0 { 0.0 } else { x % y },
        ),
        K::Pow(a, b) => arith(&sub(a)?, &sub(b)?, f64::powf),
        K::Value { id, args } => value(*id, args, scope)?,
        _ => return None,
    })
}

fn at(items: &[Value], index: f64) -> Value {
    // Out of range gives 0, and so does an index that is not a whole number.
    if index < 0.0 || index.is_nan() || index.fract() != 0.0 {
        return Value::Num(0.0);
    }
    items
        .get(index as usize)
        .cloned()
        .unwrap_or(Value::Num(0.0))
}

fn compare(op: CmpOp, a: &Value, b: &Value) -> bool {
    match op {
        CmpOp::Eq => loose_eq(a, b),
        CmpOp::Ne => !loose_eq(a, b),
        CmpOp::Lt => a.num() < b.num(),
        CmpOp::Le => a.num() <= b.num(),
        CmpOp::Gt => a.num() > b.num(),
        CmpOp::Ge => a.num() >= b.num(),
    }
}

/// An operator on numbers, applied component-wise when either side is a vector.
fn arith(a: &Value, b: &Value, f: impl Fn(f64, f64) -> f64) -> Value {
    if a.is_vector() || b.is_vector() {
        let (p, q) = (a.vector(), b.vector());
        Value::Vector([f(p[0], q[0]), f(p[1], q[1]), f(p[2], q[2])])
    } else {
        Value::Num(f(a.num(), b.num()))
    }
}

/// Runs `f` for every element with the current element and index set.
fn each(
    items: &[Value],
    scope: &mut Scope,
    mut f: impl FnMut(&mut Scope, &Value) -> Option<()>,
) -> Option<()> {
    let outer = scope.elem.take();
    for (i, item) in items.iter().enumerate() {
        scope.elem = Some((item.clone(), i));
        f(scope, item)?;
    }
    scope.elem = outer;
    Some(())
}

fn value(id: V, args: &[Expr], scope: &mut Scope) -> Option<Value> {
    let arg = |i: usize, scope: &mut Scope| eval(args.get(i)?, scope);
    Some(match id {
        V::True => Value::Bool(true),
        V::False => Value::Bool(false),
        V::Null => Value::Null,
        V::EmptyArray => Value::Array(vec![]),
        V::Array => Value::Array(args.iter().map(|a| eval(a, scope)).collect::<Option<_>>()?),
        V::Abs => match arg(0, scope)? {
            Value::Vector(v) => Value::Num(v.iter().map(|c| c * c).sum::<f64>().sqrt()),
            other => Value::Num(other.num().abs()),
        },
        V::Sqrt => Value::Num(arg(0, scope)?.num().sqrt()),
        V::Min => Value::Num(arg(0, scope)?.num().min(arg(1, scope)?.num())),
        V::Max => Value::Num(arg(0, scope)?.num().max(arg(1, scope)?.num())),
        V::Sin => Value::Num(arg(0, scope)?.num().sin()),
        V::Cos => Value::Num(arg(0, scope)?.num().cos()),
        V::Round => {
            let x = arg(0, scope)?.num();
            let ExprKind::Const(c) = args.get(1)?.kind() else {
                return None;
            };
            Value::Num(match c.key {
                "__roundDown__" => x.floor(),
                "__roundUp__" => x.ceil(),
                _ => x.round(),
            })
        }
        V::Len => Value::Num(arg(0, scope)?.items().len() as f64),
        V::FirstOf => at(&arg(0, scope)?.items(), 0.0),
        V::Last => arg(0, scope)?
            .items()
            .last()
            .cloned()
            .unwrap_or(Value::Num(0.0)),
        V::ArrayContains => {
            let (a, x) = (arg(0, scope)?, arg(1, scope)?);
            Value::Bool(a.items().iter().any(|item| loose_eq(item, &x)))
        }
        V::Index => {
            let (a, x) = (arg(0, scope)?, arg(1, scope)?);
            let found = a.items().iter().position(|item| loose_eq(item, &x));
            Value::Num(found.map_or(-1.0, |i| i as f64))
        }
        V::Slice => {
            let (a, start, count) = (arg(0, scope)?, arg(1, scope)?.num(), arg(2, scope)?.num());
            // The window `start..start + count` clipped to the array.
            let items = a.items();
            let from = start.max(0.0) as usize;
            let to = (start + count.max(0.0)).max(0.0) as usize;
            Value::Array(
                items
                    .get(from..to.min(items.len()).max(from))
                    .unwrap_or(&[])
                    .to_vec(),
            )
        }
        V::FilteredArray | V::MappedArray | V::SortedArray | V::All | V::Any => {
            let items = arg(0, scope)?.items();
            let body = args.get(1)?;
            let mut results = Vec::new();
            each(&items, scope, |scope, _| {
                results.push(eval(body, scope)?);
                Some(())
            })?;
            match id {
                V::FilteredArray => Value::Array(
                    items
                        .into_iter()
                        .zip(&results)
                        .filter(|(_, r)| r.truthy())
                        .map(|(i, _)| i)
                        .collect(),
                ),
                V::MappedArray => Value::Array(results),
                V::All => Value::Bool(results.iter().all(Value::truthy)),
                V::Any => Value::Bool(results.iter().any(Value::truthy)),
                _ => {
                    let mut keyed: Vec<_> = items.into_iter().zip(results).collect();
                    keyed.sort_by(|a, b| a.1.num().total_cmp(&b.1.num()));
                    Value::Array(keyed.into_iter().map(|(i, _)| i).collect())
                }
            }
        }
        V::CharAt => {
            let (s, i) = (arg(0, scope)?.text(), arg(1, scope)?.num());
            let c = if i < 0.0 {
                None
            } else {
                s.chars().nth(i as usize)
            };
            Value::Str(c.map(String::from).unwrap_or_default())
        }
        V::StrContains => Value::Bool(arg(0, scope)?.text().contains(&arg(1, scope)?.text())),
        V::Replace => {
            let (s, from, to) = (
                arg(0, scope)?.text(),
                arg(1, scope)?.text(),
                arg(2, scope)?.text(),
            );
            Value::Str(if from.is_empty() {
                s
            } else {
                s.replace(&from, &to)
            })
        }
        V::Split => {
            let (s, sep) = (arg(0, scope)?.text(), arg(1, scope)?.text());
            Value::Array(if sep.is_empty() {
                s.chars().map(|c| Value::Str(c.to_string())).collect()
            } else {
                s.split(&sep).map(|p| Value::Str(p.to_owned())).collect()
            })
        }
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lower::testutil::{call, num};
    use crate::opt::testutil::{arr, cmp, index, ite, not, tru};

    fn run(e: &Expr) -> Value {
        eval(e, &mut Scope::default()).expect("evaluates")
    }

    #[test]
    fn falsy_values_and_array_truthiness() {
        assert!(!run(&not(num(1.0))).truthy());
        assert!(!Value::Array(vec![]).truthy());
        assert!(!Value::Array(vec![Value::Num(0.0), Value::Num(5.0)]).truthy());
        assert!(Value::Array(vec![Value::Num(2.0)]).truthy());
        assert!(!Value::Null.truthy() && !Value::Str(String::new()).truthy());
    }

    #[test]
    fn indexing_out_of_range_gives_zero() {
        let a = arr(vec![num(5.0)]);
        assert!(loose_eq(
            &run(&index(a.clone(), num(0.0))),
            &Value::Num(5.0)
        ));
        assert!(loose_eq(
            &run(&index(a.clone(), num(3.0))),
            &Value::Num(0.0)
        ));
        assert!(loose_eq(&run(&index(a, num(-1.0))), &Value::Num(0.0)));
    }

    #[test]
    fn index_of_missing_value_is_minus_one() {
        let found = call(
            ow_workshop_data::ValueId::Index,
            vec![arr(vec![num(1.0), num(2.0)]), num(2.0)],
        );
        let missing = call(
            ow_workshop_data::ValueId::Index,
            vec![arr(vec![num(1.0)]), num(9.0)],
        );
        assert!(loose_eq(&run(&found), &Value::Num(1.0)));
        assert!(loose_eq(&run(&missing), &Value::Num(-1.0)));
    }

    #[test]
    fn comparisons_coerce_types() {
        let eq = |a: Expr, b: Expr| matches!(run(&cmp(CmpOp::Eq, a, b)), Value::Bool(true));
        assert!(eq(tru(), num(1.0)));
        assert!(eq(crate::lower::testutil::null(), num(0.0)));
        assert!(eq(arr(vec![num(7.0)]), num(7.0)));
        assert!(!eq(num(1.0), num(2.0)));
    }

    #[test]
    fn if_then_else_and_vectors() {
        assert!(loose_eq(
            &run(&ite(tru(), num(1.0), num(2.0))),
            &Value::Num(1.0)
        ));
        let v = Expr::new(ExprKind::Vector([num(1.0), num(2.0), num(3.0)]), None);
        let doubled = Expr::new(ExprKind::Mul(v, num(2.0)), None);
        assert!(loose_eq(&run(&doubled), &Value::Vector([2.0, 4.0, 6.0])));
    }
}
