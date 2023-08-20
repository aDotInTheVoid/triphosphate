use cid::Version;
use triphosphate::{
    client::Client,
    lex::com::atproto::{identity, sync},
};
use triphosphate_vocab::{Did, Handle, StringFormat};

#[tokio::test]
async fn get_head() -> anyhow::Result<()> {
    let client = triphosphate::client::Client::new()?;
    let did = Did::from_str("did:plc:ewvi7nxzyoun6zhxrhs64oiz").unwrap();

    let cid = sync::get_head(&client, &sync::get_head::Params { did }).await?;

    assert_ne!(cid.root.version(), Version::V0);

    Ok(())
}

#[tokio::test]
async fn resolove_atproto_dotcom() -> anyhow::Result<()> {
    let handle = Handle::from_str("atproto.com")?;
    let client = Client::new()?;

    let resolved_did = Did::from_str("did:plc:ewvi7nxzyoun6zhxrhs64oiz").unwrap();

    let resp =
        identity::resolve_handle(&client, &identity::resolve_handle::Params { handle }).await?;

    assert_eq!(resp.did, resolved_did);

    Ok(())
}
