//! Expansion of `#[derive(WorkshopEnum)]`.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Expr, Fields, Path, spanned::Spanned};

pub fn expand(input: TokenStream) -> TokenStream {
    match try_expand(input) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error(),
    }
}

fn enum_type(input: &DeriveInput) -> syn::Result<Option<Path>> {
    let mut ty = None;
    for attr in input.attrs.iter().filter(|a| a.path().is_ident("ow")) {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("ty") {
                ty = Some(meta.value()?.parse()?);
                Ok(())
            } else {
                Err(meta.error("expected `ty`"))
            }
        })?;
    }
    Ok(ty)
}

fn variant_value(variant: &syn::Variant) -> syn::Result<Option<Expr>> {
    let mut value = None;
    for attr in variant.attrs.iter().filter(|a| a.path().is_ident("ow")) {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("value") {
                value = Some(meta.value()?.parse()?);
                Ok(())
            } else {
                Err(meta.error("expected `value`"))
            }
        })?;
    }
    Ok(value)
}

fn try_expand(input: TokenStream) -> syn::Result<TokenStream> {
    let input: DeriveInput = syn::parse2(input)?;
    let Data::Enum(data) = &input.data else {
        return Err(syn::Error::new(input.span(), "expected an enum"));
    };
    if let Some(variant) = data
        .variants
        .iter()
        .find(|v| !matches!(v.fields, Fields::Unit))
    {
        return Err(syn::Error::new(
            variant.span(),
            "variants must not have fields",
        ));
    }
    let ty = enum_type(&input)?;
    let values = data
        .variants
        .iter()
        .map(variant_value)
        .collect::<syn::Result<Vec<_>>>()?;
    let with_value = values.iter().filter(|v| v.is_some()).count();
    if with_value != 0 && with_value != values.len() {
        return Err(syn::Error::new(
            input.span(),
            "either every variant or none must have `#[ow(value = ..)]`",
        ));
    }
    if with_value != 0 && ty.is_none() {
        return Err(syn::Error::new(
            input.span(),
            "`#[ow(value = ..)]` needs `#[ow(ty = Type)]` on the enum",
        ));
    }

    let name = &input.ident;
    let names: Vec<_> = data.variants.iter().map(|v| &v.ident).collect();
    let count = names.len();
    let target = ty.map_or_else(|| quote!(::ow_workshop::Num), |t| quote!(#t));

    let conversion = if with_value == 0 {
        // Discriminants follow Rust's rules: explicit, or the previous one plus one.
        quote! { ::ow_workshop::IntoExpr::<::ow_workshop::Num>::into_expr(value as i32) }
    } else {
        let arms = names.iter().zip(&values).map(|(variant, value)| {
            quote! { #name::#variant => ::ow_workshop::IntoExpr::<#target>::into_expr(#value) }
        });
        quote! { match value { #(#arms),* } }
    };

    // The names are indexed directly when the values are 0, 1, 2, ..; otherwise the value's
    // position among the variants' values picks the name.
    let by_position = quote! {
        names.get(<#name>::values().index_of(value))
    };
    let render = if with_value == 0 {
        quote! {
            let contiguous = [#(#name::#names as i64),*]
                .iter()
                .zip(0_i64..)
                .all(|(value, position)| *value == position);
            if contiguous {
                names.get(value)
            } else {
                #by_position
            }
        }
    } else {
        by_position
    };

    Ok(quote! {
        impl ::core::convert::From<#name> for ::ow_workshop::Expr<#target> {
            fn from(value: #name) -> Self {
                #conversion
            }
        }

        impl ::ow_workshop::IntoExpr<#target> for #name {
            fn into_expr(self) -> ::ow_workshop::Expr<#target> {
                ::core::convert::From::from(self)
            }
        }

        impl ::ow_workshop::DebugRender for #name {
            type Value = #target;

            fn debug_render(
                value: ::ow_workshop::Expr<#target>,
            ) -> ::ow_workshop::Expr<::ow_workshop::Str> {
                let names: ::ow_workshop::Expr<::ow_workshop::Array<::ow_workshop::Str>> =
                    ::ow_workshop::IntoExpr::into_expr([#(::core::stringify!(#names)),*]);
                #render
            }
        }

        impl #name {
            /// Number of variants.
            pub const COUNT: usize = #count;

            /// Every variant's value, in declaration order.
            pub fn values() -> ::ow_workshop::Expr<::ow_workshop::Array<#target>> {
                ::ow_workshop::IntoExpr::into_expr([#(#name::#names),*])
            }
        }
    })
}
