# Game data

Source data for `cargo xtask codegen`. Schema: spoo-bar/ow-workshop#20 and #25, with the additions below.

Imported once from OverPy (`spoo-bar/overpy@e349a32`, v9.7.20). This repo owns the data now; edit the JSON directly.

| File | Contents |
|---|---|
| `actions.json` | 225 actions |
| `values.json` | 267 values |
| `constants.json` | 42 constant enums + 3 internal (`ModifyOp`, `CompareOp`, `Rounding`) |
| `heroes.json` | 54 heroes |
| `maps.json` | 91 maps |
| `gamemodes.json` | 35 gamemodes |
| `keywords.json` | structural keywords, events, event team/player, settings words |
| `custom_game_settings.json` | settings schema and extensions |
| `localized_strings.json` | 721 phrases accepted by `String()` |
| `string_entities.json`, `textures.json`, `font_widths.json`, `types.json` | strings, tags, font metrics, type tree |

## Additions to the #20 schema

- Absent booleans mean `false`. An absent `replace` means no replacements.
- Functions have `source_key` (the OverPy key), `internal` (compiler-only, e.g. `abortIf`, `setGlobalVariable`) and `doc` (English description; may mention OverPy names).
- A `{"value": ..}` default may carry `args`, e.g. all players is `{"value": "getPlayers", "args": [{"const": ["Team", "ALL"]}]}`.
- Enum names: `Color`, `Button`, `Team` (types keep the `*Literal` variants). Colors with `only_in_tooling` have no names.
- Extra internal types exist in signatures: `CustomStringLiteral`, `LocalizedStringLiteral`, `GlobalVariable`, `PlayerVariable`, `Subroutine`, `Value`, `ModifyOp`, `CompareOp`, `Rounding`, and the number-literal types in `types.json`.
- Settings:
  - `gamemodes.enabled_maps` / `disabled_maps` hold the list labels.
  - `heroes` has `teams`, `enabled_heroes`, `disabled_heroes`, `general` and `per_hero`.
  - A setting may have `include` / `exclude` hero lists.
  - The source has no `min` / `max` ranges.

## Corrections

These defaults were wrong in the source and are corrected here:

- `createIcon.reevaluation` → `IconReeval.VISIBILITY_AND_POSITION`
- `createIcon.color` → `Color.WHITE`
- `getNumberOfHeroes.team` → `Team.ALL`
