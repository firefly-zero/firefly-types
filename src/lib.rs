#![cfg_attr(not(test), no_std)]
mod meta;
mod validators;

pub use meta::Meta;
pub use validators::{valid_id, valid_name, valid_path_part};
