//! Derive macros for `ow-workshop`.

mod fmt;
mod vars;
mod workshop_enum;

use proc_macro::TokenStream;

/// Declares a struct of global variable handles (`Global<T>` fields).
///
/// Field attributes: `#[ow(name = "X")]`, `#[ow(index = N)]`, `#[ow(init = expr)]`.
#[proc_macro_derive(GlobalVars, attributes(ow))]
pub fn derive_global_vars(input: TokenStream) -> TokenStream {
    vars::expand(input.into(), vars::Scope::Global).into()
}

/// Declares a struct of player variable handles (`PlayerVar<T>` fields).
///
/// Field attributes: `#[ow(name = "X")]`, `#[ow(index = N)]`, `#[ow(init = expr)]`.
#[proc_macro_derive(PlayerVars, attributes(ow))]
pub fn derive_player_vars(input: TokenStream) -> TokenStream {
    vars::expand(input.into(), vars::Scope::Player).into()
}

/// Turns a fieldless enum into workshop constants, inlined at every use.
///
/// Values follow the Rust discriminants. `#[ow(value = expr)]` on every variant together with
/// `#[ow(ty = Type)]` on the enum gives non-numeric values. Also derive `Clone` and `Copy`.
#[proc_macro_derive(WorkshopEnum, attributes(ow))]
pub fn derive_workshop_enum(input: TokenStream) -> TokenStream {
    workshop_enum::expand(input.into()).into()
}

/// A `format!`-like workshop string: `ws_fmt!("Score: {} / {}", a, b)`.
///
/// Supports `{}`, `{0}` (not together), `{name}` (an argument `name = expr` or an identifier in
/// scope) and `{{` / `}}`. The result is an `Expr<Str>` of any length; the compiler splits it into
/// nested custom strings.
#[proc_macro]
pub fn ws_fmt(input: TokenStream) -> TokenStream {
    fmt::expand(input.into()).into()
}
