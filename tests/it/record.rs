use anyhow::{Context, Result};
use triphosphate::{client::Client, lex::app::bsky::feed::Post};
use triphosphate_vocab::{AtIdentifier, Datetime, StringFormat};

#[tokio::test]
async fn get_post() -> Result<()> {
    let client = Client::new()?;

    let p = client
        .get_record::<Post>(
            AtIdentifier::from_str("alona.page")?,
            "3k6gdcdcl3n25".to_owned(),
        )
        .await
        .context("failed to get post")?;

    assert_eq!(p.value.text, "Testing Testing, Can I get this record.");

    Ok(())
}

#[tokio::test]
#[ignore = "needs auth variables"]
async fn put_record() -> Result<()> {
    let mut client = triphosphate::client::Client::new()?;

    let handle = std::env::var("ATP_USERNAME").unwrap();
    let password = std::env::var("ATP_PASSWORD").unwrap();

    client.login(&handle, &password).await?;

    let my_handle = AtIdentifier::from_str("triphosphate-tests.bsky.social")?;

    let my_post = Post {
        created_at: Datetime::now(),
        embed: None,
        entities: None,
        facets: None,
        labels: None,
        langs: None,
        reply: None,

        text: "It's a bad idea to call prod in tests, but lol lmao.".to_owned(),
    };

    client.create_record(&my_post, my_handle).await?;

    Ok(())
}
