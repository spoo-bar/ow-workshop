//! Typed mirror of `data/*.json` (see `data/SCHEMA.md`).

use std::collections::BTreeMap;

use serde::Deserialize;

/// Localized names keyed by language code; `en-US` is mandatory.
pub type Names = BTreeMap<String, String>;

/// The 15 language codes workshop code can be written in.
pub const LANGS: [&str; 15] = [
    "de-DE", "en-US", "es-ES", "es-MX", "fr-FR", "it-IT", "ja-JP", "ko-KR", "pl-PL", "pt-BR",
    "ru-RU", "th-TH", "tr-TR", "zh-CN", "zh-TW",
];

/// A type in a signature.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Ty {
    Named(String),
    Array {
        #[serde(rename = "Array")]
        array: Box<Ty>,
    },
    Union(Vec<Ty>),
    Vector(VectorTy),
}

/// A vector-like type with per-component types.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VectorTy {
    #[serde(rename = "Vector")]
    pub vector: Option<[Box<Ty>; 3]>,
    #[serde(rename = "Direction")]
    pub direction: Option<[Box<Ty>; 3]>,
    #[serde(rename = "Position")]
    pub position: Option<[Box<Ty>; 3]>,
    #[serde(rename = "Velocity")]
    pub velocity: Option<[Box<Ty>; 3]>,
}

/// The `"null"` default.
#[derive(Debug, Deserialize)]
pub enum Null {
    #[serde(rename = "null")]
    Null,
}

/// A default argument value.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Default {
    Null(Null),
    Num {
        num: f64,
    },
    Bool {
        bool: bool,
    },
    Const {
        r#const: (String, String),
    },
    Value {
        value: String,
        #[serde(default)]
        args: Vec<Default>,
    },
    Vector {
        vector: [f64; 3],
    },
}

/// Literal replacements allowed in optimize-for-size mode.
#[allow(clippy::struct_excessive_bools, reason = "mirrors the JSON schema")]
#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Replace {
    #[serde(default)]
    pub zero_by_false: bool,
    #[serde(default)]
    pub zero_by_null: bool,
    #[serde(default)]
    pub one_by_true: bool,
    #[serde(default)]
    pub null_vector_by_null: bool,
    #[serde(default)]
    pub empty_string_by_empty_array: bool,
}

/// An action or value argument.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Arg {
    pub name: String,
    pub ty: Ty,
    pub default: Option<Default>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub literal_max: Option<f64>,
    pub replace: Option<Replace>,
}

/// An action or value signature.
#[allow(clippy::struct_excessive_bools, reason = "mirrors the JSON schema")]
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Function {
    pub key: String,
    pub source_key: String,
    #[serde(default)]
    pub internal: bool,
    pub names: Names,
    pub doc: Option<String>,
    pub args: Vec<Arg>,
    pub ret: Ty,
    #[serde(default)]
    pub member: bool,
    #[serde(default)]
    pub has_literal_limit: bool,
    #[serde(default)]
    pub is_constant: bool,
    #[serde(default)]
    pub is_literal: bool,
    #[serde(default)]
    pub can_be_put_in_boolean: bool,
}

/// One member of a constant enum.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Member {
    pub names: Option<Names>,
    pub extension: Option<String>,
    pub rgba: Option<[u8; 4]>,
    #[serde(default)]
    pub only_in_tooling: bool,
    pub doc: Option<String>,
}

/// `constants.json`: enum name to members.
pub type Constants = BTreeMap<String, BTreeMap<String, Member>>;

/// A hero.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Hero {
    pub names: Names,
    #[serde(default)]
    pub abilities: BTreeMap<String, Names>,
}

/// A map.
#[allow(clippy::struct_field_names, reason = "mirrors the JSON schema")]
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Map {
    pub names: Names,
    #[serde(default)]
    pub gamemodes: Vec<String>,
    pub variants: Option<BTreeMap<String, String>>,
    #[serde(default)]
    pub is_workshop_map: bool,
    #[serde(default)]
    pub only_in_ow1: bool,
    pub symmetry: Option<serde_json::Value>,
}

/// A gamemode.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Gamemode {
    pub names: Names,
    pub default_team1_slots: Option<u32>,
    pub default_team2_slots: Option<u32>,
    pub default_ffa_slots: Option<u32>,
    #[serde(default)]
    pub only_in_ow1: bool,
}

/// `keywords.json`: structural words by group.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Keywords {
    pub structure: BTreeMap<String, Names>,
    pub event: BTreeMap<String, Names>,
    pub event_team: BTreeMap<String, Names>,
    pub event_player: BTreeMap<String, Names>,
    pub settings: BTreeMap<String, Names>,
}

/// A phrase accepted by the workshop `String()` value.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocalizedString {
    pub names: Names,
}

/// Value kind of a custom game setting.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    BoolYesNo,
    BoolEnabled,
    BoolReverseEnabled,
    BoolOnOff,
    BoolReverseOnOff,
    Combo,
    Percent,
    Int,
    Float,
    String,
}

/// A custom game setting.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Setting {
    pub names: Names,
    pub kind: Kind,
    pub choices: Option<BTreeMap<String, Names>>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub max_chars: Option<u32>,
    pub max_bytes: Option<u32>,
    pub default: Option<serde_json::Value>,
    pub include: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
}

/// A named group of settings.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Section {
    pub names: Names,
    pub values: BTreeMap<String, Setting>,
}

/// The `gamemodes` settings section.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GamemodeSettings {
    pub names: Names,
    pub enabled_maps: Names,
    pub disabled_maps: Names,
    pub values: BTreeMap<String, ModeSection>,
}

/// One gamemode's settings, with its default slot counts.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModeSection {
    pub names: Names,
    pub values: BTreeMap<String, Setting>,
    pub default_team1_slots: Option<u32>,
    pub default_team2_slots: Option<u32>,
    pub default_ffa_slots: Option<u32>,
}

/// The `heroes` settings section.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HeroSettings {
    pub names: Names,
    pub teams: BTreeMap<String, Names>,
    pub enabled_heroes: Names,
    pub disabled_heroes: Names,
    pub general: BTreeMap<String, Setting>,
    pub per_hero: BTreeMap<String, BTreeMap<String, Setting>>,
}

/// A workshop extension.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Extension {
    pub names: Names,
    pub points: u32,
}

/// The `extensions` settings section.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExtensionSettings {
    pub names: Names,
    pub values: BTreeMap<String, Extension>,
}

/// The `workshop` settings section (only a title).
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkshopSettings {
    pub names: Names,
}

/// `custom_game_settings.json`.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CustomGameSettings {
    pub main: Section,
    pub lobby: Section,
    pub gamemodes: GamemodeSettings,
    pub heroes: HeroSettings,
    pub workshop: WorkshopSettings,
    pub extensions: ExtensionSettings,
}

/// A node of the type tree: a leaf or a parent with children.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum TypeNode {
    Leaf(String),
    Parent(BTreeMap<String, Vec<TypeNode>>),
}

/// `font_widths.json`.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FontWidths {
    pub default: u32,
    pub widths: BTreeMap<String, u32>,
    pub spaces: BTreeMap<String, String>,
    pub lowercase: BTreeMap<String, Lowercase>,
}

/// Glyph metrics of a lowercase letter.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Lowercase {
    pub lower: String,
    pub width: u32,
    pub lower_width: u32,
    pub xmin: i32,
    pub lower_xmin: i32,
}
