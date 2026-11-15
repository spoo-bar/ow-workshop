//! Writes every example mod to `target/<profile>/<name>.owmod`.

#[path = "src/hello/mod.rs"]
mod hello;

fn main() {
    ow_workshop::build::emit_as::<hello::Hello>("hello");
}
