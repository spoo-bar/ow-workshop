//! `cargo xtask validate-data`: typed parse and consistency checks of `data/*.json`.

#[allow(dead_code, reason = "fields are read by the codegen that follows")]
mod model;

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use model::{
    Constants, CustomGameSettings, Default, FontWidths, Function, Gamemode, Hero, Keywords, LANGS,
    LocalizedString, Map, Names, Setting, Ty, TypeNode,
};
use serde::de::DeserializeOwned;

/// Types that appear in signatures but not in `types.json` or `constants.json`.
const INTERNAL_TYPES: [&str; 6] = [
    "CustomStringLiteral",
    "GlobalVariable",
    "LocalizedStringLiteral",
    "PlayerVariable",
    "Subroutine",
    "Value",
];

/// Parses every data file in `dir` and checks cross-references.
pub fn validate_dir(dir: &Path) -> Result<(), String> {
    let actions: Vec<Function> = load(dir, "actions.json")?;
    let values: Vec<Function> = load(dir, "values.json")?;
    let constants: Constants = load(dir, "constants.json")?;
    let heroes: BTreeMap<String, Hero> = load(dir, "heroes.json")?;
    let maps: BTreeMap<String, Map> = load(dir, "maps.json")?;
    let gamemodes: BTreeMap<String, Gamemode> = load(dir, "gamemodes.json")?;
    let keywords: Keywords = load(dir, "keywords.json")?;
    let settings: CustomGameSettings = load(dir, "custom_game_settings.json")?;
    let strings: Vec<LocalizedString> = load(dir, "localized_strings.json")?;
    let _: BTreeMap<String, u32> = load(dir, "string_entities.json")?;
    let _: BTreeMap<String, String> = load(dir, "textures.json")?;
    let fonts: FontWidths = load(dir, "font_widths.json")?;
    let tree: Vec<TypeNode> = load(dir, "types.json")?;

    let mut problems = Vec::new();
    let mut check = Checker {
        problems: &mut problems,
    };
    let mut known: BTreeSet<&str> = INTERNAL_TYPES.into_iter().collect();
    known.extend(constants.keys().map(String::as_str));
    for node in &tree {
        collect_type_names(node, &mut known);
    }

    for function in actions.iter().chain(&values) {
        let at = format!("function `{}`", function.key);
        check.names(&at, &function.names);
        check.ty(&at, &function.ret, &known);
        for arg in &function.args {
            let at = format!("{at}, arg `{}`", arg.name);
            check.ty(&at, &arg.ty, &known);
            if let Some(default) = &arg.default {
                check.default(&at, default, &constants);
            }
        }
    }
    check.unique(
        "function key",
        actions.iter().chain(&values).map(|f| f.key.as_str()),
    );

    check.constants(&constants, &settings);
    check.world(&heroes, &maps, &gamemodes);

    for (group, words) in [
        ("structure", &keywords.structure),
        ("event", &keywords.event),
        ("event_team", &keywords.event_team),
        ("event_player", &keywords.event_player),
        ("settings", &keywords.settings),
    ] {
        for (key, names) in words {
            check.names(&format!("keyword `{group}.{key}`"), names);
        }
    }
    for (index, string) in strings.iter().enumerate() {
        check.names(&format!("localized string #{index}"), &string.names);
    }
    check.settings(&settings, &heroes);

    for (ch, lowercase) in &fonts.lowercase {
        if ch.chars().count() != 1 || lowercase.lower.chars().count() != 1 {
            check.problem(format!("font lowercase `{ch}`: expected single characters"));
        }
    }

    if problems.is_empty() {
        println!(
            "{}: ok ({} actions, {} values, {} enums, {} heroes, {} maps, {} gamemodes)",
            dir.display(),
            actions.len(),
            values.len(),
            constants.len(),
            heroes.len(),
            maps.len(),
            gamemodes.len(),
        );
        Ok(())
    } else {
        let count = problems.len();
        problems.truncate(50);
        Err(format!(
            "{count} problem(s) in {}:\n  {}",
            dir.display(),
            problems.join("\n  ")
        ))
    }
}

fn collect_type_names<'a>(node: &'a TypeNode, out: &mut BTreeSet<&'a str>) {
    match node {
        TypeNode::Leaf(name) => {
            out.insert(name);
        }
        TypeNode::Parent(children) => {
            for (name, nodes) in children {
                out.insert(name);
                for child in nodes {
                    collect_type_names(child, out);
                }
            }
        }
    }
}

fn load<T: DeserializeOwned>(dir: &Path, file: &str) -> Result<T, String> {
    let path = dir.join(file);
    let text = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))
}

struct Checker<'a> {
    problems: &'a mut Vec<String>,
}

