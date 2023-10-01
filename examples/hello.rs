use triphosphate::lex::app::bsky::feed::Post;
use triphosphate_vocab::{AtIdentifier, Datetime};

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
        tags: None,
        text: "test, langs=None".to_string(),
    };

    let my_repo = AtIdentifier::Did(creds.did);
    let resp = client.create_record(&post, my_repo).await?;

    dbg!(resp);

    Ok(())
}
