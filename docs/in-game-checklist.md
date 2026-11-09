# In-game verification checklist

Some things only the game can confirm. Paste the output into *Custom Game → Settings → Workshop*
(*Paste Rules*, or the clipboard paste button in *Settings*) and check each item.

Generate code with `cargo run -p ow-workshop --example <name>`; add `--lang fr-FR` or
`--lang ja-JP` for another client language. The code goes to stdout, diagnostics to stderr.
Copy stdout only.

| Example | What it covers |
|---|---|
| `hello_world` | the smallest mod, with a settings block |
| `zombie_survival` | variables, subroutines, switch, HUD, a larger element count |
| `custom_settings` | settings block, workshop settings, French translations |
| `in_game_checklist` | the long string, tags, cased text and the text helpers |

## Checks

- [ ] `hello_world`, `zombie_survival` and `custom_settings` paste without errors (en-US).
- [ ] `hello_world` pastes in a French and a Japanese client (`--lang fr-FR`, `--lang ja-JP`).
- [ ] The element count matches the in-game counter for `zombie_survival`. The compiler's count is
  `CompileOutput::element_count`, or run with `CompileOptions { debug_element_count: true, .. }`
  for a summary comment at the top of the output.
- [ ] A 300-character string with 7 arguments displays correctly: the first HUD text of
  `in_game_checklist` reads *Round … of the match … word31 …* with the seven live numbers.
- [ ] `<fg>` color and `<tx>` texture tags render: the second HUD text shows *Red* in red and
  the ultimate icon.
- [ ] Cased progress-bar text renders lowercase: the *Lowercase* rule shows `lowercase text`
  above the map origin, in lowercase.
- [ ] The translated HUD shows French for a French client, in both modes
  (`custom_settings`, with the French catalog and with the player-variable mode).
- [ ] `array_to_string`, `time_to_string` and `button_to_string` display as expected: the last
  two HUD texts of `in_game_checklist` show `[1, 2, 3]`, the match time as `m:ss` and the
  jump key's name.
- [ ] The settings block applies: slots, maps, hero restrictions and extensions (`custom_settings`
  after pasting, open *Settings → Maps / Heroes / Extensions*).

## Results

One row per run. Fill in the checks that were run and note anything that did not match.

| Date | Game version | Tester | Checks passed | Notes |
|---|---|---|---|---|
| _not run yet_ | | | | |
