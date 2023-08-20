use triphosphate::{
    lex::{app::bsky::feed::Post, com::atproto::repo::create_record},
    vocab::{AtIdentifier, Datetime, Nsid, StringFormat},
    LexItem,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let mut client = triphosphate::client::Client::new()?;

    let handle = std::env::var("ATP_USERNAME").unwrap();
    let password = std::env::var("ATP_PASSWORD").unwrap();

    let creds = client.login(&handle, &password).await?;

    let post = Post {
        created_at: Datetime::now(),
        embed: None,
        entities: None,
        facets: None,
        langs: None,
        reply: None,
        labels: None,
        text: "Hello from Triphosphate! Now contains procedures!".to_string(),
    };

    let resp = create_record(
        &client,
        &create_record::Args {
            collection: Nsid::from_str(Post::URI).unwrap(),
            record: serde_json::to_value(&post)?,
            repo: AtIdentifier::Did(creds.did),
            validate: Some(true),
            rkey: None,
            swap_commit: None,
        },
    )
    .await?;

    dbg!(resp);

    Ok(())
}
