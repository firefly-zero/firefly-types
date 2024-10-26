use alloc::boxed::Box;
use serde::{Deserialize, Serialize};

/// The progress that the player made earning badges in the game.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AppProgress {
    /// How much XP the player has earned in the game.
    pub xp: u16,

    /// Progress of earning each badge.
    pub badges: Box<[u16]>,
}

pub struct BadgeProgress {
    /// How many points are already earned for the badge.
    pub done: u16,

    /// How many points needed to earn the badge.
    pub goal: u16,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Badges<'a> {
    /// Detailed information about achievements.
    #[serde(borrow)]
    pub badges: Box<[Badge<'a>]>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Badge<'a> {
    /// The order in which achievement should be displayed, ascending.
    ///
    /// Earned achievments bubble up.
    pub position: u16,

    /// How much XP earning the achievement brings to the player.
    pub xp: u8,

    /// If the achievement should be hidden until earned.
    pub hidden: bool,

    /// Human-readable achievement name.
    pub name: &'a str,

    /// Human-readable achievement description. Typically, a hint on how to earn it.
    pub descr: &'a str,
}

impl<'a> Badge<'a> {
    pub fn validate() {
        todo!()
    }
}
