use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Meta<'a> {
    app_id:      &'a str,
    app_name:    &'a str,
    author_id:   &'a str,
    author_name: &'a str,
}

impl<'a> Meta<'a> {
    pub fn decode(s: &'a [u8]) -> Result<Self, postcard::Error> {
        postcard::from_bytes(s)
    }

    pub fn encode(&self) -> Result<Vec<u8>, postcard::Error> {
        postcard::to_allocvec(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip() {
        let given = Meta {
            app_id:      "some-app-id",
            app_name:    "Some App Name",
            author_id:   "some-author-id",
            author_name: "Some Author Name",
        };
        let raw = given.encode().unwrap();
        let actual = Meta::decode(&raw).unwrap();
        assert_eq!(given, actual);
    }
}
