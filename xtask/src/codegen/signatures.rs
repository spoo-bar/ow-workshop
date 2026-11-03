//! The `Ty` enum and the action and value signature tables.

use std::collections::{BTreeMap, BTreeSet};

use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};

use super::enums::{build, enum_def};
use super::naming::pascal;
use crate::data::Data;
use crate::data::model::{Default as Dflt, Function, Ty, TypeNode};

pub fn ty_file(data: &Data) -> Result<TokenStream, String> {
    let mut names: BTreeSet<&str> = data.constants.keys().map(String::as_str).collect();
    let mut parents: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    for node in &data.tree {
        collect(node, None, &mut names, &mut parents);
    }
    // Literal types missing from the tree are subtypes of their base type.
    for function in data.actions.iter().chain(&data.values) {
        for arg in &function.args {
            literal_names(&arg.ty, &mut names);
        }
    }
    let literals: Vec<_> = names
        .iter()
        .copied()
        .filter(|n| n.ends_with("Literal"))
        .collect();
    for name in literals {
        if parents.contains_key(name) {
            continue;
        }
        let base = name.strip_suffix("Literal").unwrap_or(name);
        let parent = match base {
            "CustomString" | "LocalizedString" | "String" => "String",
            _ if names.contains(base) => base,
            _ => continue,
        };
        parents.entry(name).or_default().insert(parent);
    }

    let vars =
        build(names.iter().map(|n| (*n, (*n).to_owned()))).map_err(|e| format!("types: {e}"))?;
    let ident = |name: &str| format_ident!("{}", pascal(name).unwrap_or_default());
    let variants = vars.iter().map(|v| &v.name);
    let docs = vars.iter().map(|v| format!(" `{}`", v.doc));
    let mut arms: BTreeMap<Vec<&str>, Vec<&str>> = BTreeMap::new();
    for (child, set) in &parents {
        arms.entry(set.iter().copied().collect())
            .or_default()
            .push(child);
    }
    let arms = arms.iter().map(|(ps, children)| {
        let children = children.iter().map(|c| ident(c));
        let ps = ps.iter().map(|p| ident(p));
        quote! { #( Self::#children )|* => &[ #( Self::#ps ),* ], }
    });
    Ok(quote! {
        //! The type grammar of action and value arguments.

        /// A type in a signature: a named type, an array, a union or a typed vector.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[non_exhaustive]
        pub enum Ty {
            #( #[doc = #docs] #variants, )*
            /// An array with a known element type.
            ArrayOf(&'static Ty),
            /// Any of the listed types.
            Union(&'static [Ty]),
            /// A vector with per-component types.
            Vec3(VectorKind, &'static [Ty; 3]),
        }

        /// Which vector-like type a `Ty::Vec3` stands for.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub enum VectorKind {
            /// `Vector`
            Vector,
            /// `Direction`
            Direction,
            /// `Position`
            Position,
            /// `Velocity`
            Velocity,
        }

        impl Ty {
            /// The types this type is a direct subtype of.
            pub fn parents(self) -> &'static [Ty] {
                match self {
                    #( #arms )*
                    _ => &[],
                }
            }
        }
    })
}

fn collect<'a>(
    node: &'a TypeNode,
    parent: Option<&'a str>,
    names: &mut BTreeSet<&'a str>,
    parents: &mut BTreeMap<&'a str, BTreeSet<&'a str>>,
) {
    match node {
        TypeNode::Leaf(name) => {
            names.insert(name);
            if let Some(parent) = parent {
                parents.entry(name).or_default().insert(parent);
            }
        }
        TypeNode::Parent(map) => {
            for (name, children) in map {
                names.insert(name);
                if let Some(parent) = parent {
                    parents.entry(name).or_default().insert(parent);
                }
                for child in children {
                    collect(child, Some(name), names, parents);
                }
            }
        }
    }
}

fn literal_names<'a>(ty: &'a Ty, names: &mut BTreeSet<&'a str>) {
    match ty {
        Ty::Named(name) => {
            names.insert(name);
        }
        Ty::Array { array } => literal_names(array, names),
        Ty::Union(types) => types.iter().for_each(|t| literal_names(t, names)),
        Ty::Vector(v) => {
            for part in [&v.vector, &v.direction, &v.position, &v.velocity]
                .into_iter()
                .flatten()
                .flatten()
            {
                literal_names(part, names);
            }
        }
    }
}

