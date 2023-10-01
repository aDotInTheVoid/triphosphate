use chrono::{FixedOffset, TimeZone};
use triphosphate::lex::app::bsky::feed::Post;
use triphosphate_vocab::Datetime;

use super::check;

#[test]
fn basic() {
    check(&Post {
        text: "Hello, world!".to_owned(),
        created_at: Datetime::new(
            FixedOffset::east_opt(0)
                .unwrap()
                .with_ymd_and_hms(2015, 2, 18, 23, 16, 9)
                .unwrap(),
        ),
        embed: None,
        entities: None,
        facets: None,
        langs: None,
        reply: None,
        labels: None,
        tags: None,
    })
}

#[test]
fn with_reply() {
    // This was hacked to remove the $type field.
    // TODO: Support $type.
    let post_s = r#"
    {
        "text": "I can’t wait",
        "langs": [
            "en"
        ],
        "reply": {
            "root": {
                "cid": "bafyreidmd3de32yan7s5kenrm4kk6pekon2ils5z5my5qfqlgyc6shjkwy",
                "uri": "at://did:plc:p2cp5gopk7mgjegy6wadk3ep/app.bsky.feed.post/3k5dwfkz4gg23"
            },
            "parent": {
                "cid": "bafyreiamthrwyh4aso66wilslaxfgqwg77gv7lo5woenfay2x6ih23skve",
                "uri": "at://did:plc:mmbnkonsfestr6vvcpy7ffz3/app.bsky.feed.post/3k5dzus5lew2m"
            }
        },
        "createdAt": "2023-08-20T09:00:33.274Z"
    }"#;

    let post: Post = serde_json::from_str(post_s).unwrap();

    assert_eq!(
        serde_json::to_value(&post).unwrap(),
        serde_json::from_str::<serde_json::Value>(post_s).unwrap()
    );
    check(&post);
}
