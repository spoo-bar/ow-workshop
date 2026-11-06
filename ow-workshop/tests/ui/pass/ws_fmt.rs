use ow_workshop::prelude::*;

fn main() {
    let n = Expr::<Num>::from(3);
    let _: Expr<Str> = ws_fmt!("{n} / {}", 1.5);
}
