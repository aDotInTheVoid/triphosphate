use triphosphate::lex::app::bsky::feed::post::ReplyRef;
use triphosphate::lex::{app::bsky::feed::Post, com::atproto::repo::StrongRef};
use triphosphate_vocab::{AtIdentifier, Datetime, StringFormat};

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let mut client = triphosphate::client::Client::new()?;

    let handle = std::env::var("ATP_USERNAME").unwrap();
    let password = std::env::var("ATP_PASSWORD").unwrap();

    client.login(&handle, &password).await?;

    let my_handle = AtIdentifier::from_str("triphosphate-tests.bsky.social")?;
    let their_handle = AtIdentifier::from_str("mozzius.dev")?;

    let their_post = client
        .get_record::<Post>(their_handle, "3k5evzd6mb22n".to_owned())
        .await?;

    let parent = StrongRef {
        cid: their_post.cid.unwrap(),
        uri: their_post.uri,
    };

    let my_post = Post {
        created_at: Datetime::now(),

        embed: None,
        entities: None,
        facets: None,
        labels: None,

        langs: Some(vec![]),

        reply: Some(ReplyRef {
            root: match their_post.value.reply {
                Some(r) => r.root,
                None => parent.clone(),
            },
            parent,
        }),

        text: "tada!".to_owned(),
    };

    client.create_record(&my_post, my_handle).await?;

    Ok(())
}
