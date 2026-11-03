use ow_workshop::prelude::*;

fn main() {
    let a: Expr<Str> = "a".into();
    let b: Expr<Str> = "b".into();
    let _ = a + b;
}
