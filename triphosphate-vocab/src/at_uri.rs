use winnow::Parser;

use crate::parsing;

use super::{ParseError, StringFormat};

#[derive(Debug, Clone, PartialEq)]
pub struct AtUri {
    s: String,
}

impl StringFormat for AtUri {
    fn as_str(&self) -> &str {
        &self.s
    }

    type Error = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Error> {
        match parsing::at_uri.parse(s) {
            Ok(_) => Ok(Self { s: s.to_owned() }),
            Err(_) => Err(ParseError(())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid() {
        crate::tests::valids::<AtUri>(&[
            "at://did:plc:44ybard66vv44zksje25o7dz/app.bsky.feed.post/3jwdwj2ctlk26",
            "at://bnewbold.bsky.team/app.bsky.feed.post/3jwdwj2ctlk26",
            "at://foo.com/com.example.foo/123",
        ]);
    }

    #[test]
    fn invalid() {
        crate::tests::invalids::<AtUri>(&[
            "at://foo.com/example/123",
            "at://computer",
            "at://example.com:3000",
            "at://foo.com/",
            "at://user:pass@foo.com",
        ]);
    }

    // TODO: Import https://github.com/bluesky-social/atproto/blob/main/packages/uri/tests/uri.test.ts
}
