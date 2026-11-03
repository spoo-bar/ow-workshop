//! Expansion of `#[derive(GlobalVars)]` and `#[derive(PlayerVars)]`.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Expr, ExprLit, Field, Fields, Lit, LitInt, LitStr, spanned::Spanned};

/// Which kind of variables a derive declares.
#[derive(Clone, Copy)]
pub enum Scope {
    Global,
    Player,
}

#[derive(Default)]
struct FieldAttrs {
    name: Option<LitStr>,
    index: Option<LitInt>,
    init: Option<Expr>,
}

pub fn expand(input: TokenStream, scope: Scope) -> TokenStream {
    match try_expand(input, scope) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error(),
    }
}

fn try_expand(input: TokenStream, scope: Scope) -> syn::Result<TokenStream> {
    let input: DeriveInput = syn::parse2(input)?;
    let Data::Struct(data) = &input.data else {
        return Err(syn::Error::new(input.span(), "expected a struct"));
    };
    let Fields::Named(fields) = &data.fields else {
        return Err(syn::Error::new(input.span(), "expected named fields"));
    };

    let (trait_path, handle_path) = match scope {
        Scope::Global => (
            quote!(::ow_workshop::GlobalVars),
            quote!(::ow_workshop::GlobalHandle),
        ),
        Scope::Player => (
            quote!(::ow_workshop::PlayerVars),
            quote!(::ow_workshop::PlayerHandle),
        ),
    };

    let mut inits = Vec::new();
    for field in &fields.named {
        let attrs = parse_attrs(field)?;
        let ident = field
            .ident
            .clone()
            .ok_or_else(|| syn::Error::new(field.span(), "unnamed field"))?;
        let ty = &field.ty;
        let name = attrs
            .name
            .map_or_else(|| ident.to_string(), |lit| lit.value());
        let index = if let Some(lit) = &attrs.index {
            let n: u8 = lit.base10_parse()?;
            quote!(::core::option::Option::Some(#n))
        } else {
            quote!(::core::option::Option::None)
        };
        let init = if let Some(expr) = &attrs.init {
            quote!(::core::option::Option::Some(::ow_workshop::IntoExpr::into_expr(#expr)))
        } else {
            quote!(::core::option::Option::None)
        };
        inits.push(quote! {
            #ident: <#ty as #handle_path>::declare(reg, #name, #index, #init)
        });
    }

    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    Ok(quote! {
        impl #impl_generics #trait_path for #ident #ty_generics #where_clause {
            fn declare(reg: &mut ::ow_workshop::VarRegistry) -> Self {
                Self { #(#inits),* }
            }
        }
    })
}

fn parse_attrs(field: &Field) -> syn::Result<FieldAttrs> {
    let mut attrs = FieldAttrs::default();
    for attr in field.attrs.iter().filter(|a| a.path().is_ident("ow")) {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("name") {
                attrs.name = Some(meta.value()?.parse()?);
            } else if meta.path.is_ident("index") {
                let expr: Expr = meta.value()?.parse()?;
                match expr {
                    Expr::Lit(ExprLit {
                        lit: Lit::Int(int), ..
                    }) => attrs.index = Some(int),
                    other => {
                        return Err(syn::Error::new(
                            other.span(),
                            "index must be an integer literal",
                        ));
                    }
                }
            } else if meta.path.is_ident("init") {
                attrs.init = Some(meta.value()?.parse()?);
            } else {
                return Err(meta.error("expected `name`, `index` or `init`"));
            }
            Ok(())
        })?;
    }
    Ok(attrs)
}
