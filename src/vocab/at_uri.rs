use super::StringFormat;

/// <https://atproto.com/specs/at-uri-scheme>
#[derive(Clone)]
pub struct AtUri {
    /// ```bnf
    /// AT-URI        = "at://" AUTHORITY [ "/" COLLECTION [ "/" RKEY ] ]
    ///
    /// AUTHORITY     = HANDLE | DID
    /// COLLECTION    = NSID
    /// RKEY          = RECORD-KEY
    /// ```
    url: url::Url,
}

impl AtUri {
    pub fn new(url: url::Url) -> Self {
        Self { url }
    }
}

impl StringFormat for AtUri {
    fn as_str(&self) -> &str {
        self.url.as_str()
    }

    type Error = url::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Error> {
        url::Url::parse(s).map(Self::new)
    }
}

serde_impls! { AtUri }
