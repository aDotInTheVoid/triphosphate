use winnow::Parser;

use crate::parsing;

use super::{ParseError, StringFormat};

#[derive(Debug, Clone, PartialEq)]
// TODO: Reduce visibiltiy
pub struct Handle(pub(super) String);

impl StringFormat for Handle {
    fn as_str(&self) -> &str {
        &self.0
    }

    type Error = super::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Error> {
        match parsing::handle.parse(s) {
            Ok(_) => Ok(Self(s.to_owned())),
            Err(_) => Err(ParseError(())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // https://github.com/bluesky-social/atproto/blob/ea9d96e3a44119ca6189e7a3989a1bd9b54989a9/packages/identifier/tests/handle.test.ts#L8

    #[test]
    fn valid() {
        crate::tests::valids::<Handle>(&[
            "A.ISI.EDU",
            "XX.LCS.MIT.EDU",
            "SRI-NIC.ARPA",
            "john.test",
            "jan.test",
            "a234567890123456789.test",
            "john2.test",
            "john-john.test",
            "john.bsky.app",
            "jo.hn",
            "a.co",
            "a.org",
            "joh.n",
            "j0.h0",
            "shoooort.loooooooooooooooooooooooooong.loooooooooooooooooooooooooong.loooooooooooooooooooooooooong.loooooooooooooooooooooooooong.loooooooooooooooooooooooooong.loooooooooooooooooooooooooong.loooooooooooooooooooooooooong.loooooooooooooooooooooooooong.test",
            "short.ooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo.test",
            "jaymome-johnber123456.test",
            "jay.mome-johnber123456.test",
            "john.test.bsky.app",
            "john.t",
            "laptop.local",
            "laptop.arpa",
            "xn--ls8h.test",     // 💩.test
            "xn--bcher-kva.tld", // bücher.tld
            "xn--3jk.com",
            "xn--w3d.com",
            "xn--vqb.com",
            "xn--ppd.com",
            "xn--cs9a.com",
            "xn--8r9a.com",
            "xn--cfd.com",
            "xn--5jk.com",
            "xn--2lb.com",
            "expyuzz4wqqyqhjn.onion",
            "friend.expyuzz4wqqyqhjn.onion",
            "g2zyxa5ihm7nsggfxnu52rck2vv4rvmdlkiu3zzui5du4xyclen53wid.onion",
            "friend.g2zyxa5ihm7nsggfxnu52rck2vv4rvmdlkiu3zzui5du4xyclen53wid.onion",
            "friend.g2zyxa5ihm7nsggfxnu52rck2vv4rvmdlkiu3zzui5du4xyclen53wid.onion",
            "2gzyxa5ihm7nsggfxnu52rck2vv4rvmdlkiu3zzui5du4xyclen53wid.onion",
            "friend.2gzyxa5ihm7nsggfxnu52rck2vv4rvmdlkiu3zzui5du4xyclen53wid.onion",
            "12345.test",
            "8.cn",
            "4chan.org",
            "4chan.o-g",
            "blah.4chan.org",
            "thing.a01",
            "120.0.0.1.com",
            "0john.test",
            "9sta--ck.com",
            "99stack.com",
            "0ohn.test",
            "john.t--t",
            "thing.0aa.thing",
            "stack.com",
            "sta-ck.com",
            "sta---ck.com",
            "sta--ck9.com",
            "stack99.com",
            "sta99ck.com",
            "google.com.uk",
            "google.co.in",
            "google.com",
            "maselkowski.pl",
            "m.maselkowski.pl",
            "xn--masekowski-d0b.pl",
            "xn--fiqa61au8b7zsevnm8ak20mc4a87e.xn--fiqs8s",
            "xn--stackoverflow.com",
            "stackoverflow.xn--com",
            "stackoverflow.co.uk",
            "xn--masekowski-d0b.pl",
            "xn--fiqa61au8b7zsevnm8ak20mc4a87e.xn--fiqs8s",
        ]);
    }

    #[test]
    fn invalid() {
        crate::tests::invalids::<Handle>(&[
            "did:thing.test",
            "did:thing",
            "john-.test",
            "john.0",
            "john.-",
            "short.oooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo.test",
            // "shooooort.loooooooooooooooooooooooooong.loooooooooooooooooooooooooong.loooooooooooooooooooooooooong.loooooooooooooooooooooooooong.loooooooooooooooooooooooooong.loooooooooooooooooooooooooong.loooooooooooooooooooooooooong.loooooooooooooooooooooooooong.test",
            "xn--bcher-.tld",
            "john..test",
            "jo_hn.test",
            "-john.test",
            ".john.test",
            "jo!hn.test",
            "jo%hn.test",
            "jo&hn.test",
            "jo@hn.test",
            "jo*hn.test",
            "jo|hn.test",
            "jo:hn.test",
            "jo/hn.test",
            "john💩.test",
            "bücher.test",
            "john .test",
            "john.test.",
            "john",
            "john.",
            ".john",
            "john.test.",
            ".john.test",
            " john.test",
            "john.test ",
            "joh-.test",
            "john.-est",
            "john.tes-",
            "org",
            "ai",
            "gg",
            "io",
            "cn.8",
            "thing.0aa",
            "thing.0aa",
            "127.0.0.1",
            "192.168.0.142",
            "fe80::7325:8a97:c100:94b",
            "2600:3c03::f03c:9100:feb0:af1f",
            "-notvalid.at-all",
            "-thing.com",
            "www.masełkowski.pl.com",
        ]);
    }
}
