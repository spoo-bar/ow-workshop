use ow_workshop::prelude::*;

#[derive(GlobalVars)]
struct Globals {
    list: Global<Array<Num>>,
}

fn main() {
    let mut reg = VarRegistry::default();
    let g = Globals::declare(&mut reg);
    let mut b = Block::default();
    b.chase_at_rate(g.list.at(0), 1, 1, ChaseRateReeval::DestinationAndRate);
}
