//! Game data tables for the compiler (hand-maintained).

#[allow(
    clippy::too_many_arguments,
    clippy::too_many_lines,
    clippy::doc_markdown,
    clippy::similar_names,
    clippy::wildcard_imports,
    clippy::unreadable_literal,
    dead_code,
    missing_docs
)]
mod values;

#[allow(
    clippy::too_many_arguments,
    clippy::too_many_lines,
    clippy::doc_markdown,
    clippy::similar_names,
    clippy::wildcard_imports,
    clippy::unreadable_literal,
    dead_code,
    missing_docs
)]
mod actions;

#[allow(missing_docs)]
pub(crate) mod settings;

/// Everything generated, under one path.
pub mod api {
    pub use super::actions::*;
    pub use super::values::*;
}

#[cfg(test)]
mod tests;
