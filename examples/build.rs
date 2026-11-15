//! Writes every example mod to `target/<profile>/<name>.owmod`.

#[path = "src/hello/mod.rs"]
mod hello;
#[path = "src/settings_only/mod.rs"]
mod settings_only;

fn main() {
    ow_workshop::build::emit_as::<hello::Hello>("hello");
    ow_workshop::build::emit_as::<settings_only::SettingsOnly>("settings_only");
}
