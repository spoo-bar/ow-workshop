//! Array lambdas, membership and indexing on `Expr<Array<T>>`.
//!
//! The workshop's array functions take an expression over `Current Array Element` and
//! `Current Array Index`, so they cannot be nested.

use ow_workshop_data::ValueId;

use crate::expr::Expr;
use crate::ir::{self, ExprKind};
use crate::ty::{Array, Bool, IntoExpr, Num, ValueType};

impl<T: ValueType> Expr<Array<T>> {
    #[track_caller]
    fn call<R>(&self, id: ValueId, rest: impl IntoIterator<Item = ir::Expr>) -> Expr<R> {
        let mut args = vec![self.ir().clone()];
        args.extend(rest);
        Expr::from_ir(ir::Expr::here(ExprKind::Value { id, args }))
    }

    /// Runs `body` with the current element and index placeholders.
    #[track_caller]
    fn lambda<R>(body: impl FnOnce(Expr<T>, Expr<Num>) -> Expr<R>) -> ir::Expr {
        let elem = Expr::from_ir(ir::Expr::here(ExprKind::ArrayElem));
        let index = Expr::from_ir(ir::Expr::here(ExprKind::ArrayIndex));
        body(elem, index).into_ir()
    }

    /// `Filtered Array`: the elements for which the closure returns true.
    #[track_caller]
    #[must_use]
    pub fn filter(&self, cond: impl FnOnce(Expr<T>, Expr<Num>) -> Expr<Bool>) -> Expr<Array<T>> {
        let body = Self::lambda(cond);
        self.call(ValueId::FilteredArray, [body])
    }

    /// `Mapped Array`: the closure's result for each element.
    #[track_caller]
    #[must_use]
    pub fn map<U: ValueType>(
        &self,
        f: impl FnOnce(Expr<T>, Expr<Num>) -> Expr<U>,
    ) -> Expr<Array<U>> {
        let body = Self::lambda(f);
        self.call(ValueId::MappedArray, [body])
    }

    /// `Is True For All`.
    #[track_caller]
    #[must_use]
    pub fn all(&self, cond: impl FnOnce(Expr<T>, Expr<Num>) -> Expr<Bool>) -> Expr<Bool> {
        let body = Self::lambda(cond);
        self.call(ValueId::All, [body])
    }

    /// `Is True For Any`.
    #[track_caller]
    #[must_use]
    pub fn any(&self, cond: impl FnOnce(Expr<T>, Expr<Num>) -> Expr<Bool>) -> Expr<Bool> {
        let body = Self::lambda(cond);
        self.call(ValueId::Any, [body])
    }

    /// `Sorted Array`: ordered by the closure's key, ascending.
    #[track_caller]
    #[must_use]
    pub fn sorted_by(&self, key: impl FnOnce(Expr<T>, Expr<Num>) -> Expr<Num>) -> Expr<Array<T>> {
        let body = Self::lambda(key);
        self.call(ValueId::SortedArray, [body])
    }

    /// `Is True For All` over the elements themselves.
    #[track_caller]
    #[must_use]
    pub fn all_true(&self) -> Expr<Bool> {
        self.all(|e, _| e.erase().cast())
    }

    /// `Is True For Any` over the elements themselves.
    #[track_caller]
    #[must_use]
    pub fn any_true(&self) -> Expr<Bool> {
        self.any(|e, _| e.erase().cast())
    }

    /// `Array Contains`.
    #[track_caller]
    #[must_use]
    pub fn contains(&self, value: impl IntoExpr<T>) -> Expr<Bool> {
        self.call(ValueId::ArrayContains, [value.into_expr().into_ir()])
    }

    /// `Index Of Array Value`; -1 when absent.
    #[track_caller]
    #[must_use]
    pub fn index_of(&self, value: impl IntoExpr<T>) -> Expr<Num> {
        self.call(ValueId::Index, [value.into_expr().into_ir()])
    }

    /// `Value In Array`.
    #[track_caller]
    #[must_use]
    pub fn get(&self, index: impl IntoExpr<Num>) -> Expr<T> {
        Expr::from_ir(ir::Expr::here(ExprKind::Index {
            array: self.ir().clone(),
            index: index.into_expr().into_ir(),
        }))
    }

    /// `First Of`.
    #[track_caller]
    #[must_use]
    pub fn first(&self) -> Expr<T> {
        self.call(ValueId::FirstOf, [])
    }

    /// `Last Of`.
    #[track_caller]
    #[must_use]
    pub fn last(&self) -> Expr<T> {
        self.call(ValueId::Last, [])
    }

    /// `Count Of`.
    #[track_caller]
    #[must_use]
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> Expr<Num> {
        self.call(ValueId::Len, [])
    }

    /// `Array Slice`: `count` elements from `start`.
    #[track_caller]
    #[must_use]
    pub fn slice(&self, start: impl IntoExpr<Num>, count: impl IntoExpr<Num>) -> Expr<Array<T>> {
        self.call(
            ValueId::Slice,
            [start.into_expr().into_ir(), count.into_expr().into_ir()],
        )
    }

    /// `Append To Array` with another array.
    #[track_caller]
    #[must_use]
    pub fn concat(&self, other: impl IntoExpr<Array<T>>) -> Expr<Array<T>> {
        self.call(ValueId::Concat, [other.into_expr().into_ir()])
    }

    /// `Append To Array` with a single element.
    #[track_caller]
    #[must_use]
    pub fn with(&self, value: impl IntoExpr<T>) -> Expr<Array<T>> {
        self.call(ValueId::Concat, [value.into_expr().into_ir()])
    }

