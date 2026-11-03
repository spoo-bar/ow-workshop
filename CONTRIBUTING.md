# Contributing

Design and conventions: [#30](https://github.com/spoo-bar/ow-workshop/issues/30).

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
- Regenerate data tables with `cargo xtask codegen`.
