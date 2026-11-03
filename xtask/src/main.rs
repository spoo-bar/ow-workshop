//! Repo automation tasks (`cargo xtask <task>`).

mod codegen;
mod data;

use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let result = match args.next().as_deref() {
        Some("validate-data") => {
            let dir = args
                .next()
                .map_or_else(|| PathBuf::from("data"), PathBuf::from);
            data::validate_dir(&dir)
        }
        Some("codegen") => codegen::run(
            &PathBuf::from("data"),
            &PathBuf::from("ow-workshop-data/src/generated"),
        ),
        _ => Err("usage: cargo xtask <validate-data [dir] | codegen>".to_owned()),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::FAILURE
        }
    }
}
