use crate::encode::Encode;
use alloc::borrow::Cow;
use alloc::boxed::Box;
use core::fmt::Display;
use serde::{Deserialize, Serialize};

pub enum BoardValidationError {
    EmptyName,
    NameTooLong,
    MinGtMax,
}

impl BoardValidationError {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyName => "name must not be empty",
            Self::NameTooLong => "name is too long",
            Self::MinGtMax => "min must be less than or equal to max",
        }
    }
}

impl Display for BoardValidationError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Scores {
    boards: Box<[BoardScores]>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct BoardScores {
    /// Top scores of the local player.
    me: Box<[u16; 8]>,

    /// Top scores of friends.
    friends: Box<[FriendScore; 8]>,
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

impl<'a> Boards<'a> {
    #[must_use]
    pub const fn new(boards: Cow<'a, [Board<'a>]>) -> Self {
        Self { boards }
    }
}

impl<'a> Encode<'a> for Boards<'a> {}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Board<'a> {
    /// The order in which the board should be displayed, ascending.
    pub position: u16,

    /// The minimum value for a score to be added to the board.
    pub min: u16,

    /// The maximum value for a score to be added to the board.
    ///
    /// Useful for filtering out obvious cheating.
    pub max: u16,

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

impl<'a> Board<'a> {
    /// Validate board attributes.
    ///
    /// # Errors
    ///
    /// Returns [`BoardValidationError`] if any of the attributes are not valid.
    pub const fn validate(&self) -> Result<(), BoardValidationError> {
        if self.name.is_empty() {
            return Err(BoardValidationError::EmptyName);
        }
        if self.name.len() > 64 {
            return Err(BoardValidationError::NameTooLong);
        }
        if self.min > self.max {
            return Err(BoardValidationError::MinGtMax);
        }
        Ok(())
    }
}
