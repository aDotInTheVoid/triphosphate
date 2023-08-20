use triphosphate::lex::app::bsky::feed::Post;
use triphosphate_vocab::{AtIdentifier, StringFormat};

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let mut client = triphosphate::client::Client::new()?;

    let handle = std::env::var("ATP_USERNAME").unwrap();
    let password = std::env::var("ATP_PASSWORD").unwrap();

    client.login(&handle, &password).await?;

    let handle = AtIdentifier::from_str("triphosphate-tests.bsky.social")?;

    let p = client
        .get_record::<Post>(handle, "3k5fdh2qmie2m".to_owned())
        .await?;

    dbg!(p);

    Ok(())
}
