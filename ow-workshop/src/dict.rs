//! Dictionaries: a lookup convenience that can only be read, never stored.

use std::marker::PhantomData;

use crate::expr::Expr;
use crate::ir::{self, ExprKind};
use crate::ty::{IntoExpr, ValueType};

/// A key-value table; consume it with [`Dict::get`]. Missing keys give `Null` unless a default is set.
#[derive(Clone, Debug)]
pub struct Dict<K, V> {
    entries: Vec<(ir::Expr, ir::Expr)>,
    default: Option<ir::Expr>,
    _types: PhantomData<fn() -> (K, V)>,
}

/// Builds a dictionary from key-value pairs; name the types: `dict::<Num, Str>([(1, "a")])`.
#[must_use]
pub fn dict<K, V, KE, VE>(entries: impl IntoIterator<Item = (KE, VE)>) -> Dict<K, V>
where
    K: ValueType,
    V: ValueType,
    KE: IntoExpr<K>,
    VE: IntoExpr<V>,
{
    Dict {
        entries: entries
            .into_iter()
            .map(|(k, v)| (k.into_expr().into_ir(), v.into_expr().into_ir()))
            .collect(),
        default: None,
        _types: PhantomData,
    }
}

impl<K: ValueType, V: ValueType> Dict<K, V> {
    /// The value for keys that are not in the table.
    #[must_use]
    pub fn or_default(mut self, default: impl IntoExpr<V>) -> Self {
        self.default = Some(default.into_expr().into_ir());
        self
    }

    /// Looks `key` up; lowering turns this into array lookups.
    #[track_caller]
    #[must_use]
    pub fn get(&self, key: impl IntoExpr<K>) -> Expr<V> {
        let table = ir::Expr::here(ExprKind::Dict {
            entries: self.entries.clone(),
            default: self.default.clone(),
        });
        Expr::from_ir(ir::Expr::here(ExprKind::Index {
            array: table,
            index: key.into_expr().into_ir(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ty::{Num, Str};

    #[test]
    fn get_looks_up_in_the_table() {
        let d = dict::<Num, Str, _, _>([(1, "one"), (2, "two")]);
        let e = d.get(2);
        let ExprKind::Index { array, index } = e.ir().kind() else {
            panic!("not an index")
        };
        assert_eq!(index.kind(), &ExprKind::Num(2.0));
        let ExprKind::Dict { entries, default } = array.kind() else {
            panic!("not a dict")
        };
        assert_eq!(entries.len(), 2);
        assert!(default.is_none());
    }

    #[test]
    fn default_is_recorded() {
        let d = dict::<Num, Num, _, _>([(1, 10)]).or_default(0);
        let e = d.get(5);
        let ExprKind::Index { array, .. } = e.ir().kind() else {
            panic!("not an index")
        };
        let ExprKind::Dict { default, .. } = array.kind() else {
            panic!("not a dict")
        };
        assert_eq!(
            default.as_ref().map(ir::Expr::kind),
            Some(&ExprKind::Num(0.0))
        );
    }
}
