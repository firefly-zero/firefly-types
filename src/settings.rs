use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Settings {
    /// How much XP the player eared over all games.
    xp: u32,

    /// How many badges the player eared over all games.
    badges: u32,

    /// A two-letter ASCII ISO 639 Set 1 language code.
    lang: [u8; 2],

    /// The device name.
    name: String,

    /// The full timezone name as in the IANA database.
    timezone: String,
}
