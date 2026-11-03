use ow_workshop::prelude::*;

fn main() {
    let a: Expr<Bool> = true.into();
    let _ = a.lt(false);
}
