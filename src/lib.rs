#![cfg_attr(not(test), no_std)]
mod meta;
mod validators;

pub use meta::{Meta, ShortMeta};
pub use validators::*;
