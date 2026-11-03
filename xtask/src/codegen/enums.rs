//! Enums for constants, heroes, maps, gamemodes and extensions.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::naming::variants;
use crate::data::Data;
use crate::data::model::Names;

/// A generated file: module name and its tokens.
pub type File = (&'static str, TokenStream);

/// One enum variant: its Rust name, data key and English name.
struct Variant<'a> {
    key: &'a str,
    name: proc_macro2::Ident,
    doc: String,
}

fn english(names: &Names) -> String {
    names.get("en-US").cloned().unwrap_or_default()
}

fn doc_name(text: &str) -> String {
    format!(" `{}`", text.replace('`', "'"))
}

fn build<'a>(entries: impl Iterator<Item = (&'a str, String)>) -> Result<Vec<Variant<'a>>, String> {
    let entries: Vec<_> = entries.collect();
    let names = variants(entries.iter().map(|(k, _)| *k))?;
    Ok(names
        .into_iter()
        .zip(entries)
        .map(|((key, name), (_, doc))| Variant {
            key,
            name: format_ident!("{name}"),
            doc,
        })
        .collect())
}

/// Enum definition plus `ALL` and `key()`.
fn enum_def(name: &str, doc: &str, variants: &[Variant]) -> TokenStream {
    let ident = format_ident!("{name}");
    let names: Vec<_> = variants.iter().map(|v| &v.name).collect();
    let docs = variants.iter().map(|v| doc_name(&v.doc));
    let keys = variants.iter().map(|v| v.key);
    let doc = format!(" {doc}");
    quote! {
        #[doc = #doc]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[non_exhaustive]
        pub enum #ident {
            #( #[doc = #docs] #names, )*
        }

        impl #ident {
            /// Every variant, in declaration order.
            pub const ALL: &'static [Self] = &[ #( Self::#names ),* ];

            /// The data key, for debugging.
            pub fn key(self) -> &'static str {
                match self {
                    #( Self::#names => #keys, )*
                }
            }
        }
    }
}

/// Groups variants sharing a value so equal match arms collapse into one or-pattern.
fn grouped<'a, K: Ord>(
    entries: impl Iterator<Item = (&'a proc_macro2::Ident, K)>,
) -> Vec<(Vec<&'a proc_macro2::Ident>, K)> {
    let mut groups: std::collections::BTreeMap<K, Vec<&proc_macro2::Ident>> =
        std::collections::BTreeMap::new();
    for (name, key) in entries {
        groups.entry(key).or_default().push(name);
    }
    groups
        .into_iter()
        .map(|(key, names)| (names, key))
        .collect()
}

