use anyhow::Context;
use reqwest::RequestBuilder;
use serde::{de::DeserializeOwned, Serialize};

use triphosphate_vocab::{AtIdentifier, AtUri, Cid};

use crate::{
    lex::com::atproto::{
        repo::{create_record, get_record},
        server::create_session,
    },
    AsParams, LexRecord,
};

pub struct Client {
    http: reqwest::Client,
    jwt: Option<String>,
}

impl Client {
    pub fn new() -> anyhow::Result<Self> {
        let http = reqwest::Client::builder()
            .user_agent("triphosphate") // TODO: Let users set this.
            .build()?;

        Ok(Self { http, jwt: None })
    }

    pub fn set_jwt(&mut self, jwt: String) {
        self.jwt = Some(jwt)
    }

    pub async fn login(
        &mut self,
        identifier: &str,
        password: &str,
    ) -> anyhow::Result<create_session::Responce> {
        let args = &create_session::Args {
            identifier: identifier.to_owned(),
            password: password.to_owned(),
        };

        let resp = create_session(self, args).await?;

        self.set_jwt(resp.access_jwt.clone());

        Ok(resp)
    }

    pub(crate) async fn do_query<Params: AsParams, Resp: DeserializeOwned>(
        &self,
        id: &'static str,
        params: &Params,
    ) -> anyhow::Result<Resp> {
        // TODO: Error handling.

        let url = reqwest::Url::parse_with_params(&self.xrpc_url(id), params.as_params())
            .context("failed to build URL")?;

        self.exec(self.http.get(url)).await
    }

    pub(crate) async fn do_procedure<Input: Serialize, Resp: DeserializeOwned>(
        &self,
        id: &'static str,
        input: &Input,
    ) -> anyhow::Result<Resp> {
        // TODO: Error handling

        self.exec(self.http.post(self.xrpc_url(id)).json(input))
            .await
    }

    fn xrpc_url(&self, id: &'static str) -> String {
        // TODO: Configuable base.
        format!("https://bsky.social/xrpc/{id}")
    }

    async fn exec<T: DeserializeOwned>(&self, mut req: RequestBuilder) -> anyhow::Result<T> {
        if let Some(jwt) = &self.jwt {
            req = req.bearer_auth(jwt)
        }

        let resp = req.send().await?;

        if let Err(e) = resp.error_for_status_ref() {
            let body = resp.bytes().await?;
            return Err(e).with_context(|| format!("got {body:?}"));
        }

        let resp_body = resp.json::<T>().await.context("failed to decode to json")?;

        Ok(resp_body)
    }

    pub async fn create_record<R: LexRecord>(
        &self,
        record: &R,
        repo: AtIdentifier,
    ) -> anyhow::Result<create_record::Responce> {
        create_record(
            self,
            &create_record::Args {
                collection: R::NSID,
                record: triphosphate_vocab::to_any(record)?, // PERF: Avoid this
                repo,
                // TODO: Make configurable
                rkey: None,
                swap_commit: None,
                validate: Some(true),
            },
        )
        .await
    }

    pub async fn get_record<R: LexRecord>(
        &self,
        repo: AtIdentifier,
        rkey: String,
    ) -> anyhow::Result<GetRecord<R>> {
        let resp = get_record(
            self,
            &get_record::Params {
                cid: None,
                collection: R::NSID,
                repo,
                rkey,
            },
        )
        .await
        .context("failed to make XRPC call")?;

        let value = triphosphate_vocab::from_any(resp.value)
            .context("failed to convert responce to expected type")?;

        Ok(GetRecord {
            cid: resp.cid,
            uri: resp.uri,
            value,
        })
    }
}

#[derive(Debug)]
pub struct GetRecord<T> {
    pub cid: Option<Cid>,
    pub value: T,
    pub uri: AtUri,
}
