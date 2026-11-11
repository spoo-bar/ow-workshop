use ow_workshop::prelude::*;

fn defs() -> RuleDef<'static> {
    rule("Wrong", OngoingGlobal, |r, _| {
        r.team(TeamFilter::Team1);
    })
}

fn main() {
    let _ = defs;
}
