# ow-workshop

Write Overwatch Workshop mods in Rust. `cargo build` writes the script you paste into *Custom Game → Settings → Workshop*.

The library is being rewritten. [The epic](https://github.com/spoo-bar/ow-workshop/issues/244) lists what is done and what is open.

## A mod

A mod is one type implementing `Workshop`. Its rules are types implementing `Rule`, in any file or crate.

```rust
use ow_workshop::{Hero, Lobby, Mode, Rule, Workshop, utils};

#[derive(Default)]
pub struct MyMod;

impl Workshop for MyMod {
    const NAME: &'static str = "My Mode";
    const DESCRIPTION: &'static str = "Made with ow-workshop";

    type Globals = ();
    type Players = ();

    fn lobby() -> Lobby {
        Lobby::new().set("Max Team 1 Players", 5)
    }

    fn modes() -> Vec<Mode> {
        vec![Mode::new("Team Deathmatch").enabled_maps(["Workshop Island"])]
    }

    fn heroes() -> Vec<Hero> {
        Vec::new()
    }

    fn rules(&self, _: &(), _: &()) -> Vec<Box<dyn Rule>> {
        vec![Box::new(utils::DisableDamageAndHealing)]
    }
}
```

The mod crate's `build.rs` pulls the mod in and writes the script:

```rust
#[path = "src/my_mod/mod.rs"]
mod my_mod;

fn main() {
    ow_workshop::build::emit::<my_mod::MyMod>();
}
```

`ow-workshop` goes in both `[dependencies]` and `[build-dependencies]`.

## Building

```bash
cargo build -p my-mod
```

writes `target/debug/my-mod.owmod` with every rule (disabled ones as `disabled rule`), comments, and debug helpers: a HUD line for each variable marked `#[debug(...)]`, and an inspector log line for each rule marked `#[debug(log_to_inspector)]`.

```bash
cargo build -p my-mod --release
```

writes `target/release/my-mod.owmod` with only what the mod needs to run.

## Example

[`examples/src/hello`](examples/src/hello) is a small mod spread over three files. `cargo build -p examples` writes `target/debug/hello.owmod`.

See [CONTRIBUTING.md](CONTRIBUTING.md) to get involved. Licensed under GPL-3.0.
