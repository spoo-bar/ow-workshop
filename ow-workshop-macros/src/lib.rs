//! Derive macros for `ow-workshop`.

mod vars;

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
