//! `Block::columns`: assigns a table written as rows to one array per column.
//!
//! Rows read best in source, but one array per column is cheapest to store and index.

use crate::block::Block;
use crate::expr::Expr;
use crate::ir::{self, ExprKind, Location, Stmt, StmtKind};
use crate::ty::{Any, Array, IntoExpr, ValueType};
use crate::vars::{Global, Indexed, Indexed2, Indexed3, Place, PlayerVarRef, RawPlace};

/// An array variable that holds one column of a table.
pub trait ColumnPlace {
    /// The type of the column's cells.
    type Item: ValueType;

    #[doc(hidden)]
    fn into_column(self) -> RawPlace;
}

macro_rules! column_places {
    ($($place:ident),* $(,)?) => {$(
        impl<T: ValueType> ColumnPlace for $place<Array<T>> {
            type Item = T;

            fn into_column(self) -> RawPlace {
                self.into_place()
            }
        }
    )*};
}

column_places!(Global, PlayerVarRef, Indexed, Indexed2, Indexed3);

/// Places holding one column each, for rows with one cell per column.
pub trait Table<Row> {
    #[doc(hidden)]
    fn split(self, rows: Vec<Row>) -> Vec<(RawPlace, Vec<Expr<Any>>)>;
}

macro_rules! tables {
    ($( ( $(($P:ident, $R:ident, $i:tt)),+ ) )+) => {$(
        impl<$($P, $R),+> Table<($($R,)+)> for ($($P,)+)
        where
            $( $P: ColumnPlace, $R: IntoExpr<$P::Item> ),+
        {
            fn split(self, rows: Vec<($($R,)+)>) -> Vec<(RawPlace, Vec<Expr<Any>>)> {
                let mut columns = vec![$( (self.$i.into_column(), Vec::with_capacity(rows.len())) ),+];
                for row in rows {
                    $( columns[$i].1.push(row.$i.into_expr().erase()); )+
                }
                columns
            }
        }
    )+};
}

tables! {
    ((P0, R0, 0), (P1, R1, 1))
    ((P0, R0, 0), (P1, R1, 1), (P2, R2, 2))
    ((P0, R0, 0), (P1, R1, 1), (P2, R2, 2), (P3, R3, 3))
    ((P0, R0, 0), (P1, R1, 1), (P2, R2, 2), (P3, R3, 3), (P4, R4, 4))
    ((P0, R0, 0), (P1, R1, 1), (P2, R2, 2), (P3, R3, 3), (P4, R4, 4), (P5, R5, 5))
    ((P0, R0, 0), (P1, R1, 1), (P2, R2, 2), (P3, R3, 3), (P4, R4, 4), (P5, R5, 5), (P6, R6, 6))
    ((P0, R0, 0), (P1, R1, 1), (P2, R2, 2), (P3, R3, 3), (P4, R4, 4), (P5, R5, 5), (P6, R6, 6), (P7, R7, 7))
}

/// A table assignment being built; it is recorded when dropped.
#[derive(Debug)]
pub struct Columns<'a> {
    block: &'a mut Block,
    loc: Location,
    cells: Vec<(RawPlace, Vec<ir::Expr>)>,
    compressed: bool,
}

// The builder is recorded on drop, so the chain needs no `#[must_use]`.
#[allow(clippy::return_self_not_must_use)]
impl Columns<'_> {
    /// Stores every column made only of number or vector literals as a compressed string
    /// (see [`compressed`](crate::compressed)); other columns, or ones that do not fit, stay
    /// plain arrays.
    pub fn compressed(mut self) -> Self {
        self.compressed = true;
        self
    }
}

impl Drop for Columns<'_> {
    fn drop(&mut self) {
        for (RawPlace(target), items) in std::mem::take(&mut self.cells) {
            let packed = self
                .compressed
                .then(|| crate::compress::compress_items(&items, self.loc))
                .flatten();
            let value =
                packed.unwrap_or_else(|| ir::Expr::new(ExprKind::Array(items), Some(self.loc)));
            self.block.ir.stmts.push(Stmt {
                kind: StmtKind::Set { target, value },
                loc: Some(self.loc),
                comment: None,
                suppress: self.block.suppressed,
            });
        }
    }
}

impl Block {
    /// Assigns a table written as rows: one `Set` per column, each an array of that column.
    ///
    /// `places` is a tuple of 2 to 8 array variables and every row a tuple of one cell per
    /// column; a cell is a value or an `Option` (`None` is `Null`). Chain `.compressed()` to
    /// store literal columns as compressed strings.
    #[track_caller]
    pub fn columns<P: Table<R>, R>(
        &mut self,
        places: P,
        rows: impl IntoIterator<Item = R>,
    ) -> Columns<'_> {
        Columns {
            loc: Location::caller(),
            cells: places
                .split(rows.into_iter().collect())
                .into_iter()
                .map(|(place, items)| (place, items.into_iter().map(Expr::into_ir).collect()))
                .collect(),
            compressed: false,
            block: self,
        }
    }
}

#[cfg(test)]
mod tests;
