//! `ow-workshop/src/generated/{values,actions}.rs`: the typed values and actions.

use std::collections::{BTreeMap, BTreeSet};

use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};
use serde::Deserialize;

use super::naming::pascal;
use crate::data::Data;
use crate::data::model::{Default as Dflt, Function, Ty};

const KEYWORDS: [&str; 53] = [
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "typeof", "unsized", "virtual", "yield", "try", "gen", "union",
];

/// `renames.toml`.
#[derive(Default, Deserialize)]
pub struct Renames {
    #[serde(default)]
    values: BTreeMap<String, String>,
    #[serde(default)]
    actions: BTreeMap<String, String>,
    #[serde(default)]
    skip: Skip,
}

#[derive(Default, Deserialize)]
struct Skip {
    #[serde(default)]
    actions: Vec<String>,
    #[serde(default)]
    values: Vec<String>,
}

impl Renames {
    pub fn parse(text: &str) -> Result<Self, String> {
        toml::from_str(text).map_err(|e| format!("renames.toml: {e}"))
    }
}

/// Lower snake case of a camelCase, `UPPER_SNAKE` or dotted key.
fn snake(key: &str) -> String {
    let chars: Vec<char> = key.chars().collect();
    let mut out = String::new();
    for (i, &c) in chars.iter().enumerate() {
        if matches!(c, '.' | ' ' | '-') {
            out.push('_');
        } else if c.is_uppercase() {
            let prev = i.checked_sub(1).map(|j| chars[j]);
            let next = chars.get(i + 1).copied();
            let boundary = prev.is_some_and(|p| {
                p.is_lowercase()
                    || p.is_ascii_digit()
                    || (p.is_uppercase() && next.is_some_and(char::is_lowercase))
            });
            if boundary && !out.ends_with('_') {
                out.push('_');
            }
            out.extend(c.to_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

fn escape(name: String) -> String {
    if KEYWORDS.contains(&name.as_str()) {
        format!("{name}_")
    } else {
        name
    }
}

/// A key for a type, normalized so unions are order-independent.
fn ty_key(ty: &Ty) -> String {
    match ty {
        Ty::Named(name) => name.clone(),
        Ty::Array { array } => format!("Array<{}>", ty_key(array)),
        Ty::Union(members) => {
            let mut keys: Vec<_> = members.iter().map(ty_key).collect();
            keys.sort();
            keys.join("|")
        }
        Ty::Vector(v) => {
            let name = if v.vector.is_some() {
                "Vector"
            } else if v.direction.is_some() {
                "Direction"
            } else if v.position.is_some() {
                "Position"
            } else {
                "Velocity"
            };
            name.to_owned()
        }
    }
}

/// The marker type of a signature type.
fn marker(key: &str, data: &Data) -> Result<TokenStream, String> {
    let tokens = match key {
        "float"
        | "unsigned float"
        | "int"
        | "unsigned int"
        | "signed int"
        | "signed float"
        | "IntLiteral"
        | "FloatLiteral"
        | "UnsignedIntLiteral"
        | "SignedIntLiteral"
        | "UnsignedFloatLiteral"
        | "SignedFloatLiteral" => quote!(Num),
        "bool" | "BoolLiteral" => quote!(Bool),
        "String" | "CustomStringLiteral" | "LocalizedStringLiteral" | "StringLiteral" => {
            quote!(Str)
        }
        "Object" | "Array|Object" | "Array<Object>|Object" => quote!(Any),
        "Array" | "Array<Object>" => quote!(Array<Any>),
        "Array<Player>" => quote!(Array<Player>),
        "Array<Hero>" => quote!(Array<Hero>),
        "Array<Position>" => quote!(Array<Position>),
        "Array<String>" | "Array<CustomStringLiteral>" => quote!(Array<Str>),
        "Array<Player>|Player" => quote!(Players),
        "Array<Hero>|Hero" => quote!(Heroes),
        "Player|Position" => quote!(PlayerOrPosition),
        "EntityId|Player" => quote!(EntityOrPlayer),
        "DamageModificationId" => quote!(DamageModId),
        "HealingModificationId" => quote!(HealingModId),
        "HeroLiteral" => quote!(Hero),
        "Player" | "Position" | "Direction" | "Vector" | "Velocity" | "EntityId" | "TextId"
        | "HealthPoolId" | "AssistId" | "DotId" | "HotId" | "Hero" | "Map" | "Gamemode" => {
            let ident = format_ident!("{key}");
            quote!(#ident)
        }
        _ if data.constants.contains_key(key) => {
            let ident = format_ident!("{key}");
            quote!(#ident)
        }
        _ => return Err(format!("no marker type for `{key}`")),
    };
    Ok(tokens)
}

/// The marker of a return type; unions are untyped.
fn ret_marker(key: &str, data: &Data) -> Result<TokenStream, String> {
    if key.contains('|') && key != "Array<Player>|Player" {
        return Ok(quote!(Any));
    }
    if key.contains('|') {
        return Ok(quote!(Any));
    }
    marker(key, data)
}

fn ir_default(default: &Dflt) -> Result<TokenStream, String> {
    Ok(match default {
        Dflt::Null(_) => quote!(support::null()),
        Dflt::Num { num } => {
            let n = Literal::f64_suffixed(*num);
            quote!(support::num(#n))
        }
        Dflt::Bool { bool } => quote!(support::boolean(#bool)),
        Dflt::Const { r#const: (e, k) } => quote!(support::constant(#e, #k)),
        Dflt::Value { value, args } => {
            let id = format_ident!("{}", pascal(value)?);
            let args = args.iter().map(ir_default).collect::<Result<Vec<_>, _>>()?;
            quote!(support::value_ir(ValueId::#id, vec![#(#args),*]))
        }
        Dflt::Vector { vector: [x, y, z] } => {
            let (x, y, z) = (
                Literal::f64_suffixed(*x),
                Literal::f64_suffixed(*y),
                Literal::f64_suffixed(*z),
            );
            quote!(support::vector([#x, #y, #z]))
        }
    })
}

struct Param {
    ident: proc_macro2::Ident,
    ty: TokenStream,
    default: Option<TokenStream>,
}

struct Item<'a> {
    f: &'a Function,
    name: String,
    module: Option<String>,
    receiver: Option<&'static str>,
    params: Vec<Param>,
    ret: TokenStream,
}

impl Item<'_> {
    fn required(&self) -> impl Iterator<Item = &Param> {
        self.params.iter().filter(|p| p.default.is_none())
    }

    fn optional(&self) -> Vec<&Param> {
        self.params.iter().filter(|p| p.default.is_some()).collect()
    }

    fn args_ty(&self) -> proc_macro2::Ident {
        format_ident!("{}Args", pascal(&self.f.key).unwrap_or_default())
    }

    fn doc(&self) -> String {
        format!(
            " `{}`.",
            self.f.names.get("en-US").map_or("", String::as_str)
        )
    }

    /// The `Args` struct and its `Default`, if any argument is optional.
    fn args_struct(&self) -> Option<TokenStream> {
        let optional = self.optional();
        if optional.is_empty() {
            return None;
        }
        let ty = self.args_ty();
        let idents: Vec<_> = optional.iter().map(|p| &p.ident).collect();
        let types: Vec<_> = optional.iter().map(|p| &p.ty).collect();
        let defaults: Vec<_> = optional.iter().filter_map(|p| p.default.as_ref()).collect();
        let doc = format!(
            " Optional arguments of `{}`.",
            self.f.names.get("en-US").map_or("", String::as_str)
        );
        Some(quote! {
            #[doc = #doc]
            #[derive(Clone, Debug)]
            pub struct #ty {
                #( #[doc = " An optional argument."] pub #idents: Expr<#types>, )*
            }

            impl Default for #ty {
                fn default() -> Self {
                    Self { #( #idents: Expr::from_ir(#defaults) ),* }
                }
            }
        })
    }

    /// Argument conversions in signature order.
    fn arg_exprs(&self) -> Vec<TokenStream> {
        self.params
            .iter()
            .map(|p| {
                let ident = &p.ident;
                if p.default.is_some() {
                    quote!(opts.#ident.into_ir())
                } else {
                    quote!(#ident.into_expr().into_ir())
                }
            })
            .collect()
    }

    fn required_decls(&self) -> Vec<TokenStream> {
        self.required()
            .map(|p| {
                let (ident, ty) = (&p.ident, &p.ty);
                quote!(#ident: impl IntoExpr<#ty>)
            })
            .collect()
    }

    fn required_names(&self) -> Vec<&proc_macro2::Ident> {
        self.required().map(|p| &p.ident).collect()
    }
}

fn build_item<'a>(f: &'a Function, data: &Data, receiver_ok: bool) -> Result<Item<'a>, String> {
    let mut params = Vec::new();
    let skip = usize::from(f.member && receiver_ok);
    let mut seen: BTreeMap<String, usize> = BTreeMap::new();
    for arg in f.args.iter().skip(skip) {
        *seen.entry(snake(&arg.name)).or_default() += 1;
    }
    let mut used: BTreeMap<String, usize> = BTreeMap::new();
    for arg in f.args.iter().skip(skip) {
        let base = snake(&arg.name);
        let name = if seen[&base] > 1 {
            let n = used.entry(base.clone()).or_default();
            *n += 1;
            format!("{base}_{n}")
        } else {
            base
        };
        params.push(Param {
            ident: format_ident!("{}", escape(name)),
            ty: marker(&ty_key(&arg.ty), data).map_err(|e| format!("{}: {e}", f.key))?,
            default: arg.default.as_ref().map(ir_default).transpose()?,
        });
    }
    Ok(Item {
        f,
        name: String::new(),
        module: None,
        receiver: None,
        params,
        ret: quote!(()),
    })
}

/// Picks Rust names, dropping `get_` where that stays unambiguous.
fn name_items(
    items: &mut [Item<'_>],
    overrides: &BTreeMap<String, String>,
    scope: impl Fn(&Item<'_>) -> String,
) -> Result<(), String> {
    let mut counts: BTreeMap<(String, String), usize> = BTreeMap::new();
    let base: Vec<String> = items
        .iter()
        .map(|i| {
            overrides
                .get(&i.f.key)
                .cloned()
                .unwrap_or_else(|| snake(&i.f.key))
        })
        .collect();
    for (item, name) in items.iter().zip(&base) {
        *counts.entry((scope(item), name.clone())).or_default() += 1;
    }
    let taken: BTreeSet<(String, String)> = counts.keys().cloned().collect();
    for (item, name) in items.iter_mut().zip(base) {
        let mut name = name;
        if !overrides.contains_key(&item.f.key) {
            let short = name.strip_prefix("get_").map(str::to_owned);
            if let Some(short) = short {
                let key = (scope(item), short.clone());
                if !taken.contains(&key)
                    && !short.is_empty()
                    && !short.starts_with(|c: char| c.is_ascii_digit())
                {
                    name = short;
                }
            }
        }
        match name.split_once("::") {
            Some((module, rest)) => {
                item.module = Some(module.to_owned());
                item.name = escape(rest.to_owned());
            }
            None => item.name = escape(name),
        }
    }
    let mut seen = BTreeSet::new();
    for item in items.iter() {
        if !seen.insert((scope(item), item.module.clone(), item.name.clone())) {
            return Err(format!(
                "`{}` becomes the duplicate name `{}`",
                item.f.key, item.name
            ));
        }
    }
    Ok(())
}

/// Which generated file to produce.
#[derive(Clone, Copy)]
// `Actions` is only requested once its output is wired in.
#[allow(dead_code)]
pub enum Part {
    /// Free functions, receiver traits and their argument structs.
    Values,
    /// Methods on `Block` and their argument structs.
    Actions,
}

#[allow(clippy::too_many_lines)]
pub fn generate(data: &Data, renames: &Renames, part: Part) -> Result<TokenStream, String> {
    let skip_actions: BTreeSet<&str> = renames.skip.actions.iter().map(String::as_str).collect();
    let skip_values: BTreeSet<&str> = renames.skip.values.iter().map(String::as_str).collect();

    // Actions are methods on `Block`.
    let mut actions = Vec::new();
    for f in data
        .actions
        .iter()
        .filter(|f| !f.internal && !skip_actions.contains(f.key.as_str()))
    {
        let mut item = build_item(f, data, false)?;
        item.ret = quote!(());
        actions.push(item);
    }
    name_items(&mut actions, &renames.actions, |_| String::new())?;

    // Values are free functions, or methods on a receiver trait.
    let mut values = Vec::new();
    for f in data
        .values
        .iter()
        .filter(|f| !f.internal && !skip_values.contains(f.key.as_str()))
    {
        let receiver = if f.member {
            let first = f.args.first().map(|a| ty_key(&a.ty)).unwrap_or_default();
            Some(match first.as_str() {
                "Player" => "Player",
                "String" => "Str",
                "Array" | "Array<Object>" => "Array",
                other => return Err(format!("{}: no receiver trait for `{other}`", f.key)),
            })
        } else {
            None
        };
        let mut item = build_item(f, data, true)?;
        item.receiver = receiver;
        item.ret = ret_marker(&ty_key(&f.ret), data).map_err(|e| format!("{}: {e}", f.key))?;
        values.push(item);
    }
    name_items(&mut values, &renames.values, |i| {
        i.receiver.unwrap_or("").to_owned()
    })?;

    let mut seen_args = BTreeSet::new();
    for item in actions.iter().chain(&values) {
        if !item.optional().is_empty() && !seen_args.insert(item.args_ty().to_string()) {
            return Err(format!("duplicate args struct for `{}`", item.f.key));
        }
    }

    let action_methods = actions.iter().map(action_method);
    let action_args = actions.iter().filter_map(Item::args_struct);
    let value_args = values.iter().filter_map(Item::args_struct);

    let free: Vec<_> = values
        .iter()
        .filter(|i| i.receiver.is_none() && i.module.is_none())
        .map(free_fn)
        .collect();
    let mut modules: BTreeMap<&str, Vec<TokenStream>> = BTreeMap::new();
    for item in values.iter().filter(|i| i.receiver.is_none()) {
        if let Some(module) = &item.module {
            modules.entry(module).or_default().push(free_fn(item));
        }
    }
    let modules = modules.into_iter().map(|(name, fns)| {
        let ident = format_ident!("{name}");
        let doc = format!(" Functions of the `{name}` group.");
        quote! {
            #[doc = #doc]
            pub mod #ident {
                use super::*;
                #(#fns)*
            }
        }
    });

    let mut traits = Vec::new();
    for (recv, trait_name, bound) in [
        ("Player", "PlayerExt", quote!(Player)),
        ("Str", "StrExt", quote!(Str)),
        ("Array", "ArrayExt", quote!(Array<Any>)),
    ] {
        let members: Vec<_> = values.iter().filter(|i| i.receiver == Some(recv)).collect();
        if members.is_empty() {
            continue;
        }
        let trait_ident = format_ident!("{trait_name}");
        let decls = members.iter().map(|i| member_decl(i));
        let impls = members.iter().map(|i| member_impl(i));
        let doc = format!(" Values that take a `{recv}` as their first argument.");
        traits.push(quote! {
            #[doc = #doc]
            pub trait #trait_ident {
                #(#decls)*
            }

            impl<T: Is<#bound>> #trait_ident for Expr<T> {
                #(#impls)*
            }
        });
    }

    let generated_actions = actions
        .iter()
        .map(|i| format_ident!("{}", pascal(&i.f.key).unwrap_or_default()));
    let generated_values = values
        .iter()
        .map(|i| format_ident!("{}", pascal(&i.f.key).unwrap_or_default()));
    let hand_actions = renames
        .skip
        .actions
        .iter()
        .map(|k| format_ident!("{}", pascal(k).unwrap_or_default()));
    let hand_values = renames
        .skip
        .values
        .iter()
        .map(|k| format_ident!("{}", pascal(k).unwrap_or_default()));

    Ok(match part {
        Part::Values => quote! {
            //! Typed values generated from the signature tables.

            use ow_workshop_data::ValueId;

            use crate::expr::Expr;
            use crate::support;
            use crate::ty::*;

            /// Values generated here.
            pub(crate) const GENERATED_VALUES: &[ValueId] = &[ #(ValueId::#generated_values),* ];
            /// Values with a hand-written API instead.
            pub(crate) const HAND_WRITTEN_VALUES: &[ValueId] = &[ #(ValueId::#hand_values),* ];

            #(#free)*
            #(#modules)*
            #(#traits)*
            #(#value_args)*
        },
        Part::Actions => quote! {
            //! Typed actions generated from the signature tables.

            use ow_workshop_data::{ActionId, ValueId};

            use crate::block::Block;
            use crate::expr::Expr;
            use crate::ir::StmtKind;
            use crate::support;
            use crate::ty::*;

            /// Actions generated here.
            pub(crate) const GENERATED_ACTIONS: &[ActionId] = &[ #(ActionId::#generated_actions),* ];
            /// Actions with a hand-written API instead.
            pub(crate) const HAND_WRITTEN_ACTIONS: &[ActionId] = &[ #(ActionId::#hand_actions),* ];

            #(#action_args)*

            impl Block {
                #(#action_methods)*
            }
        },
    })
}

fn free_fn(item: &Item<'_>) -> TokenStream {
    let name = format_ident!("{}", item.name);
    let id = format_ident!("{}", pascal(&item.f.key).unwrap_or_default());
    let (doc, ret) = (item.doc(), &item.ret);
    let decls = item.required_decls();
    let names = item.required_names();
    let args = item.arg_exprs();
    if item.optional().is_empty() {
        return quote! {
            #[doc = #doc]
            #[track_caller]
            #[must_use]
            pub fn #name(#(#decls),*) -> Expr<#ret> {
                support::value(ValueId::#id, vec![#(#args),*])
            }
        };
    }
    let with = format_ident!("{}_with", item.name);
    let args_ty = item.args_ty();
    quote! {
        #[doc = #doc]
        #[track_caller]
        #[must_use]
        pub fn #name(#(#decls),*) -> Expr<#ret> {
            #with(#(#names,)* #args_ty::default())
        }

        #[doc = #doc]
        #[track_caller]
        #[must_use]
        pub fn #with(#(#decls,)* opts: #args_ty) -> Expr<#ret> {
            support::value(ValueId::#id, vec![#(#args),*])
        }
    }
}

fn member_decl(item: &Item<'_>) -> TokenStream {
    let name = format_ident!("{}", item.name);
    let (doc, ret) = (item.doc(), &item.ret);
    let decls = item.required_decls();
    if item.optional().is_empty() {
        return quote! {
            #[doc = #doc]
            #[track_caller]
            fn #name(&self, #(#decls),*) -> Expr<#ret>;
        };
    }
    let with = format_ident!("{}_with", item.name);
    let args_ty = item.args_ty();
    let names = item.required_names();
    quote! {
        #[doc = #doc]
        #[track_caller]
        fn #name(&self, #(#decls),*) -> Expr<#ret> {
            self.#with(#(#names,)* #args_ty::default())
        }

        #[doc = #doc]
        #[track_caller]
        fn #with(&self, #(#decls,)* opts: #args_ty) -> Expr<#ret>;
    }
}

fn member_impl(item: &Item<'_>) -> TokenStream {
    let id = format_ident!("{}", pascal(&item.f.key).unwrap_or_default());
    let ret = &item.ret;
    let decls = item.required_decls();
    let args = item.arg_exprs();
    if item.optional().is_empty() {
        let name = format_ident!("{}", item.name);
        quote! {
            #[track_caller]
            fn #name(&self, #(#decls),*) -> Expr<#ret> {
                support::value(ValueId::#id, vec![self.ir().clone() #(, #args)*])
            }
        }
    } else {
        let with = format_ident!("{}_with", item.name);
        let args_ty = item.args_ty();
        quote! {
            #[track_caller]
            fn #with(&self, #(#decls,)* opts: #args_ty) -> Expr<#ret> {
                support::value(ValueId::#id, vec![self.ir().clone() #(, #args)*])
            }
        }
    }
}

fn action_method(item: &Item<'_>) -> TokenStream {
    let name = format_ident!("{}", item.name);
    let id = format_ident!("{}", pascal(&item.f.key).unwrap_or_default());
    let doc = item.doc();
    let decls = item.required_decls();
    let names = item.required_names();
    let args = item.arg_exprs();
    if item.optional().is_empty() {
        return quote! {
            #[doc = #doc]
            #[track_caller]
            pub fn #name(&mut self, #(#decls),*) {
                self.push(StmtKind::Action { id: ActionId::#id, args: vec![#(#args),*] });
            }
        };
    }
    let with = format_ident!("{}_with", item.name);
    let args_ty = item.args_ty();
    quote! {
        #[doc = #doc]
        #[track_caller]
        pub fn #name(&mut self, #(#decls),*) {
            self.#with(#(#names,)* #args_ty::default());
        }

        #[doc = #doc]
        #[track_caller]
        pub fn #with(&mut self, #(#decls,)* opts: #args_ty) {
            self.push(StmtKind::Action { id: ActionId::#id, args: vec![#(#args),*] });
        }
    }
}
