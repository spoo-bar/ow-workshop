//! Typed handles into the variable, subroutine and label tables.

macro_rules! id {
    ($(#[$doc:meta])* $name:ident) => {
        $(#[$doc])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub(crate) struct $name(pub(crate) u32);
    };
}

/// Handle of a declared global or player variable.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VarId(pub(crate) u32);

id!(/// Handle of a declared subroutine.
    SubId);
id!(/// Handle of a jump label.
    LabelId);
