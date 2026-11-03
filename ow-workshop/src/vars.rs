//! Variable handles, the registry the derives declare into, and places.

use std::marker::PhantomData;

use crate::expr::Expr;
use crate::ir::{self, ExprKind, VarDecl, VarId, Vars};
use crate::ty::{Array, IntoExpr, Is, Num, Player, ValueType};

/// Collects variable declarations while the handles are created.
#[derive(Debug, Default)]
pub struct VarRegistry {
    pub(crate) vars: Vars,
}

impl VarRegistry {
    /// Declares a variable; used by the derives.
    #[track_caller]
    fn declare<T: ValueType>(
        &mut self,
        name: &str,
        index: Option<u8>,
        init: Option<Expr<T>>,
    ) -> VarId {
        let mut decl = VarDecl::new(name);
        decl.index = index;
        decl.init = init.map(Expr::into_ir);
        self.vars.declare(decl)
    }
}

/// A struct of global variable handles.
pub trait GlobalVars: Sized {
    /// Registers the variables and returns their handles.
    fn declare(reg: &mut VarRegistry) -> Self;
}

/// A struct of player variable handles.
pub trait PlayerVars: Sized {
    /// Registers the variables and returns their handles.
    fn declare(reg: &mut VarRegistry) -> Self;
}

impl GlobalVars for () {
    fn declare(_: &mut VarRegistry) -> Self {}
}

impl PlayerVars for () {
    fn declare(_: &mut VarRegistry) -> Self {}
}

/// A field type of a `#[derive(GlobalVars)]` struct.
pub trait GlobalHandle: Sized {
    /// The value type of the variable.
    type Value: ValueType;
    /// Declares the variable and returns its handle.
    #[track_caller]
    fn declare(
        reg: &mut VarRegistry,
        name: &str,
        index: Option<u8>,
        init: Option<Expr<Self::Value>>,
    ) -> Self;
}

/// A field type of a `#[derive(PlayerVars)]` struct.
pub trait PlayerHandle: Sized {
    /// The value type of the variable.
    type Value: ValueType;
    /// Declares the variable and returns its handle.
    #[track_caller]
    fn declare(
        reg: &mut VarRegistry,
        name: &str,
        index: Option<u8>,
        init: Option<Expr<Self::Value>>,
    ) -> Self;
}

/// A global variable holding a `T`.
pub struct Global<T> {
    id: VarId,
    _type: PhantomData<fn() -> T>,
}

/// A player variable holding a `T`; read or write it through [`Expr::var`].
pub struct PlayerVar<T> {
    id: VarId,
    _type: PhantomData<fn() -> T>,
}

macro_rules! handle {
    ($name:ident) => {
        impl<T> Clone for $name<T> {
            fn clone(&self) -> Self {
                *self
            }
        }
        impl<T> Copy for $name<T> {}
        impl<T> std::fmt::Debug for $name<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple(stringify!($name)).field(&self.id).finish()
            }
        }
    };
}

handle!(Global);
handle!(PlayerVar);

impl<T: ValueType> GlobalHandle for Global<T> {
    type Value = T;
    #[track_caller]
    fn declare(
        reg: &mut VarRegistry,
        name: &str,
        index: Option<u8>,
        init: Option<Expr<T>>,
    ) -> Self {
        Self {
            id: reg.declare(name, index, init),
            _type: PhantomData,
        }
    }
}

impl<T: ValueType> PlayerHandle for PlayerVar<T> {
    type Value = T;
    #[track_caller]
    fn declare(
        reg: &mut VarRegistry,
        name: &str,
        index: Option<u8>,
        init: Option<Expr<T>>,
    ) -> Self {
        Self {
            id: reg.declare(name, index, init),
            _type: PhantomData,
        }
    }
}

impl<T: ValueType> Global<T> {
    /// The variable's current value.
    #[track_caller]
    #[must_use]
    pub fn get(self) -> Expr<T> {
        Expr::from_ir(ir::Expr::here(ExprKind::GlobalVar(self.id)))
    }
}

impl<S: Is<T>, T: ValueType> IntoExpr<T> for Global<S> {
    #[track_caller]
    fn into_expr(self) -> Expr<T> {
        Expr::from_ir(self.get().into_ir())
    }
}

/// A player variable of one player.
pub struct PlayerVarRef<T> {
    player: ir::Expr,
    var: VarId,
    _type: PhantomData<fn() -> T>,
}

impl<T> Clone for PlayerVarRef<T> {
    fn clone(&self) -> Self {
        Self {
            player: self.player.clone(),
            var: self.var,
            _type: PhantomData,
        }
    }
}

impl<T> std::fmt::Debug for PlayerVarRef<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PlayerVarRef")
            .field("var", &self.var)
            .finish_non_exhaustive()
    }
}