    /// `Remove From Array` with an array of elements to drop.
    #[track_caller]
    #[must_use]
    pub fn exclude(&self, other: impl IntoExpr<Array<T>>) -> Expr<Array<T>> {
        self.call(ValueId::Exclude, [other.into_expr().into_ir()])
    }

    /// `Remove From Array` with a single element to drop.
    #[track_caller]
    #[must_use]
    pub fn without(&self, value: impl IntoExpr<T>) -> Expr<Array<T>> {
        self.call(ValueId::Exclude, [value.into_expr().into_ir()])
    }

    /// `Random Value In Array`.
    #[track_caller]
    #[must_use]
    pub fn random_choice(&self) -> Expr<T> {
        self.call(ValueId::RandomChoice, [])
    }

    /// `Randomized Array`.
    #[track_caller]
    #[must_use]
    pub fn shuffled(&self) -> Expr<Array<T>> {
        self.call(ValueId::RandomShuffle, [])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{Diagnostics, Program};
    use crate::prelude::*;

    fn nums() -> Expr<Array<Num>> {
        [1, 2, 3].into_expr()
    }

    fn call(e: &Expr<impl ValueType>) -> (ValueId, &[ir::Expr]) {
        match e.ir().kind() {
            ExprKind::Value { id, args } => (*id, args),
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn lambdas_bind_the_element_and_index() {
        let filtered = nums().filter(|e, i| e.gt(1).and(i.lt(5)));
        let (id, args) = call(&filtered);
        assert_eq!(id, ValueId::FilteredArray);
        assert_eq!(args.len(), 2);
        assert!(matches!(args[0].kind(), ExprKind::Array(_)));
        let ExprKind::And(lhs, rhs) = args[1].kind() else {
            panic!("not an and")
        };
        let ExprKind::Compare { lhs: e, .. } = lhs.kind() else {
            panic!("not a compare")
        };
        assert_eq!(e.kind(), &ExprKind::ArrayElem);
        let ExprKind::Compare { lhs: i, .. } = rhs.kind() else {
            panic!("not a compare")
        };
        assert_eq!(i.kind(), &ExprKind::ArrayIndex);

        let mapped: Expr<Array<Num>> = nums().map(|e, _| e * 2);
        assert_eq!(call(&mapped).0, ValueId::MappedArray);
        assert_eq!(call(&nums().all(|e, _| e.gt(0))).0, ValueId::All);
        assert_eq!(call(&nums().any(|e, _| e.gt(0))).0, ValueId::Any);
        assert_eq!(call(&nums().sorted_by(|e, _| -e)).0, ValueId::SortedArray);
    }

    #[test]
    fn closure_free_truthiness() {
        let flags: Expr<Array<Bool>> = [true, false].into_expr();
        let all = flags.all_true();
        let (id, args) = call(&all);
        assert_eq!(id, ValueId::All);
        assert_eq!(args[1].kind(), &ExprKind::ArrayElem);
        assert_eq!(call(&nums().any_true()).0, ValueId::Any);
    }

    #[test]
    fn plain_methods() {
        assert_eq!(call(&nums().contains(2)).0, ValueId::ArrayContains);
        assert_eq!(call(&nums().index_of(2)).0, ValueId::Index);
        assert_eq!(call(&nums().first()).0, ValueId::FirstOf);
        assert_eq!(call(&nums().last()).0, ValueId::Last);
        assert_eq!(call(&nums().len()).0, ValueId::Len);
        let sliced = nums().slice(1, 2);
        let (id, args) = call(&sliced);
        assert_eq!((id, args.len()), (ValueId::Slice, 3));
        assert_eq!(call(&nums().concat([4])).0, ValueId::Concat);
        assert_eq!(call(&nums().with(4)).0, ValueId::Concat);
        assert_eq!(call(&nums().exclude([1])).0, ValueId::Exclude);
        assert_eq!(call(&nums().without(1)).0, ValueId::Exclude);
        assert_eq!(call(&nums().random_choice()).0, ValueId::RandomChoice);
        assert_eq!(call(&nums().shuffled()).0, ValueId::RandomShuffle);
        assert!(matches!(nums().get(0).ir().kind(), ExprKind::Index { .. }));
    }

    fn nested_errors(build: impl FnOnce(&mut Block)) -> Diagnostics {
        let mut ws = Builder::new();
        ws.rule("r", OngoingGlobal, |r, _| r.actions(build));
        let (program, _): (Program, _) = ws.finish(Program::default());
        let mut diags = Diagnostics::default();
        crate::check::check_nested_lambdas(&program.rules, &mut diags);
        diags
    }

    #[test]
    fn lambdas_side_by_side_are_fine() {
        let diags = nested_errors(|b| {
            let a = nums().filter(|e, _| e.gt(1));
            let c = nums().filter(|e, _| e.gt(2));
            b.wait(a.len() + c.len());
        });
        assert_eq!(diags.0, vec![]);
    }

    #[test]
    fn lambda_in_the_source_array_is_fine() {
        let diags = nested_errors(|b| {
            let inner = nums().filter(|e, _| e.gt(1));
            b.wait(inner.filter(|e, _| e.lt(3)).len());
        });
        assert_eq!(diags.0, vec![]);
    }

    #[test]
    fn nesting_is_reported_with_a_location() {
        let diags = nested_errors(|b| {
            let other = nums();
            let outer = nums().filter(|e, _| other.filter(|o, _| o.eq(e.clone())).len().gt(0));
            b.wait(outer.len());
        });
        assert_eq!(diags.0.len(), 1);
        assert_eq!(diags.0[0].code, "nested-lambda");
        assert!(diags.0[0].location.is_some());
    }
}
