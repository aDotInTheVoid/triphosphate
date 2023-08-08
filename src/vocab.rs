use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AtUri;

#[derive(Deserialize, Serialize)]
pub struct Cid;

#[derive(Deserialize, Serialize)]
pub struct Did;

#[derive(Deserialize, Serialize)]
pub struct Handle;

#[derive(Deserialize, Serialize)]
pub struct Uri;

#[derive(Deserialize, Serialize)]
pub struct Blob;

pub type Datetime = chrono::DateTime<chrono::FixedOffset>;

pub type Unknown = serde_json::Value; // TODO

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_timezone() {
        fn check(start: &str, end: &str) {
            let dt: Datetime = serde_json::from_str(start).unwrap();
            let dt = serde_json::to_string(&dt).unwrap();

            assert_eq!(dt, end);
        }

        // Ensure we can round-trip without losing TZ info.
        check(
            r#""2023-08-05T00:10:41.220151955+01:00""#,
            r#""2023-08-05T00:10:41.220151955+01:00""#,
        );
        check(
            r#""2023-08-05T00:10:41.220151955+06:00""#,
            r#""2023-08-05T00:10:41.220151955+06:00""#,
        );

        // TODO: More tests: https://atproto.com/specs/lexicon#datetime
    }
}
