//! Typed setters for the custom game settings builders (`ow-workshop/src/generated/settings.rs`).

use std::collections::BTreeMap;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::data::Data;
use crate::data::model::{Kind, Setting};

/// What a setter takes, which is what decides the stored value.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Shape {
    Bool,
    Number,
    Percent,
    Str,
    Choice,
}

fn shape(kind: &Kind) -> Shape {
    match kind {
        Kind::BoolYesNo
        | Kind::BoolEnabled
        | Kind::BoolReverseEnabled
        | Kind::BoolOnOff
        | Kind::BoolReverseOnOff => Shape::Bool,
        Kind::Int | Kind::Float => Shape::Number,
        Kind::Percent => Shape::Percent,
        Kind::String => Shape::Str,
        Kind::Combo => Shape::Choice,
    }
}

/// Lower snake case of a camelCase key; `%` becomes `_percent`.
fn snake(key: &str) -> String {
    let chars: Vec<char> = key.chars().collect();
    let mut out = String::new();
    for (i, &c) in chars.iter().enumerate() {
        if c == '%' {
            out.push_str("_percent");
        } else if c.is_uppercase() {
            let prev = i.checked_sub(1).map(|j| chars[j]);
            let next = chars.get(i + 1).copied();
            let boundary = prev.is_some_and(|p| {
                p.is_lowercase()
                    || p.is_ascii_digit()
                    || (p.is_uppercase() && next.is_some_and(char::is_lowercase))
            });
            if boundary {
                out.push('_');
            }
            out.extend(c.to_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

/// Settings by key; a key whose shape differs between sections gets no typed setter.
#[derive(Default)]
struct Group<'a>(BTreeMap<&'a str, Option<(Shape, &'a Setting)>>);

impl<'a> Group<'a> {
    fn add(&mut self, values: &'a BTreeMap<String, Setting>) {
        for (key, setting) in values {
            let shape = shape(&setting.kind);
            self.0
                .entry(key)
                .and_modify(|slot| {
                    if slot.is_some_and(|(s, _)| s != shape) {
                        *slot = None;
                    }
                })
                .or_insert(Some((shape, setting)));
        }
    }

    fn setters(&self, owner: &str, field: &str) -> Result<TokenStream, String> {
        let owner = format_ident!("{owner}");
        let field = format_ident!("{field}");
        let mut seen = BTreeMap::new();
        let mut methods = Vec::new();
        for (key, slot) in &self.0 {
            let Some((shape, _)) = slot else { continue };
            let name = snake(key);
            if let Some(other) = seen.insert(name.clone(), *key) {
                return Err(format!("`{key}` and `{other}` both become `{name}`"));
            }
            let name = format_ident!("{name}");
            let doc = format!(" Sets `{key}`.");
            let (param, value) = match shape {
                Shape::Bool => (quote!(bool), quote!(SettingValue::Bool(value))),
                Shape::Number => (
                    quote!(impl Into<f64>),
                    quote!(SettingValue::Number(value.into())),
                ),
                Shape::Percent => (
                    quote!(impl Into<f64>),
                    quote!(SettingValue::Percent(value.into())),
                ),
                Shape::Str => (
                    quote!(impl Into<String>),
                    quote!(SettingValue::Str(value.into())),
                ),
                Shape::Choice => (quote!(&str), quote!(SettingValue::Choice(value.to_owned()))),
            };
            methods.push(quote! {
                #[doc = #doc]
                #[must_use]
                pub fn #name(mut self, value: #param) -> Self {
                    self.#field.set(#key, #value);
                    self
                }
            });
        }
        Ok(quote! { impl #owner { #( #methods )* } })
    }
}

pub fn generate(data: &Data) -> Result<TokenStream, String> {
    let s = &data.settings;
    let mut lobby = Group::default();
    lobby.add(&s.lobby.values);
    let mut mode = Group::default();
    for section in s.gamemodes.values.values() {
        mode.add(&section.values);
    }
    let mut hero = Group::default();
    hero.add(&s.heroes.general);
    let (lobby, mode, hero) = (
        lobby.setters("Lobby", "values")?,
        mode.setters("ModeSettings", "values")?,
        hero.setters("HeroSettings", "general")?,
    );
    Ok(quote! {
        //! Typed setters for the custom game settings builders.

        use crate::settings::{HeroSettings, Lobby, ModeSettings, SettingValue};

        #lobby
        #mode
        #hero
    })
}
