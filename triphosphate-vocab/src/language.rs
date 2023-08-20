use oxilangtag::LanguageTag;

use crate::StringFormat;

#[derive(Debug, Clone)]
pub struct Language {
    // TODO: Cow?
    tag: LanguageTag<String>,
}

impl StringFormat for Language {
    fn as_str(&self) -> &str {
        self.tag.as_str()
    }

    type Error = oxilangtag::LanguageTagParseError;

    fn from_str(s: &str) -> Result<Self, Self::Error> {
        let tag = LanguageTag::parse(s.to_owned())?;

        Ok(Self { tag })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // https://github.com/bluesky-social/atproto/blob/8de64178c07b07ffcdaf25ae5afa78831168d02f/packages/common-web/tests/strings.test.ts#L30
    #[test]
    fn valid() {
        for s in [
            "de",
            "de-CH",
            "de-DE-1901",
            "es-419",
            "sl-IT-nedis",
            "mn-Cyrl-MN",
            "x-fr-CH",
            "en-GB-boont-r-extended-sequence-x-private",
            "sr-Cyrl",
            "hy-Latn-IT-arevela",
            "i-klingon",
        ] {
            let l = Language::from_str(s).unwrap();

            assert_eq!(l.as_str(), s);
        }
    }

    #[test]
    fn invalid() {
        for s in ["", "x", "de-CH-", "i-bad-grandfathered"] {
            match Language::from_str(s) {
                Err(_) => {}
                Ok(l) => panic!("unexpectedly parsed: {l:?}"),
            }
        }
    }

    // TODO: Accessors with tests.
}
