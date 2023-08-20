use reqwest::RequestBuilder;
use serde::{de::DeserializeOwned, Serialize};

use crate::lex::com::atproto::server::create_session;

pub struct Client {
    http: reqwest::Client,
    jwt: Option<String>,
}

impl Client {
    pub fn new() -> reqwest::Result<Self> {
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
    ) -> reqwest::Result<create_session::Responce> {
        let args = &create_session::Args {
            identifier: identifier.to_owned(),
            password: password.to_owned(),
        };

        let resp = create_session(self, args).await?;

        self.set_jwt(resp.access_jwt.clone());

        Ok(resp)
    }

    pub(crate) async fn do_query<Params: Serialize, Resp: DeserializeOwned>(
        &self,
        id: &'static str,
        params: &Params,
    ) -> reqwest::Result<Resp> {
        // TODO: Error handling.
        self.exec(self.http.get(self.xrpc_url(id)).form(params))
            .await
    }

    pub(crate) async fn do_procedure<Input: Serialize, Resp: DeserializeOwned>(
        &self,
        id: &'static str,
        input: &Input,
    ) -> reqwest::Result<Resp> {
        // TODO: Error handling

        self.exec(self.http.post(self.xrpc_url(id)).json(input))
            .await
    }

    fn xrpc_url(&self, id: &'static str) -> String {
        // TODO: Configuable base.
        format!("https://bsky.social/xrpc/{id}")
    }

    async fn exec<T: DeserializeOwned>(&self, mut req: RequestBuilder) -> reqwest::Result<T> {
        if let Some(jwt) = &self.jwt {
            req = req.bearer_auth(jwt)
        }

        req.send().await?.json().await
    }
}
