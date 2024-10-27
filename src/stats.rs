use crate::encode::Encode;
use alloc::boxed::Box;
use serde::{Deserialize, Serialize};

/// Player-specific app stats, like playtime.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Stats {
    /// How many minutes the app was played for every player count from 1 to 4+.
    ///
    /// Games with more than 4 players are recorded as 4 players.
    /// So, the last index (3) is "4+ players".
    ///
    /// The play time is calculated based on the number of the calls
    /// to the `update` callback. So, the number is an approximation
    /// and may drift a bit.
    ///
    /// Plays shorter than 1 minute are not recorded.
    pub minutes: [u32; 4],

    /// How many minutes the app was played the longest for every player count from 1 to 4+.
    ///
    /// Use this metric carefully. The runtime might or might not account for players
    /// pausing the app for days instead of exiting it. Which means, multiple play
    /// sessions may be counted as one.
    pub longest_play: [u32; 4],

    /// How many times the app was launched for every player count from 1 to 4+.
    pub launches: [u32; 4],

    /// The date when the app was installed.
    ///
    /// The date is a tuple of year, month, and day of month.
    pub installed_on: (u16, u8, u8),

    /// The date when the app was updated.
    ///
    /// The date is a tuple of year, month, and day of month.
    pub updated_on: (u16, u8, u8),

    /// The date when the app was launched last time.
    ///
    /// The date is a tuple of year, month, and day of month.
    pub launched_on: (u16, u8, u8),

    /// How much XP the player has earned in the game.
    ///
    /// Cannot be more than 1000.
    pub xp: u16,

    /// The progress of earning each badge.
    ///
    /// The len is equal to the number of badges that the app has.
    /// See [`crate::Badges`].
    pub badges: Box<[BadgeProgress]>,
}

impl<'a> Encode<'a> for Stats {}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip() {
        let given = Stats {
            minutes: [11, 12, 13, 14],
            longest_play: [21, 22, 23, 24],
            launches: [31, 32, 33, 34],
            installed_on: (2023, 12, 31),
            updated_on: (2024, 1, 17),
            launched_on: (2024, 2, 28),
            xp: 32,
            badges: Box::new([]),
        };
        let mut buf = vec![0; given.size()];
        let raw = given.encode(&mut buf).unwrap();
        let actual = Stats::decode(raw).unwrap();
        assert_eq!(given, actual);
    }
}
