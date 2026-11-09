//! `reversed` and `unique` for arrays.

use crate::expr::Expr;
use crate::ty::{Array, Is, Num, ValueType};

impl<T: ValueType> Expr<Array<T>> {
    /// The elements in reverse order: `Sorted Array(arr, -1 * Current Array Index)`.
    #[track_caller]
    #[must_use]
    pub fn reversed(&self) -> Expr<Array<T>> {
        self.sorted_by(|_, index| Expr::<Num>::from(-1) * index)
    }

    /// The first occurrence of each element, in order.
    #[track_caller]
    #[must_use]
    pub fn unique(&self) -> Expr<Array<T>>
    where
        T: Is<T>,
    {
        self.filter(|elem, index| self.index_of(elem).eq(index))
    }
}

#[cfg(test)]
mod tests {
    use crate::expr::Expr;
    use crate::opt::testutil::sh;
    use crate::ty::{Array, Num};

    #[test]
    fn reversed_sorts_by_the_negated_index() {
        let arr: Expr<Array<Num>> = Expr::from([1, 2, 3].map(Expr::<Num>::from));
        assert_eq!(sh(arr.reversed().ir()), "SortedArray([1, 2, 3], (-1 * #))");
    }

    #[test]
    fn unique_keeps_the_first_occurrence() {
        let arr: Expr<Array<Num>> = Expr::from([1, 1, 2].map(Expr::<Num>::from));
        assert_eq!(
            sh(arr.unique().ir()),
            "FilteredArray([1, 1, 2], (Index([1, 1, 2], @) == #))"
        );
    }
}
