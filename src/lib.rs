#![cfg_attr(not(test), no_std)]
#![feature(ascii_char)]
mod meta;
mod validators;

pub use meta::Meta;
pub use validators::*;
