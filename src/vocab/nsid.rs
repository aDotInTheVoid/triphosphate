use std::{fmt, sync::OnceLock};

use regex::Regex;

use super::StringFormat;

/// A [Namespaced Identifier](atproto_docs).
///
/// [atproto_docs]: https://atproto.com/specs/nsid
pub struct Nsid {
    repr: String,
    last_dot: usize,
}
serde_impls! { Nsid }

impl Nsid {
    /// The domain authority of the NSID.
    /// ```
    /// # use triphosphate::vocab::{Nsid, StringFormat};
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
    /// # use triphosphate::vocab::{Nsid, StringFormat};
    /// let nsid = Nsid::from_str("com.atproto.sync.getHead").unwrap();
    ///
    /// assert_eq!(nsid.name(), "getHead");
    /// ```
    pub fn name(&self) -> &str {
        &self.repr[self.last_dot + 1..]
    }
}

#[derive(Debug)]
pub struct ParseError(());

impl StringFormat for Nsid {
    fn as_str(&self) -> &str {
        &self.repr
    }

    type Error = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Error> {
        static RE: OnceLock<Regex> = OnceLock::new();

        let re = RE.get_or_init(|| {
            Regex::new(
                r#"(?x)
                ^
                [a-zA-Z]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?
                (\.[a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)+
                (?<dot>\.)
                [a-zA-Z]([a-zA-Z]{0,61}[a-zA-Z])?
                $"#,
            )
            .unwrap()
        });

        let caps = re.captures(s).ok_or(ParseError(()))?;

        let dot_idx = caps.name("dot").unwrap().range();

        debug_assert_eq!(dot_idx.start + 1, dot_idx.end);

        if s.len() > 253 + 1 + 63 {
            return Err(ParseError(()));
        }

        Ok(Nsid {
            repr: s.to_owned(),
            last_dot: dot_idx.start,
        })
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse NSID")
    }
}
impl std::error::Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid() {
        for i in [
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
        ] {
            match Nsid::from_str(i) {
                Ok(nsid) => {
                    assert_eq!(nsid.as_str(), i);
                    assert_eq!(format!("{}.{}", nsid.authority(), nsid.name()), i);
                }
                Err(e) => panic!("Failed to parse {i:?}: {e:?}"),
            }
        }
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
