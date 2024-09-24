use cynic::http::ReqwestExt;
use reqwest::Client;

use crate::MinaMeshError;

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

  pub async fn send<ResponseData, Vars>(
    &self,
    operation: cynic::Operation<ResponseData, Vars>,
  ) -> Result<ResponseData, MinaMeshError>
  where
    Vars: serde::Serialize,
    ResponseData: serde::de::DeserializeOwned + 'static,
  {
    let response = self
      .client
      .post(self.mina_proxy_url.to_owned())
      .run_graphql(operation)
      .await?;
    if let Some(errors) = response.errors {
      Err(MinaMeshError::GraphqlMinaQuery(
        errors
          .into_iter()
          .map(|err| err.message)
          .collect::<Vec<_>>()
          .join("\n\n"),
      ))
    } else if let Some(data) = response.data {
      Ok(data)
    } else {
      Err(MinaMeshError::GraphqlMinaQuery("".to_string()))
    }
  }
}
