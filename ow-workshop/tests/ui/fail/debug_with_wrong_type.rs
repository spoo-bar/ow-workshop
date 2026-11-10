use ow_workshop::prelude::*;

fn render_bool(value: Expr<Bool>) -> Expr<Str> {
    ws_fmt!("{}", value)
}

#[derive(GlobalVars)]
struct Globals {
    #[ow(debug_with = render_bool)]
    wave: Global<Num>,
}

fn main() {}
