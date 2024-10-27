use alloc::borrow::Cow;
use alloc::boxed::Box;
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

/// The progress that the player made earning badges in the game.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AppProgress {
    /// How much XP the player has earned in the game.
    pub xp: u16,

    /// Progress of earning each badge.
    pub badges: Box<[u16]>,
}

pub struct BadgeProgress {
    /// If true, the earning of the badge hasn't been shown to the player yet.
    pub new: bool,

    /// How many points are already earned for the badge.
    pub done: u16,

    /// How many points needed to earn the badge.
    pub goal: u16,
}

impl BadgeProgress {
    /// If the badge has been earned by the player.
    #[must_use]
    #[inline]
    pub const fn earned(&self) -> bool {
        self.done >= self.goal
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Badges<'a> {
    /// Detailed information about achievements.
    #[serde(borrow)]
    pub badges: Cow<'a, [Badge<'a>]>,
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
