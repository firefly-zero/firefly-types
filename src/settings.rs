use crate::encode::Encode;
use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Settings {
    /// How much XP the player earned over all games.
    pub xp: u32,

    /// How many badges the player eanred over all games.
    pub badges: u32,

    /// A two-letter ASCII ISO 639 Set 1 language code.
    pub lang: [u8; 2],

    /// The device name.
    pub name: String,

    /// The full timezone name as in the IANA database.
    pub timezone: String,
}

impl<'a> Encode<'a> for Settings {}
