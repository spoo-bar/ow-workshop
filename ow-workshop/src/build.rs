//! Writes a mod's script from its `build.rs`.
//!
//! ```ignore
//! // build.rs of the mod crate
//! #[path = "src/my_mod/mod.rs"]
//! mod my_mod;
//!
//! fn main() {
//!     ow_workshop::build::emit::<my_mod::MyMod>();
//! }
//! ```
//!
//! `cargo build` then writes `target/debug/<crate>.owmod`, and
//! `cargo build --release` writes `target/release/<crate>.owmod`.

use std::path::PathBuf;
use std::{env, fs};

use crate::{Profile, Workshop, script};

/// Writes the script for `M`, named after the crate being built.
pub fn emit<M: Workshop + Default>() {
    let name = env::var("CARGO_PKG_NAME").expect("build::emit must run from a build script");
    emit_as::<M>(&name);
}

/// Writes the script for `M` as `<name>.owmod`. Use it when one crate holds several mods.
pub fn emit_as<M: Workshop + Default>(name: &str) {
    let profile = match env::var("PROFILE").as_deref() {
        Ok("release") => Profile::Release,
        _ => Profile::Debug,
    };
    let path = profile_dir().join(format!("{name}.owmod"));
    fs::write(&path, script(&M::default(), profile))
        .unwrap_or_else(|error| panic!("cannot write {}: {error}", path.display()));
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src");
}

/// The directory holding Cargo's `build` directory, which `OUT_DIR` is inside.
fn profile_dir() -> PathBuf {
    let out_dir = env::var("OUT_DIR").expect("build::emit must run from a build script");
    PathBuf::from(out_dir)
        .ancestors()
        .find(|dir| dir.file_name().is_some_and(|name| name == "build"))
        .and_then(|build| build.parent())
        .expect("OUT_DIR is inside the target directory")
        .to_path_buf()
}
