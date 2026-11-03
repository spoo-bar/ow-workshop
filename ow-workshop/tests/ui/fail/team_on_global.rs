use ow_workshop::prelude::*;

fn build(ws: &mut Builder) {
    ws.rule("Wrong", OngoingGlobal, |r, _| {
        r.team(TeamFilter::Team1);
    });
}

fn main() {
    let _ = build;
}
