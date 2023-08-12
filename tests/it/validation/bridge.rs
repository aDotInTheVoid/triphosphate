use serde_json::json;

use super::{validate, ValidationResult};

#[test]
fn empty_post() {
    assert_eq!(
        validate("app.bsky.feed.post", &json!({})),
        ValidationResult::Invalid("Record must have the property \"text\"".to_owned())
    );
}

#[test]
fn post_only_text() {
    assert_eq!(
        validate(
            "app.bsky.feed.post",
            &json!({
                "text": "No Date??!!"
            })
        ),
        ValidationResult::Invalid("Record must have the property \"createdAt\"".to_owned())
    );
}

#[test]
fn post_invalid_created_at() {
    assert_eq!(
        validate(
            "app.bsky.feed.post",
            &json!({
                "text": "Bad Date??!!",
                "createdAt": "Now",
            })
        ),
        ValidationResult::Invalid(
            "Record/createdAt must be an iso8601 formatted datetime".to_owned()
        )
    );
}

#[test]
fn valid_post_raw_json() {
    assert_eq!(
        validate(
            "app.bsky.feed.post",
            &json!({
               "text": "You're valid, and so is this post",
               "createdAt": "2014-11-28T12:45:59.324310806Z",
            })
        ),
        ValidationResult::Valid
    );
}
