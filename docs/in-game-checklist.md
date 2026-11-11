# In-game verification checklist

Some things only the game can confirm. Paste the output into *Custom Game → Settings → Workshop*
(*Paste Rules*, or the clipboard paste button in *Settings*) and check each item.

Generate code with `cargo run -p ow-workshop --release --example <name>`; add `--lang fr-FR` or
`--lang ja-JP` for another client language. The code goes to stdout, diagnostics to stderr.
Copy stdout only.

Without `--release` the example is a debug build: the code starts with `// ow-workshop debug build`
and adds a live variable panel, and a debug log is printed to stderr. Use `--release` for every
check below except the debug ones.

| Example | What it covers |
|---|---|
| `hello_world` | the smallest mod, with a settings block |

## Checks

- [ ] `hello_world` pastes without errors (en-US).
- [ ] `hello_world` pastes in a French and a Japanese client (`--lang fr-FR`, `--lang ja-JP`).

Debug builds:

- [ ] A debug paste works: `cargo run -p ow-workshop --example hello_world` (no `--release`)
  starts with `// ow-workshop debug build` and pastes without errors.
- [ ] A release paste contains no debug output: `cargo run -p ow-workshop --release --example
  hello_world` has no `Debug:` rules and no `// ow-workshop debug build` line.

## Results

One row per run. Fill in the checks that were run and note anything that did not match.

| Date | Game version | Tester | Checks passed | Notes |
|---|---|---|---|---|
| _not run yet_ | | | | |
