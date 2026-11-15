//! Derive macros for `ow-workshop`.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as Tokens;
use quote::quote;
use syn::{
    Attribute, Data, DeriveInput, Fields, Ident, LitBool, LitInt, LitStr, parse_macro_input,
};

/// Implements `Globals` for a struct whose fields are `Global`.
///
/// Field attributes: `#[var(name = "NAME")]` and `#[debug(...)]`.
#[proc_macro_derive(Globals, attributes(var, debug))]
pub fn globals(input: TokenStream) -> TokenStream {
    variables(&parse_macro_input!(input), "Globals", "Global")
}

/// Implements `Players` for a struct whose fields are `PlayerVar`.
///
/// Field attributes: `#[var(name = "NAME")]` and `#[debug(...)]`.
#[proc_macro_derive(Players, attributes(var, debug))]
pub fn players(input: TokenStream) -> TokenStream {
    variables(&parse_macro_input!(input), "Players", "PlayerVar")
}

/// Implements `WorkshopDebug`.
///
/// `#[debug(hud = Left, label = "..", sort_order = 1, log_to_inspector)]` turns
/// the debug display on. Without it the item has none.
#[proc_macro_derive(WorkshopDebug, attributes(debug))]
pub fn workshop_debug(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    let opts = match debug_opts(&input.attrs) {
        Ok(opts) => opts,
        Err(error) => return error.into_compile_error().into(),
    };
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    quote! {
        impl #impl_generics ::ow_workshop::WorkshopDebug for #name #ty_generics #where_clause {
            fn debug_opts(&self) -> ::core::option::Option<::ow_workshop::DebugOpts> {
                #opts
            }
        }
    }
    .into()
}

fn variables(input: &DeriveInput, trait_name: &str, handle: &str) -> TokenStream {
    expand_variables(input, trait_name, handle)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn expand_variables(input: &DeriveInput, trait_name: &str, handle: &str) -> syn::Result<Tokens> {
    let Data::Struct(data) = &input.data else {
        return Err(syn::Error::new_spanned(input, "expected a struct"));
    };
    let Fields::Named(fields) = &data.fields else {
        return Err(syn::Error::new_spanned(input, "expected named fields"));
    };
    let trait_name = Ident::new(trait_name, proc_macro2::Span::call_site());
    let handle = Ident::new(handle, proc_macro2::Span::call_site());
    let name = &input.ident;

    let mut idents = Vec::new();
    let mut inits = Vec::new();
    for field in &fields.named {
        let ident = field.ident.as_ref().expect("named field");
        let var_name = var_name(&field.attrs)?.unwrap_or_else(|| ident.to_string());
        let debug = debug_opts(&field.attrs)?;
        inits.push(quote! { #ident: ::ow_workshop::#handle { name: #var_name, debug: #debug } });
        idents.push(ident);
    }
    Ok(quote! {
        impl ::ow_workshop::#trait_name for #name {
            fn declare() -> Self {
                Self { #(#inits),* }
            }
            fn vars(&self) -> ::std::vec::Vec<::ow_workshop::#handle> {
                ::std::vec![#(self.#idents.clone()),*]
            }
        }
    })
}

fn var_name(attrs: &[Attribute]) -> syn::Result<Option<String>> {
    let mut name = None;
    for attr in attrs.iter().filter(|attr| attr.path().is_ident("var")) {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("name") {
                name = Some(meta.value()?.parse::<LitStr>()?.value());
                Ok(())
            } else {
                Err(meta.error("expected `name = \"...\"`"))
            }
        })?;
    }
    Ok(name)
}

/// The `Option<DebugOpts>` expression for an item's `#[debug(...)]` attribute.
fn debug_opts(attrs: &[Attribute]) -> syn::Result<Tokens> {
    let Some(attr) = attrs.iter().find(|attr| attr.path().is_ident("debug")) else {
        return Ok(quote! { ::core::option::Option::None });
    };
    let mut label = quote! { ::core::option::Option::None };
    let mut hud = quote! { ::ow_workshop::HudLocation::Left };
    let mut sort_order = quote! { 0 };
    let mut log_to_inspector = false;
    if !matches!(attr.meta, syn::Meta::Path(_)) {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("label") {
                let text: LitStr = meta.value()?.parse()?;
                label = quote! { ::core::option::Option::Some(::std::string::String::from(#text)) };
            } else if meta.path.is_ident("hud") {
                let location: Ident = meta.value()?.parse()?;
                hud = quote! { ::ow_workshop::HudLocation::#location };
            } else if meta.path.is_ident("sort_order") {
                let order: LitInt = meta.value()?.parse()?;
                sort_order = quote! { #order };
            } else if meta.path.is_ident("log_to_inspector") {
                log_to_inspector = match meta.value() {
                    Ok(value) => value.parse::<LitBool>()?.value,
                    Err(_) => true,
                };
            } else {
                return Err(
                    meta.error("expected `label`, `hud`, `sort_order` or `log_to_inspector`")
                );
            }
            Ok(())
        })?;
    }
    Ok(quote! {
        ::core::option::Option::Some(::ow_workshop::DebugOpts {
            label: #label,
            hud: #hud,
            sort_order: #sort_order,
            log_to_inspector: #log_to_inspector,
        })
    })
}