fn ty(t: &Ty) -> Result<TokenStream, String> {
    Ok(match t {
        Ty::Named(name) => {
            let name = format_ident!("{}", pascal(name)?);
            quote!(Ty::#name)
        }
        Ty::Array { array } => {
            let inner = ty(array)?;
            quote!(Ty::ArrayOf(&#inner))
        }
        Ty::Union(types) => {
            let types = types.iter().map(ty).collect::<Result<Vec<_>, _>>()?;
            quote!(Ty::Union(&[ #( #types ),* ]))
        }
        Ty::Vector(v) => {
            let (kind, parts) = [
                ("Vector", &v.vector),
                ("Direction", &v.direction),
                ("Position", &v.position),
                ("Velocity", &v.velocity),
            ]
            .into_iter()
            .find_map(|(k, p)| p.as_ref().map(|p| (k, p)))
            .ok_or("empty vector type")?;
            let kind = format_ident!("{kind}");
            let parts = parts.iter().map(|p| ty(p)).collect::<Result<Vec<_>, _>>()?;
            quote!(Ty::Vec3(VectorKind::#kind, &[ #( #parts ),* ]))
        }
    })
}

fn default(d: &Dflt) -> Result<TokenStream, String> {
    Ok(match d {
        Dflt::Num { num } => {
            let n = Literal::f64_unsuffixed(*num);
            quote!(DefaultValue::Num(#n))
        }
        Dflt::Bool { bool } => quote!(DefaultValue::Bool(#bool)),
        Dflt::Null(_) => quote!(DefaultValue::Null),
        Dflt::Const { r#const: (e, m) } => quote!(DefaultValue::Const(#e, #m)),
        Dflt::Value { value, args } => {
            let id = format_ident!("{}", pascal(value)?);
            let args = args.iter().map(default).collect::<Result<Vec<_>, _>>()?;
            quote!(DefaultValue::Value(ValueId::#id, &[ #( #args ),* ]))
        }
        Dflt::Vector { vector } => {
            let [x, y, z] = vector.map(Literal::f64_unsuffixed);
            quote!(DefaultValue::Vector([#x, #y, #z]))
        }
    })
}

fn signature(f: &Function) -> Result<TokenStream, String> {
    let mut args = Vec::new();
    for arg in &f.args {
        let (name, t) = (&arg.name, ty(&arg.ty)?);
        let mut tokens = quote!(Arg::new(#name, #t));
        if let Some(d) = &arg.default {
            let d = default(d)?;
            tokens = quote!(#tokens.default(#d));
        }
        if let (Some(min), Some(max)) = (arg.min, arg.max) {
            let (min, max) = (Literal::f64_unsuffixed(min), Literal::f64_unsuffixed(max));
            tokens = quote!(#tokens.range(#min, #max));
        }
        if let Some(max) = arg.literal_max {
            let max = Literal::f64_unsuffixed(max);
            tokens = quote!(#tokens.literal_max(#max));
        }
        if let Some(r) = &arg.replace {
            let bits = u8::from(r.zero_by_false)
                | u8::from(r.zero_by_null) << 1
                | u8::from(r.one_by_true) << 2
                | u8::from(r.null_vector_by_null) << 3
                | u8::from(r.empty_string_by_empty_array) << 4;
            if bits != 0 {
                let bits = Literal::u8_unsuffixed(bits);
                tokens = quote!(#tokens.replace(#bits));
            }
        }
        args.push(tokens);
    }
    let ret = ty(&f.ret)?;
    let flags = u8::from(f.member)
        | u8::from(f.internal) << 1
        | u8::from(f.has_literal_limit) << 2
        | u8::from(f.is_constant) << 3
        | u8::from(f.is_literal) << 4
        | u8::from(f.can_be_put_in_boolean) << 5;
    let flags = Literal::u8_unsuffixed(flags);
    Ok(quote!(Signature::new(&[ #( #args ),* ], #ret, SigFlags::from_bits(#flags))))
}

/// `ActionId` / `ValueId` plus the signature table, in key order.
fn table(functions: &[Function], id: &str, title: &str) -> Result<TokenStream, String> {
    let mut sorted: Vec<&Function> = functions.iter().collect();
    sorted.sort_by(|a, b| a.key.cmp(&b.key));
    let vars = build(sorted.iter().map(|f| {
        (
            f.key.as_str(),
            f.names.get("en-US").cloned().unwrap_or_default(),
        )
    }))
    .map_err(|e| format!("{id}: {e}"))?;
    let def = enum_def(id, &format!("An {title}."), &vars);
    let signatures = sorted
        .iter()
        .map(|f| signature(f))
        .collect::<Result<Vec<_>, _>>()?;
    let count = signatures.len();
    let module_doc = format!(" Signatures of every {title}.");
    let body = quote! {
        #def

        /// Signatures indexed by id.
        pub static SIGNATURES: [Signature; #count] = [ #( #signatures ),* ];
    };
    // Import only what the table uses, to stay warning-free.
    let text = body.to_string();
    let mut imports = vec![quote!(
        use crate::signature::{Arg, SigFlags, Signature};
    )];
    if text.contains("DefaultValue") {
        imports.push(quote!(
            use crate::signature::DefaultValue;
        ));
    }
    if text.contains("VectorKind") {
        imports.push(quote!(
            use crate::VectorKind;
        ));
    }
    if text.contains("ValueId ::") && id != "ValueId" {
        imports.push(quote!(
            use crate::ValueId;
        ));
    }
    Ok(quote! {
        #![doc = #module_doc]

        use crate::Ty;
        #( #imports )*

        #body
    })
}

pub fn generate(data: &Data) -> Result<Vec<super::enums::File>, String> {
    Ok(vec![
        ("actions", table(&data.actions, "ActionId", "action")?),
        ("ty", ty_file(data)?),
        ("values", table(&data.values, "ValueId", "value")?),
    ])
}
