use crate::encode::Encode;
use alloc::borrow::Cow;
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
    ///
    /// It's possible to have negative scores when the app needs scores on the board
    /// to be ordered in ascending order rather than descending.
    /// So the default minimum is [`i16::MIN`] rather than 0.
    pub min: i16,

    /// The maximum value for a score to be added to the board.
    ///
    /// Useful for filtering out obvious cheating.
    pub max: i16,

    /// If the score should be formatted as time.
    pub time: bool,

    /// Digits after decimal point.
    pub decimals: u8,

    /// Human-readable board name.
    pub name: &'a str,
}

impl Board<'_> {
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
