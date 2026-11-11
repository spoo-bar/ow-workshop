# ow-workshop

Write Overwatch Workshop mods in Rust.
Implement `Workshop` for a type and compile it to the text you paste into *Custom Game → Settings → Workshop*.

```rust
use ow_workshop::prelude::*;
use ow_workshop::{Gamemode, Map};

#[derive(GlobalVars)]
struct Globals { round: Global<Num> }

#[derive(PlayerVars)]
struct Players { score: PlayerVar<Num> }

struct Deathmatch;

impl Workshop for Deathmatch {
    const NAME: &'static str = "My Mode";
    const DESCRIPTION: &'static str = "Made with ow-workshop";

    type Globals = Globals;
    type Players = Players;

    fn rules<'a>(&'a self, g: &'a Globals, p: &'a Players) -> Vec<RuleDef<'a>> {
        vec![
            rule("Init", OngoingGlobal, |r, _| {
                r.actions(|b| b.set(g.round, 1));
            }),
            rule("Score on kill", PlayerDied, |r, ev| {
                r.condition(ev.attacker().ne(ev.victim()));
                r.actions(|b| b.add_to(ev.attacker().var(p.score), 1));
            }),
        ]
    }

    fn settings(&self) -> Settings {
        let maps = MapSelection::Enabled(vec![Map::WorkshopChamber.into()]);
        Settings::default()
            .with_mode(
                ModeKey::Mode(Gamemode::TeamDeathmatch),
                ModeSettings::default().maps(maps),
            )
    }
}
```

## Examples

Run it with `cargo run -p ow-workshop --release --example hello_world` to get the workshop code as is.
Without `--release` it is a debug build: the code also shows a live variable panel in game, and a
debug log is printed to stderr (see "Debugging" in the crate docs):

- [`hello_world`](ow-workshop/examples/hello_world.rs): the smallest useful mod.

See [CONTRIBUTING.md](CONTRIBUTING.md) to get involved. Licensed under GPL-3.0.
