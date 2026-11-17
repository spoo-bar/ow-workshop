# Contributing

The rewrite is tracked in [the epic](https://github.com/spoo-bar/ow-workshop/issues/244). Read its design section before picking up a sub-issue.

## Design
- Root types are traits (`Workshop`, `Rule`, `Subroutine`, `Globals`, `Players`), each with a default concrete type.
- `Event`, `Condition`, `Action` and `Value` are plain data. No closures, builder state, global or thread-local state.
- A rule must not refer to a particular `Workshop`, so it can be shared between mods. Shared rules go in `ow_workshop::utils` behind the `utils` feature.
- The build profile decides the output. Debug keeps everything and adds helpers. Release keeps only what runs.
- Output is English only.

## Rust style
- Idiomatic and clippy-clean (workspace lints). No `unsafe`.
- Keep modules small.
- One-line `///` doc on every public item. Other comments only for a non-obvious reason.

## Workflow
- PR titles follow [Conventional Commits](https://www.conventionalcommits.org/) (checked by `lint-pr.yml`).
- Keep PRs under about 1000 changed lines.
- Before pushing: `cargo fmt --all`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo test --workspace --all-features`.

## Game data
The tables in `ow-workshop-data` are the source of truth for spellings. Edit them directly when the game updates.
- Hero: `ow-workshop-data/src/generated/heroes.rs`, plus hero settings in `hero_settings.rs`.
- Map: `ow-workshop-data/src/generated/maps.rs`, and `gamemodes.rs` if it belongs to a mode.
- Action or value: `ow-workshop-data/src/generated/actions.rs` or `values.rs`. The `ow_workshop::values` constructors are generated from `values.rs` by `ow-workshop/build.rs`, so a new value needs no hand-written code.

## Golden tests
`ow-workshop-golden` compares a generated release script to a reference script after normalizing it (`assert_same_script`). The rules are documented in `ow-workshop-golden/src/normalize.rs`.

Some reference scripts have no licence file, so they are not vendored here (this repo is GPL-3.0-only). Tests that use them read them through `reference_file` and are skipped when `OW_REFERENCE_DIR` is unset. To run them, check out the reference repository listed in the epic and point the variable at it:

```bash
OW_REFERENCE_DIR=/path/to/reference-checkout cargo test -p ow-workshop-golden
```

The two owner-provided samples in `examples/samples/` are used directly.
