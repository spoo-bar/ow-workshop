# ow-workshop

Write Overwatch Workshop mods in idiomatic Rust. Implement `Workshop` for a type and compile it to the text you paste into *Custom Game → Settings → Workshop*. The core is WASM-safe and does no I/O.

```rust
use ow_workshop::prelude::*;

#[derive(GlobalVars)]
struct Globals { round: Global<Num> }

#[derive(PlayerVars)]
struct Players { score: PlayerVar<Num> }

struct Deathmatch;

impl Workshop for Deathmatch {
    type Globals = Globals;
    type Players = Players;

    fn build(&self, ws: &mut Builder, g: &Globals, p: &Players) {
        ws.rule("Init", OngoingGlobal, |r, _| {
            r.actions(|b| b.set(g.round, 1));
        });

        ws.rule("Score on kill", PlayerDied, |r, ev| {
            r.condition(ev.attacker().ne(ev.victim()));
            r.actions(|b| b.add_to(ev.attacker().var(p.score), 1));
        });
    }
}
```

*The API is under construction; this sample shows the target design.*

See [CONTRIBUTING.md](CONTRIBUTING.md) to get involved. Licensed under GPL-3.0.
