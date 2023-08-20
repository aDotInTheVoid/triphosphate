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
    })
}

// #[test]
// fn with_reply() {
//     let sr = StrongRef {
//         cid: Cid,
//         uri: AtUri,
//     };

//     check(&Post {
//         text: "Hello, world!".to_owned(),
//         created_at: Datetime::new(
//             FixedOffset::east_opt(0)
//                 .unwrap()
//                 .with_ymd_and_hms(2015, 2, 18, 23, 16, 9)
//                 .unwrap(),
//         ),
//         embed: None,
//         entities: None,
//         facets: None,
//         langs: None,
//         reply: Some(ReplyRef {
//             parent: sr.clone(),
//             root: sr,
//         }),
//         labels: None,
//     })
// }
