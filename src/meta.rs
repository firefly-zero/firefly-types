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

    /// Encode the matadata using the buffer.
    ///
    /// The buffer is required to avoid allocations on the crate side.
    /// Use [Meta::size] to calculate the required buffer size.
    pub fn encode(&self, buf: &'a mut [u8]) -> Result<&'a mut [u8], postcard::Error> {
        postcard::to_slice(self, buf)
    }

    /// Calculate the buffer size required to encode the meta.
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
        let given = Meta {
            app_id:      "some-app-id",
            app_name:    "Some App Name",
            author_id:   "some-author-id",
            author_name: "Some Author Name",
        };
        let mut buf = vec![0; given.size()];
        let raw = given.encode(&mut buf).unwrap();
        let actual = Meta::decode(raw).unwrap();
        assert_eq!(given, actual);
    }
}
