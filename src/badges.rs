use crate::encode::Encode;
use alloc::borrow::Cow;
use core::fmt::Display;
use serde::{Deserialize, Serialize};

pub enum BadgeValidationError {
    EmptyName,
    NameTooLong,
    DescrTooLong,
    TooMuchXp,
}

impl BadgeValidationError {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyName => "name must not be empty",
            Self::NameTooLong => "name is too long",
            Self::DescrTooLong => "descr is too long",
            Self::TooMuchXp => "one badge cannot reward more than 200 XP",
        }
    }
}

impl Display for BadgeValidationError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Badges<'a> {
    /// Detailed information about achievements.
    #[serde(borrow)]
    pub badges: Cow<'a, [Badge<'a>]>,
}

impl<'a> Badges<'a> {
    #[must_use]
    pub const fn new(badges: Cow<'a, [Badge<'a>]>) -> Self {
        Self { badges }
    }
}

impl<'a> Encode<'a> for Badges<'a> {}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Badge<'a> {
    /// The order in which badges should be displayed, ascending.
    ///
    /// Earned achievments bubble up.
    pub position: u16,

    /// How much XP earning the badge brings to the player.
    pub xp: u8,

    /// The number of steps required for the badge to be shown.
    ///
    /// If zero, the badge is always shown. If equal to the number of steps
    /// required to earn the badge, the badge will be shown only when earned.
    pub hidden: u16,

    /// Human-readable badge name.
    pub name: &'a str,

    /// Human-readable badge description. Typically, a hint on how to earn it.
    pub descr: &'a str,
}

impl Badge<'_> {
    /// Validate badge attributes.
    ///
    /// # Errors
    ///
    /// Returns [`BadgeValidationError`] if any of the attributes are not valid.
    pub const fn validate(&self) -> Result<(), BadgeValidationError> {
        if self.name.is_empty() {
            return Err(BadgeValidationError::EmptyName);
        }
        if self.name.len() > 64 {
            return Err(BadgeValidationError::NameTooLong);
        }
        if self.descr.len() > 256 {
            return Err(BadgeValidationError::DescrTooLong);
        }
        if self.xp > 200 {
            return Err(BadgeValidationError::TooMuchXp);
        }
        Ok(())
    }
}
