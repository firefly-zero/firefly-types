use alloc::borrow::Cow;
use alloc::boxed::Box;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Scores {
    boards: Box<[BoardScores]>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct BoardScores {
    /// Top scores of the local player.
    me: Box<[u32; 16]>,

    /// Top scores of friends.
    friends: Box<[FriendScore; 16]>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct FriendScore {
    index: u16,
    score: u16,
}

/// List of friends' names.
///
/// New friends must be appended at the end to keep the IDs.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Friends<'a> {
    #[serde(borrow)]
    pub friends: Cow<'a, [&'a str]>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Boards<'a> {
    #[serde(borrow)]
    pub boards: Cow<'a, [Board<'a>]>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Board<'a> {
    /// The order in which the board should be displayed, ascending.
    pub position: u16,

    /// The minimum value for a score to be added to the board.
    pub min: u32,

    /// The maximum value for a score to be added to the board.
    ///
    /// Useful for filtering out obvious cheating.
    pub max: u32,

    /// If the scores should go in ascending order.
    ///
    /// If false (default), uses descending ("larger is better") order.
    /// Ascending order makes sense for time in racing games.
    pub asc: bool,

    /// If the score should be formatted as time.
    pub time: bool,

    /// Digits after decimal point.
    pub decimals: u8,

    /// Human-readable board name.
    pub name: &'a str,
}
