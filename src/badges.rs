use alloc::boxed::Box;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Progress {
    /// How much XP the player has earned in the game.
    pub xp: u16,

    /// How many points are already earned for each achievement.
    pub done: Box<[u16]>,

    /// How many points needed to earn each achievement.
    ///
    /// A regular achievement would have 1 step: you simply earned it or not.
    pub goal: Box<[u16]>,
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
