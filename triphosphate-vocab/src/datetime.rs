#[derive(Debug, Clone, PartialEq)] // TODO: What't the right PartialEq here?
pub struct Datetime {
    time: chrono::DateTime<chrono::FixedOffset>,
    /// Only used for serialization, to ensure round tripping.
    ///
    /// From the [ATProto docs](https://atproto.com/specs/lexicon#datetime):
    ///
    /// > Implementations should be aware when round-tripping records containing
    /// > datetimes of two ambiguities: loss-of-precision, and ambiguity with
    /// > trailing fractional second zeros. If de-serializing Lexicon records in
    /// > to native types, and then re-serializing, the string representation
    /// > may not be the same, which could result in broken hash references,
    /// > sanity check failures, or repository update churn. A safer thing to do
    /// > is to deserialize the datetime as a simple string, which ensures
    /// > round-trip re-serialization.
    ///
    /// Should meet RCF 3339.
    repr: String,
}

impl Datetime {
    pub fn new(time: chrono::DateTime<chrono::FixedOffset>) -> Self {
        let repr = time.to_rfc3339();
        Self { time, repr }
    }

    pub fn now() -> Self {
        Self::new(chrono::Utc::now().into())
    }
}

impl super::StringFormat for Datetime {
    fn as_str(&self) -> &str {
        &self.repr
    }

    type Error = chrono::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Error> {
        let time = chrono::DateTime::parse_from_rfc3339(s)?;
        let repr = s.to_owned();
        Ok(Self { time, repr })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_roundtrip() {
        crate::tests::valids::<Datetime>(&[
            // preferred
            "1985-04-12T23:20:50.123Z",
            "1985-04-12T23:20:50.123456Z",
            "1985-04-12T23:20:50.120Z",
            "1985-04-12T23:20:50.120000Z",
            // supported
            "1985-04-12T23:20:50.1235678912345Z",
            "1985-04-12T23:20:50.100Z",
            "1985-04-12T23:20:50Z",
            "1985-04-12T23:20:50.0Z",
            "1985-04-12T23:20:50.123+00:00",
            "1985-04-12T23:20:50.123-07:00",
            // Ensure timezone is preserved.
            "2023-08-05T00:10:41.220151955+01:00",
            "2023-08-05T00:10:41.220151955+06:00",
        ]);
    }

    #[test]
    fn test_invalid() {
        crate::tests::invalids::<Datetime>(&[
            // "1985-04-12 23:20:50.123Z",
            // "1985-04-12t23:20:50.123Z", // TODO: chrono's parser is case sensitive
            // "1985-04-12T23:20:50.123z",
            "1985-04-12",
            "1985-04-12T23:20Z",
            "1985-04-12T23:20:5Z",
            "1985-04-12T23:20:50.123",
            "+001985-04-12T23:20:50.123Z",
            "23:20:50.123Z",
        ]);
    }
}
