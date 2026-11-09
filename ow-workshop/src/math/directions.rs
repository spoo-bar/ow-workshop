//! The six unit directions as constructors on [`Vector`].

use crate::expr::Expr;
use crate::generated::api::{
    vector_backward, vector_down, vector_forward, vector_left, vector_right, vector_up,
};
use crate::ty::{Direction, Vector};

impl Vector {
    /// `Up`.
    #[track_caller]
    #[must_use]
    pub fn up() -> Expr<Direction> {
        vector_up()
    }

    /// `Down`.
    #[track_caller]
    #[must_use]
    pub fn down() -> Expr<Direction> {
        vector_down()
    }

    /// `Left`.
    #[track_caller]
    #[must_use]
    pub fn left() -> Expr<Direction> {
        vector_left()
    }

    /// `Right`.
    #[track_caller]
    #[must_use]
    pub fn right() -> Expr<Direction> {
        vector_right()
    }

    /// `Forward`.
    #[track_caller]
    #[must_use]
    pub fn forward() -> Expr<Direction> {
        vector_forward()
    }

    /// `Backward`.
    #[track_caller]
    #[must_use]
    pub fn backward() -> Expr<Direction> {
        vector_backward()
    }
}

#[cfg(test)]
mod tests {
    use ow_workshop_data::ValueId;

    use super::*;
    use crate::ir::ExprKind;

    #[test]
    fn map_to_the_direction_values() {
        for (e, expected) in [
            (Vector::up(), ValueId::VectorUp),
            (Vector::down(), ValueId::VectorDown),
            (Vector::left(), ValueId::VectorLeft),
            (Vector::right(), ValueId::VectorRight),
            (Vector::forward(), ValueId::VectorForward),
            (Vector::backward(), ValueId::VectorBackward),
        ] {
            assert!(
                matches!(e.ir().kind(), ExprKind::Value { id, args } if *id == expected && args.is_empty())
            );
        }
    }
}
