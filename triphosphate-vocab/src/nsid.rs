use std::borrow::Cow;

use winnow::Parser;

use crate::parsing;

use super::{ParseError, StringFormat};

/// A [Namespaced Identifier][atproto_docs].
///
/// [atproto_docs]: https://atproto.com/specs/nsid
#[derive(Debug, Clone, PartialEq)]
pub struct Nsid {
    repr: Cow<'static, str>,
    last_dot: usize,
}

impl Nsid {
    /// The domain authority of the NSID.
    /// ```
    /// # use triphosphate_vocab::{Nsid, StringFormat};
    /// let nsid = Nsid::from_str("com.atproto.sync.getHead").unwrap();
    ///
    /// assert_eq!(nsid.authority(), "com.atproto.sync");
    /// ```
    pub fn authority(&self) -> &str {
        &self.repr[..self.last_dot]
    }

    /// The name of the identifier.
    ///
    /// ```
    /// # use triphosphate_vocab::{Nsid, StringFormat};
    /// let nsid = Nsid::from_str("com.atproto.sync.getHead").unwrap();
    ///
    /// assert_eq!(nsid.name(), "getHead");
    /// ```
    pub fn name(&self) -> &str {
        &self.repr[self.last_dot + 1..]
    }

    #[doc(hidden)]
    /// Only to be used by lexgen, where it knows that's it's parsed.
    pub const fn __new_unchecked(s: &'static str, last_dot: usize) -> Self {
        let repr = Cow::Borrowed(s);

        if s.as_bytes()[last_dot] != b'.' {
            panic!("invalid last_dot index");
        }
        // TODO: More asserts here

        Self { repr, last_dot }
    }
}

impl StringFormat for Nsid {
    fn as_str(&self) -> &str {
        &self.repr
    }

    type Error = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Error> {
        match parsing::nsid.parse(s) {
            Ok(_) => {
                let last_dot = s.rfind('.').unwrap();

                Ok(Nsid {
                    repr: Cow::Owned(s.to_owned()),
                    last_dot,
                })
            }
            Err(_) => Err(ParseError(())),
        }
    }
}

#[cfg(test)]
mod tests {
    // https://github.com/bluesky-social/atproto/blob/ea9d96e3a44119ca6189e7a3989a1bd9b54989a9/packages/nsid/tests/nsid.test.ts#L38

    use super::*;

    #[test]
    fn valid() {
        crate::tests::valids::<Nsid>(&[
            "com.example.foo",
            "com.example.fooBar",
            "net.users.bob.ping",
            "a.b.c",
            "m.xn--masekowski-d0b.pl",
            "one.two.three",
            "one.two.three.four-and.FiVe",
            "one.2.three",
            "a-0.b-1.c",
            "a0.b1.cc",
            "cn.8.lex.stuff",
            "test.12345.record",
            "a01.thing.record",
            "a.0.c",
            "xn--fiqs8s.xn--fiqa61au8b7zsevnm8ak20mc4a87e.record.two",
            "onion.expyuzz4wqqyqhjn.spec.getThing",
            "org.4chan.lex.getThing",
            "cn.8.lex.stuff",
            "onion.g2zyxa5ihm7nsggfxnu52rck2vv4rvmdlkiu3zzui5du4xyclen53wid.lex.deleteThing",
            "onion.2gzyxa5ihm7nsggfxnu52rck2vv4rvmdlkiu3zzui5du4xyclen53wid.lex.deleteThing",
            "com.ooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo.foo",
            "com.example.ooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo",
            "com.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.foo",
        ])
    }

    #[test]
    fn invalid() {
        for i in [
            "com.exa💩ple.thing",
            "com.example",
            "com.example.foo.*",
            "com.example.foo.blah*",
            "com.example.foo.*blah",
            "com.example.f00",
            "com.exa💩ple.thing",
            "a-0.b-1.c-3",
            "a-0.b-1.c-o",
            "a0.b1.c3",
            "1.0.0.127.record",
            "0two.example.foo",
            "example.com",
            "com.example",
            "a.",
            ".one.two.three",
            "one.two.three ",
            "one.two..three",
            "one .two.three",
            " one.two.three",
            "com.exa💩ple.thing",
            "com.atproto.feed.p@st",
            "com.atproto.feed.p_st",
            "com.atproto.feed.p*st",
            "com.atproto.feed.po#t",
            "com.atproto.feed.p!ot",
            "com.example-.foo",

            "com.oooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo.foo",
            "com.example.oooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo",
            "com.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.middle.foo",
        ] {
            match Nsid::from_str(i) {
                Ok(_) => panic!("Unexpectedly parsed {i:?}"),
                Err(_) => {}
            }
        }
    }
}