/// `matches!(self, Self::A | Self::B)` or `false` for an empty set.
fn any_of(names: &[&proc_macro2::Ident]) -> TokenStream {
    if names.is_empty() {
        quote!(false)
    } else {
        quote!(matches!(self, #( Self::#names )|*))
    }
}

pub fn generate(data: &Data) -> Result<Vec<File>, String> {
    Ok(vec![
        ("constants", constants(data)?),
        ("extension", extension(data)?),
        ("gamemodes", gamemodes(data)?),
        ("heroes", heroes(data)?),
        ("maps", maps(data)?),
    ])
}

fn constants(data: &Data) -> Result<TokenStream, String> {
    let mut items = Vec::new();
    for (enum_name, members) in &data.constants {
        // Tooling-only colors are not workshop values; the stdlib exposes them as `rgb()` constants.
        let kept: Vec<_> = members.iter().filter(|(_, m)| !m.only_in_tooling).collect();
        let vars = build(kept.iter().map(|(k, m)| {
            (
                k.as_str(),
                m.names.as_ref().map(english).unwrap_or_default(),
            )
        }))
        .map_err(|e| format!("{enum_name}: {e}"))?;
        let doc = constant_doc(enum_name, data);
        let mut item = enum_def(enum_name, &doc, &vars);

        let ident = format_ident!("{enum_name}");
        let rgba: Vec<_> = kept
            .iter()
            .zip(&vars)
            .filter_map(|((_, m), v)| m.rgba.map(|c| (&v.name, c)))
            .collect();
        if !rgba.is_empty() {
            let (names, colors): (Vec<_>, Vec<_>) = rgba
                .into_iter()
                .map(|(name, c)| {
                    let [r, g, b, a] = c.map(proc_macro2::Literal::u8_unsuffixed);
                    (name, quote!([#r, #g, #b, #a]))
                })
                .unzip();
            item.extend(quote! {
                impl #ident {
                    /// The color's RGBA components, if it has fixed ones.
                    pub fn rgba(self) -> Option<[u8; 4]> {
                        match self {
                            #( Self::#names => Some(#colors), )*
                            _ => None,
                        }
                    }
                }
            });
        }
        // Group effects by extension so equal arms collapse into one or-pattern.
        let mut by_extension: std::collections::BTreeMap<String, Vec<&proc_macro2::Ident>> =
            std::collections::BTreeMap::new();
        for ((_, member), variant) in kept.iter().zip(&vars) {
            if let Some(extension) = &member.extension {
                by_extension
                    .entry(pascal_key(extension))
                    .or_default()
                    .push(&variant.name);
            }
        }
        if !by_extension.is_empty() {
            let exts = by_extension.keys().map(|e| format_ident!("{e}"));
            let groups = by_extension
                .values()
                .map(|names| quote!( #( Self::#names )|* ));
            item.extend(quote! {
                impl #ident {
                    /// The workshop extension this effect requires, if any.
                    pub fn extension(self) -> Option<Extension> {
                        match self {
                            #( #groups => Some(Extension::#exts), )*
                            _ => None,
                        }
                    }
                }
            });
        }
        items.push(item);
    }
    Ok(quote! {
        //! Constant enums used as workshop action and value arguments.

        use super::extension::Extension;

        #( #items )*
    })
}

/// Variant name of an extension key (`beamEffects` to `BeamEffects`).
fn pascal_key(key: &str) -> String {
    super::naming::pascal(key).unwrap_or_default()
}

fn extension(data: &Data) -> Result<TokenStream, String> {
    let vars = build(
        data.settings
            .extensions
            .values
            .iter()
            .map(|(k, e)| (k.as_str(), english(&e.names))),
    )
    .map_err(|e| format!("extensions: {e}"))?;
    let def = enum_def("Extension", "A workshop extension.", &vars);
    Ok(quote! {
        //! Workshop extensions.

        #def
    })
}

fn gamemodes(data: &Data) -> Result<TokenStream, String> {
    let vars = build(
        data.gamemodes
            .iter()
            .map(|(k, m)| (k.as_str(), english(&m.names))),
    )
    .map_err(|e| format!("gamemodes: {e}"))?;
    let def = enum_def("Gamemode", "A game mode.", &vars);
    let modes: Vec<_> = data.gamemodes.values().collect();
    let ow1: Vec<_> = vars
        .iter()
        .zip(&modes)
        .filter(|(_, m)| m.only_in_ow1)
        .map(|(v, _)| &v.name)
        .collect();
    let ow1 = any_of(&ow1);
    let slots = |pick: fn(&crate::data::model::Gamemode) -> Option<u32>| {
        let groups = grouped(
            vars.iter()
                .zip(&modes)
                .filter_map(|(v, m)| pick(m).map(|n| (&v.name, n))),
        );
        let (names, counts): (Vec<_>, Vec<_>) = groups
            .into_iter()
            .map(|(names, n)| {
                (
                    quote!( #( Self::#names )|* ),
                    proc_macro2::Literal::u32_unsuffixed(n),
                )
            })
            .unzip();
        quote! { match self { #( #names => Some(#counts), )* _ => None } }
    };
    let (t1, t2, ffa) = (
        slots(|m| m.default_team1_slots),
        slots(|m| m.default_team2_slots),
        slots(|m| m.default_ffa_slots),
    );
    Ok(quote! {
        //! Game modes.

        #def

        /// Default player slots per team of a game mode.
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
        pub struct Slots {
            /// Slots in team 1.
            pub team1: Option<u32>,
            /// Slots in team 2.
            pub team2: Option<u32>,
            /// Slots in free-for-all.
            pub ffa: Option<u32>,
        }

        impl Gamemode {
            /// Whether the mode only exists in Overwatch 1.
            pub fn only_in_ow1(self) -> bool {
                #ow1
            }

            /// The mode's default slot counts.
            pub fn default_slots(self) -> Slots {
                Slots { team1: #t1, team2: #t2, ffa: #ffa }
            }
        }
    })
}

fn heroes(data: &Data) -> Result<TokenStream, String> {
    let vars = build(
        data.heroes
            .iter()
            .map(|(k, h)| (k.as_str(), english(&h.names))),
    )
    .map_err(|e| format!("heroes: {e}"))?;
    let def = enum_def("Hero", "A playable hero.", &vars);
    Ok(quote! {
        //! Heroes.

        #def
    })
}

fn maps(data: &Data) -> Result<TokenStream, String> {
    let vars = build(
        data.maps
            .iter()
            .map(|(k, m)| (k.as_str(), english(&m.names))),
    )
    .map_err(|e| format!("maps: {e}"))?;
    let def = enum_def("Map", "A map.", &vars);
    let maps: Vec<_> = data.maps.values().collect();
    let flagged = |pick: fn(&crate::data::model::Map) -> bool| {
        let names: Vec<_> = vars
            .iter()
            .zip(&maps)
            .filter(|(_, m)| pick(m))
            .map(|(v, _)| &v.name)
            .collect();
        any_of(&names)
    };
    let workshop = flagged(|m| m.is_workshop_map);
    let ow1 = flagged(|m| m.only_in_ow1);

    let mut by_modes = Vec::new();
    let mut by_variants = Vec::new();
    for (v, m) in vars.iter().zip(&maps) {
        if !m.gamemodes.is_empty() {
            let modes = m
                .gamemodes
                .iter()
                .map(|g| super::naming::pascal(g))
                .collect::<Result<Vec<_>, _>>()?;
            by_modes.push((&v.name, modes));
        }
        if let Some(variants) = m.variants.as_ref().filter(|v| !v.is_empty()) {
            let pairs: Vec<_> = variants
                .iter()
                .map(|(k, id)| (k.clone(), id.clone()))
                .collect();
            by_variants.push((&v.name, pairs));
        }
    }
    let (covered_modes, covered_variants) = (by_modes.len(), by_variants.len());
    let mode_arms: Vec<_> = grouped(by_modes.into_iter())
        .into_iter()
        .map(|(names, modes)| {
            let modes = modes.iter().map(|g| format_ident!("{g}"));
            quote! { #( Self::#names )|* => &[ #( Gamemode::#modes ),* ], }
        })
        .collect();
    let variant_arms: Vec<_> = grouped(by_variants.into_iter())
        .into_iter()
        .map(|(names, pairs)| {
            let (keys, ids): (Vec<_>, Vec<_>) = pairs.into_iter().unzip();
            quote! { #( Self::#names )|* => &[ #( (#keys, #ids) ),* ], }
        })
        .collect();
    // A catch-all arm is only reachable (and only compiles cleanly) if some map is missing.
    let fallback = |arms: usize| {
        if arms < vars.len() {
            quote!(_ => &[],)
        } else {
            quote!()
        }
    };
    let (mode_fallback, variant_fallback) = (fallback(covered_modes), fallback(covered_variants));
    Ok(quote! {
        //! Maps.

        use super::gamemodes::Gamemode;

        #def

        impl Map {
            /// Whether this is a workshop-only map.
            pub fn is_workshop_map(self) -> bool {
                #workshop
            }

            /// Whether the map only exists in Overwatch 1.
            pub fn only_in_ow1(self) -> bool {
                #ow1
            }

            /// The game modes this map supports.
            pub fn gamemodes(self) -> &'static [Gamemode] {
                match self {
                    #( #mode_arms )*
                    #mode_fallback
                }
            }

            /// Map variants as `(key, id)` pairs.
            pub fn variants(self) -> &'static [(&'static str, &'static str)] {
                match self {
                    #( #variant_arms )*
                    #variant_fallback
                }
            }
        }
    })
}

/// Words abbreviated in constant enum names.
const ABBREVIATIONS: [(&str, &str); 7] = [
    ("Accel", "Acceleration"),
    ("Hud", "HUD"),
    ("Los", "Line Of Sight"),
    ("Op", "Operation"),
    ("Reeval", "Reevaluation"),
    ("Spec", "Spectator"),
    ("Comms", "Communication"),
];

/// A readable one-line doc for a constant enum: its spelled-out name and where it is used.
fn constant_doc(enum_name: &str, data: &Data) -> String {
    use crate::data::model::Ty;

    // `AccelReeval` becomes "Acceleration Reevaluation".
    let mut title = String::new();
    let mut word = String::new();
    let mut flush = |word: &mut String, title: &mut String| {
        if word.is_empty() {
            return;
        }
        if !title.is_empty() {
            title.push(' ');
        }
        let spelled = ABBREVIATIONS
            .iter()
            .find(|(a, _)| *a == word.as_str())
            .map_or(word.as_str(), |(_, s)| *s);
        title.push_str(spelled);
        word.clear();
    };
    for c in enum_name.chars() {
        if c.is_uppercase() {
            flush(&mut word, &mut title);
        }
        word.push(c);
    }
    flush(&mut word, &mut title);

    // The first argument that takes this enum, and the functions that use it.
    let mut arg_name = None;
    let mut users: Vec<&str> = Vec::new();
    for function in data.actions.iter().chain(&data.values) {
        for arg in &function.args {
            if matches!(&arg.ty, Ty::Named(n) if n == enum_name) {
                arg_name.get_or_insert(arg.name.as_str());
                let name = function
                    .names
                    .get("en-US")
                    .map_or(function.key.as_str(), String::as_str);
                if !users.contains(&name) {
                    users.push(name);
                }
            }
        }
    }
    let Some(arg) = arg_name else {
        return format!("{title} (`{enum_name}`).");
    };
    let shown: Vec<String> = users.iter().take(2).map(|u| format!("`{u}`")).collect();
    let more = users.len().saturating_sub(2);
    let mut used = shown.join(", ");
    if more > 0 {
        used.push_str(&format!(" and {more} more"));
    }
    let mut doc = format!("{title} (`{enum_name}`): options for the `{arg}` argument of {used}.");
    if enum_name.ends_with("Reeval") {
        doc.push_str(
            " Reevaluation decides which of the action's arguments are re-read while it runs.",
        );
    }
    doc
}
