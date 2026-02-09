//! Public crate surface for TOML parse/stringify/edit operations.
//!
//! The crate is organized by concern:
//! - `core`: wasm bindings and high-level APIs
//! - `ops`: low-level document mutation routines
//! - `options`: JS option parsing and validation
//! - `util`: conversion and formatting helpers
pub mod core;
pub mod ops;
pub mod options;
pub mod util;

/// Re-export high-level APIs for JavaScript consumers.
pub use core::{edit, parse, stringify};
