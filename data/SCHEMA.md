# Data schema

Checked by `cargo xtask validate-data [dir]` (default `data`, plus `data/sample` in CI).

**Names.** Every localized name is `"names": { "en-US": "...", "fr-FR": "...", ... }` over 15 language codes
(`de-DE en-US es-ES es-MX fr-FR it-IT ja-JP ko-KR pl-PL pt-BR ru-RU th-TH tr-TR zh-CN zh-TW`).
`en-US` is mandatory; other languages fall back to it.

**Types (`Ty`)** are JSON values:
- a name: a primitive (`"float"`, `"int"`, `"bool"`, `"String"`, `"Player"`, `"Vector"`, `"void"`, …), an id type, a constant enum, or a `*Literal` variant of a known type
- `{ "Array": Ty }`
- `[Ty, ...]`: any of
- `{ "Vector": [Ty, Ty, Ty] }` (likewise `Direction`, `Position`, `Velocity`): per-component types

`types.json` is the subtype tree: a list of roots, each a name or `{ "<parent>": [children] }`.

**Defaults:** `{"num": 0}`, `{"bool": true}`, `"null"`, `{"const": ["Team", "ALL"]}`, `{"value": "<key>", "args": [Default]}`, `{"vector": [0, 0, 0]}`.

Absent booleans mean `false`; an absent `replace` means no replacements.

| File | Shape |
|---|---|
| `actions.json`, `values.json` | `[{ key, source_key, internal?, member?, names, doc?, args: [{ name, ty, default?, min?, max?, literal_max?, replace? }], ret, has_literal_limit?, is_constant?, is_literal?, can_be_put_in_boolean? }]`. `replace` has `zero_by_false`, `zero_by_null`, `one_by_true`, `null_vector_by_null`, `empty_string_by_empty_array`. `member` means the first arg is the receiver. |
| `constants.json` | `{ "<Enum>": { "<MEMBER>": { names, extension?, rgba?, only_in_tooling?, doc? } } }`. `only_in_tooling` members need no names. |
| `heroes.json` | `{ "<key>": { names, abilities: { "<ability>": names } } }` |
| `maps.json` | `{ "<key>": { names, gamemodes, variants?, is_workshop_map?, only_in_ow1?, symmetry? } }` |
| `gamemodes.json` | `{ "<key>": { names, default_team1_slots?, default_team2_slots?, default_ffa_slots?, only_in_ow1? } }` |
| `keywords.json` | `{ structure, event, event_team, event_player, settings }`, each `{ "<key>": names }` |
| `custom_game_settings.json` | `main`, `lobby`: `{ names, values: { "<key>": Setting } }`; `gamemodes`: `{ names, enabled_maps, disabled_maps, values: { "<mode>": { names, values, default_*_slots? } } }`; `heroes`: `{ names, teams, enabled_heroes, disabled_heroes, general: { "<key>": Setting }, per_hero: { "<hero>": { "<key>": Setting } } }`; `workshop`: `{ names }`; `extensions`: `{ names, values: { "<key>": { names, points } } }` |
| `localized_strings.json` | `[{ names }]` |
| `string_entities.json` | `{ "<name>": <codepoint> }` |
| `textures.json` | `{ "<KEY>": "<tx tag>" }` |
| `font_widths.json` | `{ default, widths: { "<char>": w }, spaces: { "<w>": "<char>" }, lowercase: { "<char>": { lower, width, lower_width, xmin, lower_xmin } } }` |

**Setting** (`custom_game_settings.json`): `{ names, kind, choices?, min?, max?, max_chars?, max_bytes?, default?, include?, exclude? }`.
`kind` is one of `bool_yes_no`, `bool_enabled`, `bool_reverse_enabled`, `bool_on_off`, `bool_reverse_on_off`, `combo`, `percent`, `int`, `float`, `string`.
`include` / `exclude` list the hero keys a setting applies to.
