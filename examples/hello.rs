use chrono::{Local, Utc};
use triphosphate::lex::app::bsky::feed::Post;

fn main() {
    let now = Utc::now();

    let post = Post {
        created_at: Local::now().fixed_offset(),
        embed: None,
        entities: None,
        facets: None,
        langs: None,
        reply: None,
        text: "Hello, world!".to_string(),
    };

    let post_json = serde_json::to_string_pretty(&post).unwrap();

    println!("{}", post_json);
}
