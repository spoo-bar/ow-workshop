//! `Ray Cast Hit Position/Player/Normal`, which share their arguments.

use ow_workshop_data::ValueId;

use crate::expr::Expr;
use crate::generated::api::all_players;
use crate::ir::{self, ExprKind};
use crate::ty::{Array, Bool, Direction, IntoExpr, Player, Position};

/// Optional arguments of [`ray_cast_with`].
#[derive(Clone, Debug)]
pub struct RayCastArgs {
    /// Players the ray can hit; default: everyone.
    pub players_to_include: Expr<Array<Player>>,
    /// Players the ray passes through; default: nobody.
    pub players_to_exclude: Expr<Array<Player>>,
    /// Whether objects owned by players block the ray; default: yes.
    pub include_player_owned_objects: Expr<Bool>,
}

impl Default for RayCastArgs {
    fn default() -> Self {
        Self {
            players_to_include: all_players(),
            players_to_exclude: Vec::<Expr<Player>>::new().into(),
            include_player_owned_objects: true.into(),
        }
    }
}

/// A ray cast; read what it hit with `hit_position`, `hit_player` or `hit_normal`.
#[derive(Clone, Debug)]
pub struct RayCast {
    args: Vec<ir::Expr>,
}

/// Casts a ray from `start` to `end`.
#[track_caller]
#[must_use]
pub fn ray_cast(start: impl IntoExpr<Position>, end: impl IntoExpr<Position>) -> RayCast {
    ray_cast_with(start, end, RayCastArgs::default())
}

/// Casts a ray with explicit optional arguments.
#[track_caller]
#[must_use]
pub fn ray_cast_with(
    start: impl IntoExpr<Position>,
    end: impl IntoExpr<Position>,
    opts: RayCastArgs,
) -> RayCast {
    RayCast {
        args: vec![
            start.into_expr().into_ir(),
            end.into_expr().into_ir(),
            opts.players_to_include.into_ir(),
            opts.players_to_exclude.into_ir(),
            opts.include_player_owned_objects.into_ir(),
        ],
    }
}

impl RayCast {
    #[track_caller]
    fn hit<T>(&self, id: ValueId) -> Expr<T> {
        Expr::from_ir(ir::Expr::here(ExprKind::Value {
            id,
            args: self.args.clone(),
        }))
    }

    /// `Ray Cast Hit Position`.
    #[track_caller]
    #[must_use]
    pub fn hit_position(&self) -> Expr<Position> {
        self.hit(ValueId::RaycastHitPosition)
    }

    /// `Ray Cast Hit Player`.
    #[track_caller]
    #[must_use]
    pub fn hit_player(&self) -> Expr<Player> {
        self.hit(ValueId::RaycastHitPlayer)
    }

    /// `Ray Cast Hit Normal`.
    #[track_caller]
    #[must_use]
    pub fn hit_normal(&self) -> Expr<Direction> {
        self.hit(ValueId::RaycastHitNormal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::vect;

    #[test]
    fn hits_share_the_cast_arguments() {
        let cast = ray_cast(vect(0, 0, 0), vect(0, 0, 10));
        let ids_and_counts: Vec<_> = [
            cast.hit_position().erase(),
            cast.hit_player().erase(),
            cast.hit_normal().erase(),
        ]
        .iter()
        .map(|e| match e.ir().kind() {
            ExprKind::Value { id, args } => (*id, args.len()),
            other => panic!("unexpected {other:?}"),
        })
        .collect();
        assert_eq!(
            ids_and_counts,
            [
                (ValueId::RaycastHitPosition, 5),
                (ValueId::RaycastHitPlayer, 5),
                (ValueId::RaycastHitNormal, 5)
            ]
        );
    }
}
