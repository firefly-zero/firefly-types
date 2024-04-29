//! Parser and serializer for the Firefly Zero metadata file format.
//!
//! Based on [postcard](https://github.com/jamesmunns/postcard),
//! no_std-compatible Rust-first binary serialization format.

#![cfg_attr(not(test), no_std)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

mod meta;
mod validators;

pub use meta::{Meta, ShortMeta};
pub use validators::*;