impl Checker<'_> {
    fn problem(&mut self, message: String) {
        self.problems.push(message);
    }

    fn names(&mut self, at: &str, names: &Names) {
        if !names.contains_key("en-US") {
            self.problem(format!("{at}: missing `en-US` name"));
        }
        for lang in names.keys().filter(|l| !LANGS.contains(&l.as_str())) {
            self.problem(format!("{at}: unknown language `{lang}`"));
        }
    }

    fn ty(&mut self, at: &str, ty: &Ty, known: &BTreeSet<&str>) {
        match ty {
            Ty::Named(name) => {
                let base = name.strip_suffix("Literal").unwrap_or(name);
                if !known.contains(name.as_str()) && !known.contains(base) {
                    self.problem(format!("{at}: unknown type `{name}`"));
                }
            }
            Ty::Array { array } => self.ty(at, array, known),
            Ty::Union(types) => types.iter().for_each(|t| self.ty(at, t, known)),
            Ty::Vector(v) => {
                let parts = [&v.vector, &v.direction, &v.position, &v.velocity];
                for part in parts.into_iter().flatten().flatten() {
                    self.ty(at, part, known);
                }
            }
        }
    }

    fn default(&mut self, at: &str, default: &Default, constants: &Constants) {
        match default {
            Default::Const {
                r#const: (name, member),
            } => {
                if !constants.get(name).is_some_and(|e| e.contains_key(member)) {
                    self.problem(format!("{at}: unknown default constant `{name}.{member}`"));
                }
            }
            Default::Value { args, .. } => {
                for arg in args {
                    self.default(at, arg, constants);
                }
            }
            _ => {}
        }
    }

    fn constants(&mut self, constants: &Constants, settings: &CustomGameSettings) {
        for (enum_name, members) in constants {
            for (member, data) in members {
                let at = format!("constant `{enum_name}.{member}`");
                // Tooling-only colors are not workshop values, so they need no translations.
                match &data.names {
                    Some(names) if !data.only_in_tooling => self.names(&at, names),
                    Some(_) => {}
                    None if data.only_in_tooling => {}
                    None => self.problem(format!("{at}: missing names")),
                }
                if let Some(extension) = &data.extension {
                    if !settings.extensions.values.contains_key(extension) {
                        self.problem(format!("{at}: unknown extension `{extension}`"));
                    }
                }
            }
        }
    }

    fn world(
        &mut self,
        heroes: &BTreeMap<String, Hero>,
        maps: &BTreeMap<String, Map>,
        gamemodes: &BTreeMap<String, Gamemode>,
    ) {
        for (key, hero) in heroes {
            let at = format!("hero `{key}`");
            self.names(&at, &hero.names);
            for (ability, names) in &hero.abilities {
                self.names(&format!("{at}, ability `{ability}`"), names);
            }
        }
        for (key, map) in maps {
            let at = format!("map `{key}`");
            self.names(&at, &map.names);
            for mode in &map.gamemodes {
                if !gamemodes.contains_key(mode) {
                    self.problem(format!("{at}: unknown gamemode `{mode}`"));
                }
            }
        }
        for (key, mode) in gamemodes {
            self.names(&format!("gamemode `{key}`"), &mode.names);
        }
    }

    fn unique<'k>(&mut self, what: &str, keys: impl Iterator<Item = &'k str>) {
        let mut seen = BTreeSet::new();
        for key in keys {
            if !seen.insert(key) {
                self.problem(format!("duplicate {what} `{key}`"));
            }
        }
    }

    fn setting(&mut self, at: &str, setting: &Setting, heroes: &BTreeMap<String, Hero>) {
        self.names(at, &setting.names);
        for (key, names) in setting.choices.iter().flatten() {
            self.names(&format!("{at}, choice `{key}`"), names);
        }
        for hero in setting.include.iter().chain(&setting.exclude).flatten() {
            if !heroes.contains_key(hero) {
                self.problem(format!("{at}: unknown hero `{hero}`"));
            }
        }
    }

    fn settings(&mut self, s: &CustomGameSettings, heroes: &BTreeMap<String, Hero>) {
        for (name, section) in [("main", &s.main), ("lobby", &s.lobby)] {
            self.names(name, &section.names);
            for (key, setting) in &section.values {
                self.setting(&format!("settings `{name}.{key}`"), setting, heroes);
            }
        }
        let g = &s.gamemodes;
        for names in [&g.names, &g.enabled_maps, &g.disabled_maps] {
            self.names("settings `gamemodes`", names);
        }
        for (mode, section) in &g.values {
            self.names(&format!("settings `gamemodes.{mode}`"), &section.names);
            for (key, setting) in &section.values {
                self.setting(
                    &format!("settings `gamemodes.{mode}.{key}`"),
                    setting,
                    heroes,
                );
            }
        }
        let h = &s.heroes;
        for names in [&h.names, &h.enabled_heroes, &h.disabled_heroes]
            .into_iter()
            .chain(h.teams.values())
        {
            self.names("settings `heroes`", names);
        }
        for (key, setting) in &h.general {
            self.setting(&format!("settings `heroes.general.{key}`"), setting, heroes);
        }
        for (hero, settings) in &h.per_hero {
            if !heroes.contains_key(hero) {
                self.problem(format!("settings `heroes.{hero}`: unknown hero"));
            }
            for (key, setting) in settings {
                self.setting(&format!("settings `heroes.{hero}.{key}`"), setting, heroes);
            }
        }
        self.names("settings `workshop`", &s.workshop.names);
        self.names("settings `extensions`", &s.extensions.names);
        for (key, extension) in &s.extensions.values {
            self.names(&format!("extension `{key}`"), &extension.names);
        }
    }
}
