use ow_workshop::prelude::*;

#[derive(GlobalVars)]
struct Globals {
    hyper: Global<Array<Array<Array<Array<Num>>>>>,
}

fn main() {
    let mut reg = VarRegistry::default();
    let g = Globals::declare(&mut reg);
    let _ = g.hyper.at(0).at(0).at(0).at(0);
}
