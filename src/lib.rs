//! Parser and serializer for the Firefly Zero metadata file format.
//!
//! Based on [postcard](https://github.com/jamesmunns/postcard),
//! no_std-compatible Rust-first binary serialization format.

#![cfg_attr(not(test), no_std)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::derive_partial_eq_without_eq)]

extern crate alloc;

mod badges;
mod meta;
pub mod serial;
mod stats;
mod validators;

pub use badges::*;
pub use meta::{Meta, ShortMeta};
pub use stats::Stats;
pub use validators::*;
