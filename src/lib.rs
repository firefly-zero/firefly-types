//! Parser and serializer for the Firefly Zero metadata file format.
//!
//! Based on [postcard](https://github.com/jamesmunns/postcard),
//! no_std-compatible Rust-first binary serialization format.

#![cfg_attr(not(test), no_std)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::module_name_repetitions)]
#![expect(clippy::derive_partial_eq_without_eq, clippy::multiple_crate_versions)]

extern crate alloc;

mod badges;
mod boards;
mod encode;
mod meta;
pub mod serial;
mod settings;
pub mod spi;
mod stats;
mod validators;

pub use badges::*;
pub use boards::*;
pub use encode::*;
pub use meta::{Meta, ShortMeta};
pub use settings::*;
pub use stats::*;
pub use validators::*;
