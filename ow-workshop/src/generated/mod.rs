//! Generated code (`cargo xtask codegen`).

// Formatted by the generator (prettyplease), not rustfmt.
#[rustfmt::skip]
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

#[rustfmt::skip]
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

/// Everything generated, under one path.
pub mod api {
    pub use super::actions::*;
    pub use super::values::*;
}

#[cfg(test)]
mod tests;
