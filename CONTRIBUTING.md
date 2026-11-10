# Contributing

## Design
- Typed values (`Expr<T>`); comparisons are methods (`eq`, `lt`, ...), not `PartialEq`.
- Statements are recorded on an explicit `&mut Block`, never through hidden state.
- Optional arguments use an args struct with `Default`, not long positional lists.
- No preprocessor: reuse is plain Rust (functions, extension traits, modules).
- Every public builder method that creates IR is `#[track_caller]` so diagnostics point at the user's source line. Closures break the chain: capture `Location::caller()` at the outermost public call and pass it down.

## Rust style
- Idiomatic and clippy-clean (workspace lints). No `unsafe`.
- No `unwrap`/`expect` outside tests.
- Keep modules small.

## Comments
- One-line `///` doc on every public item.
- Inline comments only for non-obvious edge cases (usually workshop quirks). No long prose.

## Architecture
- No global or thread-local mutable state.
- The core does no I/O and no printing.
- `ow-workshop` stays WASM-safe: no `std::fs`, `std::time`, `std::process`, threads or `getrandom`.

## Workflow
- PR titles follow [Conventional Commits](https://www.conventionalcommits.org/) (checked by `lint-pr.yml`).
- Keep PRs under ~1000 changed lines.
- Before pushing: `cargo fmt`, `cargo clippy --workspace --all-targets`, `cargo test --workspace`.

## Game data
The Rust tables are the source of truth; edit them directly when the game updates.
- Hero: add a variant and its entry in `ow-workshop-data/src/generated/heroes.rs`, plus any hero settings in `hero_settings.rs`.
- Map: add it in `ow-workshop-data/src/generated/maps.rs` (and `gamemodes.rs` if it belongs to a mode).
- Action: add it in `ow-workshop-data/src/generated/actions.rs`, then the typed builder in `ow-workshop/src/generated/actions.rs`.
- Add a test in the matching `ow-workshop-data/tests/` file, then run `cargo fmt`, `cargo clippy --workspace --all-targets` and `cargo test --workspace`.
