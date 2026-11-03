use ow_workshop::prelude::*;

#[derive(Clone, Copy, WorkshopEnum)]
enum Bad {
    #[ow(value = Team::One)]
    A,
}

fn main() {}
