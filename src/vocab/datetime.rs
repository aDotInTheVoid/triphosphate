use serde::de::Error;

#[derive(Debug, Clone)] // TODO: What't the right PartialEq here?
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
    fn new(time: chrono::DateTime<chrono::FixedOffset>) -> Self {
        let repr = time.to_rfc3339();
        Self { time, repr }
    }

    pub fn now() -> Self {
        Self::new(chrono::Utc::now().into())
    }
}

impl serde::Serialize for Datetime {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.repr.serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for Datetime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let repr = String::deserialize(deserializer)?;
        let time = chrono::DateTime::parse_from_rfc3339(&repr).map_err(D::Error::custom)?;
        // let time = repr.parse().map_err(D::Error::custom)?;

        Ok(Self { repr, time })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_roundtrip() {
        for i in [
            // preferred
            r#""1985-04-12T23:20:50.123Z""#,
            r#""1985-04-12T23:20:50.123456Z""#,
            r#""1985-04-12T23:20:50.120Z""#,
            r#""1985-04-12T23:20:50.120000Z""#,
            // supported
            r#""1985-04-12T23:20:50.1235678912345Z""#,
            r#""1985-04-12T23:20:50.100Z""#,
            r#""1985-04-12T23:20:50Z""#,
            r#""1985-04-12T23:20:50.0Z""#,
            r#""1985-04-12T23:20:50.123+00:00""#,
            r#""1985-04-12T23:20:50.123-07:00""#,
            // Ensure timezone is preserved.
            r#""2023-08-05T00:10:41.220151955+01:00""#,
            r#""2023-08-05T00:10:41.220151955+06:00""#,
        ] {
            let d: Datetime = serde_json::from_str(i).unwrap();
            let d2 = serde_json::to_string(&d).unwrap();
            assert_eq!(i, d2);
        }
    }

    #[test]
    fn test_invalid() {
        for i in [
            r#""1985-04-12 23:20:50.123Z""#,
            // r#""1985-04-12t23:20:50.123Z""#, // TODO: chrono's parser is case sensitive
            // r#""1985-04-12T23:20:50.123z""#,
            r#""1985-04-12""#,
            r#""1985-04-12T23:20Z""#,
            r#""1985-04-12T23:20:5Z""#,
            r#""1985-04-12T23:20:50.123""#,
            r#""+001985-04-12T23:20:50.123Z""#,
            r#""23:20:50.123Z""#,
        ] {
            if let Ok(d) = serde_json::from_str::<Datetime>(i) {
                panic!("expected error for {i}, got {d:?}");
            }
        }
    }
}
