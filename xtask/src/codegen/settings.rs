//! The custom game settings tables.

use std::collections::BTreeMap;

use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};

use super::enums::{File, build, enum_def, pack};
use crate::data::Data;
use crate::data::model::{CustomGameSettings, Kind, Names, Setting};

/// Deduplicated name table, choice sets and hero sets shared by all sections.
#[derive(Default)]
struct Tables {
    names: Vec<String>,
    name_index: BTreeMap<String, u16>,
    choice_sets: Vec<TokenStream>,
    choice_index: BTreeMap<String, usize>,
    hero_sets: Vec<Vec<String>>,
}

impl Tables {
    fn name(&mut self, names: &Names) -> Result<u16, String> {
        let packed = pack(Some(names));
        if let Some(index) = self.name_index.get(&packed) {
            return Ok(*index);
        }
        let index = u16::try_from(self.names.len()).map_err(|_| "too many names")?;
        self.name_index.insert(packed.clone(), index);
        self.names.push(packed);
        Ok(index)
    }

    fn hero_set(&mut self, heroes: &[String]) -> Result<u8, String> {
        let mut sorted = heroes.to_vec();
        sorted.sort();
        let index = if let Some(index) = self.hero_sets.iter().position(|s| *s == sorted) {
            index
        } else {
            self.hero_sets.push(sorted);
            self.hero_sets.len() - 1
        };
        u8::try_from(index).map_err(|_| "too many hero sets".to_owned())
    }

