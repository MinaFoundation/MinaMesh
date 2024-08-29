use anyhow::{bail, Context, Result};
use cynic::http::ReqwestExt;
use reqwest::Client;

#[derive(Debug)]
pub struct GraphQLClient {
  mina_proxy_url: String,
  client: Client,
}

impl GraphQLClient {
  pub fn new(mina_proxy_url: String) -> Self {
    Self {
      mina_proxy_url,
      client: Client::new(),
    }
  }

  pub async fn send<ResponseData, Vars>(&self, operation: cynic::Operation<ResponseData, Vars>) -> Result<ResponseData>
  where
    Vars: serde::Serialize,
    ResponseData: serde::de::DeserializeOwned + 'static,
  {
    let response = self
      .client
      .post(self.mina_proxy_url.to_owned())
      .run_graphql(operation)
      .await
      .context("Failed to run GraphQL query")?;
    if let Some(errors) = response.errors {
      bail!(errors
        .iter()
        .map(|err| err.message.clone())
        .collect::<Vec<String>>()
        .join("\n\n"));
    } else if let Some(data) = response.data {
      Ok(data)
    } else {
      bail!("No data contained in GraphQL response");
    }
  }
}
