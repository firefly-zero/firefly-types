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

    /// The date when the app was launched last time.
    ///
    /// The date is a tuple of year, month, and day of month.
    pub launched_on: (u16, u8, u8),
}

impl Stats {
    /// Load stats from bytes generated by [`Stats::encode`].
    ///
    /// # Errors
    ///
    /// May return an error if the buffer does not contain valid stats.
    pub fn decode(s: &[u8]) -> Result<Self, postcard::Error> {
        postcard::from_bytes(s)
    }

    /// Encode the stats using the buffer.
    ///
    /// The buffer is required to avoid allocations on the crate side.
    /// Use [`Stats::size`] to calculate the required buffer size.
    ///
    /// # Errors
    ///
    /// May return an error if the buffer is not big enough.
    pub fn encode<'b>(&self, buf: &'b mut [u8]) -> Result<&'b mut [u8], postcard::Error> {
        postcard::to_slice(self, buf)
    }

    /// Calculate the buffer size required to encode the stats.
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn size(&self) -> usize {
        let flavor = postcard::ser_flavors::Size::default();
        postcard::serialize_with_flavor(self, flavor).unwrap()
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
            launched_on: (2024, 2, 28),
        };
        let mut buf = vec![0; given.size()];
        let raw = given.encode(&mut buf).unwrap();
        let actual = Stats::decode(raw).unwrap();
        assert_eq!(given, actual);
    }
}