    fn choice_set(&mut self, choices: &BTreeMap<String, Names>) -> Result<usize, String> {
        let mut entries = Vec::new();
        for (key, names) in choices {
            entries.push((key.clone(), self.name(names)?));
        }
        let id = format!("{entries:?}");
        if let Some(index) = self.choice_index.get(&id) {
            return Ok(*index);
        }
        let count = entries.len();
        let rows = entries.iter().map(|(key, name)| {
            let name = Literal::u16_unsuffixed(*name);
            quote!(Choice::new(#key, #name))
        });
        let ident = format_ident!("CHOICES_{:02}", self.choice_sets.len());
        self.choice_sets
            .push(quote! { pub static #ident: [Choice; #count] = [ #( #rows ),* ]; });
        self.choice_index.insert(id, self.choice_sets.len() - 1);
        Ok(self.choice_sets.len() - 1)
    }

    fn row(&mut self, key: &str, s: &Setting) -> Result<TokenStream, String> {
        let name = Literal::u16_unsuffixed(self.name(&s.names)?);
        let kind = match s.kind {
            Kind::BoolYesNo => quote!(Kind::BoolYesNo),
            Kind::BoolEnabled => quote!(Kind::BoolEnabled),
            Kind::BoolReverseEnabled => quote!(Kind::BoolReverseEnabled),
            Kind::BoolOnOff => quote!(Kind::BoolOnOff),
            Kind::BoolReverseOnOff => quote!(Kind::BoolReverseOnOff),
            Kind::Percent => quote!(Kind::Percent),
            Kind::Int => quote!(Kind::Int),
            Kind::Float => quote!(Kind::Float),
            Kind::Combo => {
                let choices = s
                    .choices
                    .as_ref()
                    .ok_or(format!("`{key}`: combo without choices"))?;
                let ident = format_ident!("CHOICES_{:02}", self.choice_set(choices)?);
                quote!(Kind::Combo(&#ident))
            }
            Kind::String => {
                let option = |n: Option<u32>| {
                    n.map_or_else(
                        || quote!(None),
                        |n| {
                            let n = Literal::u32_unsuffixed(n);
                            quote!(Some(#n))
                        },
                    )
                };
                let (chars, bytes) = (option(s.max_chars), option(s.max_bytes));
                quote!(Kind::String { max_chars: #chars, max_bytes: #bytes })
            }
        };
        let mut row = quote!(Setting::new(#key, #name, #kind));
        if let Some(heroes) = &s.include {
            let set = Literal::u8_unsuffixed(self.hero_set(heroes)?);
            row = quote!(#row.only(#set));
        }
        if let Some(heroes) = &s.exclude {
            let set = Literal::u8_unsuffixed(self.hero_set(heroes)?);
            row = quote!(#row.except(#set));
        }
        Ok(row)
    }

    fn section(
        &mut self,
        key: &str,
        names: &Names,
        settings: &BTreeMap<String, Setting>,
    ) -> Result<TokenStream, String> {
        let name = Literal::u16_unsuffixed(self.name(names)?);
        let rows = settings
            .iter()
            .map(|(k, s)| self.row(k, s))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(quote!(Section::new(#key, #name, &[ #( #rows ),* ])))
    }
}

pub fn generate(data: &Data) -> Result<Vec<File>, String> {
    let mut tables = Tables::default();
    let settings = settings(data, &mut tables)?;
    let names = tables.names.iter();
    let name_count = tables.names.len();
    Ok(vec![
        (
            "setting_names",
            quote! {
                //! Names shared by all custom game settings, indexed by the settings tables.

                /// Translations joined by U+001F in `Lang` order; empty slots mean English.
                pub static NAMES: [&str; #name_count] = [ #( #names ),* ];
            },
        ),
        ("settings", settings),
    ])
}

fn settings(data: &Data, t: &mut Tables) -> Result<TokenStream, String> {
    let s: &CustomGameSettings = &data.settings;
    let main = t.section("main", &s.main.names, &s.main.values)?;
    let lobby = t.section("lobby", &s.lobby.names, &s.lobby.values)?;

    let mut mode_arms = Vec::new();
    let mut general = None;
    for (key, section) in &s.gamemodes.values {
        let tokens = t.section(key, &section.names, &section.values)?;
        if key == "general" {
            general = Some(tokens);
            continue;
        }
        let variant = format_ident!("{}", super::naming::pascal(key)?);
        mode_arms.push(
            quote! { Gamemode::#variant => { static SECTION: Section = #tokens; Some(&SECTION) } },
        );
    }
    let general = general.ok_or("gamemodes settings have no `general` section")?;

    // Hero sections are emitted separately; registering them here keeps the shared tables stable.
    t.section("general", &s.heroes.teams["allTeams"], &s.heroes.general)?;
    for (key, settings) in &s.heroes.per_hero {
        t.section(key, &data.heroes[key].names, settings)?;
    }
    let mode_fallback = if mode_arms.len() < data.gamemodes.len() {
        quote!(_ => None,)
    } else {
        quote!()
    };
    let labels = labels(s)?;
    let hero_sets = t.hero_sets.iter().map(|set| {
        let heroes = set
            .iter()
            .map(|h| format_ident!("{}", super::naming::pascal(h).unwrap_or_default()));
        quote!(&[ #( Hero::#heroes ),* ])
    });
    let hero_set_count = t.hero_sets.len();
    let choice_sets = &t.choice_sets;
    let settings = quote! {
        //! Custom game settings tables.

        use crate::lang::localized;
        use crate::settings::{Choice, Kind, Section, Setting};
        use crate::{Gamemode, Hero, Lang};

        /// Hero sets used by `Setting::only` and `Setting::except`.
        pub static HERO_SETS: [&[Hero]; #hero_set_count] = [ #( #hero_sets ),* ];

        #( #choice_sets )*

        #labels

        /// The `main` section.
        pub static MAIN: Section = #main;

        /// The `lobby` section.
        pub static LOBBY: Section = #lobby;

        /// Settings shared by all game modes.
        pub static GAMEMODE_GENERAL: Section = #general;

        /// The settings specific to a game mode.
        pub fn gamemode_settings(mode: Gamemode) -> Option<&'static Section> {
            match mode {
                #( #mode_arms )*
                #mode_fallback
            }
        }
    };
    Ok(settings)
}

/// Titles and list labels that aren't settings.
fn labels(s: &CustomGameSettings) -> Result<TokenStream, String> {
    let g = &s.gamemodes;
    let h = &s.heroes;
    let entries: Vec<(&str, &Names)> = vec![
        ("gamemodes", &g.names),
        ("enabled_maps", &g.enabled_maps),
        ("disabled_maps", &g.disabled_maps),
        ("heroes", &h.names),
        ("enabled_heroes", &h.enabled_heroes),
        ("disabled_heroes", &h.disabled_heroes),
        ("team.all", &h.teams["allTeams"]),
        ("team.team1", &h.teams["team1"]),
        ("team.team2", &h.teams["team2"]),
        ("team.ffa", &h.teams["ffa"]),
        ("workshop", &s.workshop.names),
        ("extensions", &s.extensions.names),
    ];
    let vars =
        build(entries.iter().map(|(k, n)| (*k, Some(*n)))).map_err(|e| format!("labels: {e}"))?;
    Ok(enum_def(
        "Label",
        "A title or list label of the settings screen.",
        &vars,
    ))
}
