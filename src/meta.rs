#![allow(clippy::module_name_repetitions)]

use crate::Encode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Meta<'a> {
    pub app_id: &'a str,
    /// App name is shown in the launcher in the list of apps.
    pub app_name: &'a str,
    pub author_id: &'a str,
    pub author_name: &'a str,
    /// Launcher is the app that starts first when runtime is launched.
    pub launcher: bool,
    /// Let the app to use privileged and dangerous runtime API.
    pub sudo: bool,
    /// The ever-incrementing version number of the app build.
    /// Used by netplay to ensure both devices running the same version.
    pub version: u32,
}

impl<'a> Encode<'a> for Meta<'a> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meta_roundtrip() {
        let given = Meta {
            app_id: "some-app-id",
            app_name: "Some App Name",
            author_id: "some-author-id",
            author_name: "Some Author Name",
            launcher: false,
            sudo: false,
            version: 12,
        };
        let mut buf = vec![0; given.size()];
        let raw = given.encode_buf(&mut buf).unwrap();
        let actual = Meta::decode(raw).unwrap();
        assert_eq!(given, actual);
    }
}