impl Expr<Player> {
    /// The player variable `var` of this player.
    #[must_use]
    pub fn var<T: ValueType>(&self, var: PlayerVar<T>) -> PlayerVarRef<T> {
        PlayerVarRef {
            player: self.ir().clone(),
            var: var.id,
            _type: PhantomData,
        }
    }
}

impl<T: ValueType> PlayerVarRef<T> {
    /// The variable's current value.
    #[track_caller]
    #[must_use]
    pub fn get(&self) -> Expr<T> {
        Expr::from_ir(ir::Expr::here(ExprKind::PlayerVar {
            player: self.player.clone(),
            var: self.var,
        }))
    }
}

impl<S: Is<T>, T: ValueType> IntoExpr<T> for PlayerVarRef<S> {
    #[track_caller]
    fn into_expr(self) -> Expr<T> {
        Expr::from_ir(self.get().into_ir())
    }
}

impl<S: Is<T>, T: ValueType> IntoExpr<T> for &PlayerVarRef<S> {
    #[track_caller]
    fn into_expr(self) -> Expr<T> {
        Expr::from_ir(self.get().into_ir())
    }
}

/// An element of an array variable: `list.at(i)`.
pub struct Indexed<T> {
    base: ir::Place,
    _type: PhantomData<fn() -> T>,
}

impl<T> Clone for Indexed<T> {
    fn clone(&self) -> Self {
        Self {
            base: self.base.clone(),
            _type: PhantomData,
        }
    }
}

impl<T> std::fmt::Debug for Indexed<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Indexed").field(&self.base).finish()
    }
}

impl<T: ValueType> Indexed<T> {
    /// The element's current value (`Value In Array`).
    #[track_caller]
    #[must_use]
    pub fn get(&self) -> Expr<T> {
        Expr::from_ir(place_expr(&self.base))
    }
}

impl<S: Is<T>, T: ValueType> IntoExpr<T> for Indexed<S> {
    #[track_caller]
    fn into_expr(self) -> Expr<T> {
        Expr::from_ir(self.get().into_ir())
    }
}

/// Reads a place as an expression.
#[track_caller]
fn place_expr(place: &ir::Place) -> ir::Expr {
    let kind = match place {
        ir::Place::Global(id) => ExprKind::GlobalVar(*id),
        ir::Place::Player { player, var } => ExprKind::PlayerVar {
            player: player.clone(),
            var: *var,
        },
        ir::Place::Index { base, index } => ExprKind::Index {
            array: place_expr(base),
            index: index.clone(),
        },
    };
    ir::Expr::here(kind)
}

/// An opaque untyped place.
#[derive(Clone, Debug)]
pub struct RawPlace(pub(crate) ir::Place);

/// Something a `T` can be stored in: a variable or an element of one.
pub trait Place<T: ValueType> {
    /// The untyped place.
    fn into_place(self) -> RawPlace;
}

/// A plain global or player variable: the only places that can be chased or used as loop counters.
pub trait Plain<T: ValueType>: Place<T> {}

impl<T: ValueType> Place<T> for Global<T> {
    fn into_place(self) -> RawPlace {
        RawPlace(ir::Place::Global(self.id))
    }
}

impl<T: ValueType> Plain<T> for Global<T> {}

impl<T: ValueType> Place<T> for PlayerVarRef<T> {
    fn into_place(self) -> RawPlace {
        RawPlace(ir::Place::Player {
            player: self.player,
            var: self.var,
        })
    }
}

impl<T: ValueType> Plain<T> for PlayerVarRef<T> {}

impl<T: ValueType> Place<T> for Indexed<T> {
    fn into_place(self) -> RawPlace {
        RawPlace(self.base)
    }
}

/// Places that hold an array and can be indexed.
trait ArrayPlace<E>: Sized {
    fn index_place(self, index: ir::Expr) -> Indexed<E>;
}

impl<E> ArrayPlace<E> for ir::Place {
    fn index_place(self, index: ir::Expr) -> Indexed<E> {
        Indexed {
            base: ir::Place::Index {
                base: Box::new(self),
                index,
            },
            _type: PhantomData,
        }
    }
}

impl<E: ValueType> Global<Array<E>> {
    /// The element at `index`.
    #[must_use]
    pub fn at(self, index: impl IntoExpr<Num>) -> Indexed<E> {
        ir::Place::Global(self.id).index_place(index.into_expr().into_ir())
    }
}

impl<E: ValueType> PlayerVarRef<Array<E>> {
    /// The element at `index`.
    #[must_use]
    pub fn at(&self, index: impl IntoExpr<Num>) -> Indexed<E> {
        let place = ir::Place::Player {
            player: self.player.clone(),
            var: self.var,
        };
        place.index_place(index.into_expr().into_ir())
    }
}
