use ow_workshop::prelude::*;

fn defs() -> RuleDef<'static> {
    rule("Wrong", OngoingEachPlayer, |r, ev| {
        r.condition(ev.attacker().ne(ev.event_player()));
    })
}

fn main() {
    let _ = defs;
}
