//! Derive macros for `ow-workshop`.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as Tokens;
use quote::quote;
use syn::{
    Attribute, Data, DeriveInput, Fields, Ident, LitBool, LitInt, LitStr, parse_macro_input,
};

/// Implements `Globals` for a struct whose fields are `Global`.
///
/// Field attributes: `#[var(name = "NAME", index = N)]` and `#[debug(...)]`.
///
/// Indices start at 0 in declaration order. `index` pins a variable; the others
/// fill the lowest free indices. An index must be unique and at most 127.
#[proc_macro_derive(Globals, attributes(var, debug))]
pub fn globals(input: TokenStream) -> TokenStream {
    variables(&parse_macro_input!(input), "Globals", "Global")
}

/// Implements `Players` for a struct whose fields are `PlayerVar`.
///
/// Field attributes: `#[var(name = "NAME", index = N)]` and `#[debug(...)]`.
///
/// Indices start at 0 in declaration order. `index` pins a variable; the others
/// fill the lowest free indices. An index must be unique and at most 127.
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

    let mut specs = Vec::new();
    for field in &fields.named {
        let ident = field.ident.as_ref().expect("named field");
        let attr = var_attr(&field.attrs)?;
        let var_name = attr.name.unwrap_or_else(|| ident.to_string());
        specs.push((ident, var_name, attr.index, debug_opts(&field.attrs)?));
    }
    let indices = allocate(&specs.iter().map(|spec| spec.2.clone()).collect::<Vec<_>>())?;

    let mut idents = Vec::new();
    let mut inits = Vec::new();
    for ((ident, var_name, _, debug), index) in specs.iter().zip(indices) {
        inits.push(quote! {
            #ident: ::ow_workshop::#handle { name: #var_name, index: #index, debug: #debug }
        });
        idents.push(*ident);
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

struct VarAttr {
    name: Option<String>,
    index: Option<LitInt>,
}

fn var_attr(attrs: &[Attribute]) -> syn::Result<VarAttr> {
    let mut found = VarAttr {
        name: None,
        index: None,
    };
    for attr in attrs.iter().filter(|attr| attr.path().is_ident("var")) {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("name") {
                found.name = Some(meta.value()?.parse::<LitStr>()?.value());
                Ok(())
            } else if meta.path.is_ident("index") {
                found.index = Some(meta.value()?.parse::<LitInt>()?);
                Ok(())
            } else {
                Err(meta.error("expected `name = \"...\"` or `index = N`"))
            }
        })?;
    }
    Ok(found)
}

/// The index of every variable: pinned ones as written, the rest in the lowest free slots.
fn allocate(pins: &[Option<LitInt>]) -> syn::Result<Vec<u8>> {
    const MAX: u8 = 127;
    let mut taken = [false; MAX as usize + 1];
    let mut pinned = Vec::new();
    for pin in pins {
        let Some(lit) = pin else {
            pinned.push(None);
            continue;
        };
        let index: u8 = lit
            .base10_parse()
            .ok()
            .filter(|index| *index <= MAX)
            .ok_or_else(|| syn::Error::new(lit.span(), "variable index must be at most 127"))?;
        if std::mem::replace(&mut taken[usize::from(index)], true) {
            return Err(syn::Error::new(
                lit.span(),
                format!("variable index {index} is used twice"),
            ));
        }
        pinned.push(Some(index));
    }
    let mut free = (0..=MAX).filter(|index| !taken[usize::from(*index)]);
    let mut out = Vec::new();
    for pin in pinned {
        out.push(match pin {
            Some(index) => index,
            None => free.next().ok_or_else(|| {
                syn::Error::new(
                    proc_macro2::Span::call_site(),
                    "more than 128 variables; the workshop allows 128 of each kind",
                )
            })?,
        });
    }
    Ok(out)
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
