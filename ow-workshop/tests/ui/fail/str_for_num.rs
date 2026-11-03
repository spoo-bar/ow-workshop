use ow_workshop::prelude::*;

fn takes_num(_: impl IntoExpr<Num>) {}

fn main() {
    let s: Expr<Str> = "hello".into();
    takes_num(s);
}
