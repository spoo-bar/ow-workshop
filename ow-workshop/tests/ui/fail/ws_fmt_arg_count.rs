use ow_workshop::prelude::*;

fn main() {
    let _ = ws_fmt!("{} and {}", 1);
    let _ = ws_fmt!("{}", 1, 2);
    let _ = ws_fmt!("{} {0}", 1);
    let _ = ws_fmt!("{:?}", 1);
}
