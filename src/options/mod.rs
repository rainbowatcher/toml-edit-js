//! Option parsing and validation for wasm APIs.
pub mod edit;
pub mod stringify;

/// Edit API option model and JS binding type.
pub use edit::{EditOptions, IEditOptions};
/// Stringify API option model and JS binding type.
pub use stringify::{IStringifyOptions, StringifyOptions};
