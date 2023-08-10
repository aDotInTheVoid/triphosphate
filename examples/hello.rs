use chrono::Local;
use triphosphate::{lex::app::bsky::feed::Post, vocab::Datetime};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let client = reqwest::Client::builder()
        .user_agent("triphsphate")
        .build()
        .unwrap();

    let post = Post {
        created_at: Datetime::now(),
        embed: None,
        entities: None,
        facets: None,
        langs: None,
        reply: None,
        labels: None,
        text: "Hello from Triphosphate!!".to_string(),
    };

    let post_json = serde_json::to_string_pretty(&post).unwrap();

    println!("{}", post_json);

    let resp = client
        .post("https://bsky.social/xrpc/com.atproto.server.createSession")
        .json(&LoginBody {
            identifier: std::env::var("ATP_USERNAME").unwrap(),
            password: std::env::var("ATP_PASSWORD").unwrap(),
        })
        .send()
        .await
        .unwrap();

    dbg!(resp.status());
    let creds = resp.json::<LoginResponse>().await.unwrap();

    let resp = client
        .post("https://bsky.social/xrpc/com.atproto.repo.createRecord")
        .bearer_auth(&creds.access_jwt)
        .json(&CreateRecord {
            repo: creds.did,
            collection: "app.bsky.feed.post".to_string(),
            record: post,
        })
        .send()
        .await
        .unwrap();

    dbg!(resp.status());
    dbg!(resp.text().await.unwrap());
}

// TODO: Derive these from createSession.json
#[derive(serde::Serialize)]
struct LoginBody {
    identifier: String,
    password: String,
}
#[derive(serde::Deserialize, Debug)]
struct LoginResponse {
    #[serde(rename = "accessJwt")]
    access_jwt: String,
    did: String,
}

#[derive(serde::Serialize)]
struct CreateRecord<T> {
    repo: String,
    collection: String,
    record: T,
}
