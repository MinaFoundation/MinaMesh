use cynic::http::ReqwestExt;
use reqwest::Client;

use crate::{MinaMeshError, MinaNetwork};

#[derive(Debug)]
pub struct GraphQLClient {
  pub devnet_endpoint: String,
  pub mainnet_endpoint: String,
  pub client: Client,
}

impl GraphQLClient {
  pub async fn send<ResponseData, Vars>(
    &self,
    network: &MinaNetwork,
    operation: cynic::Operation<ResponseData, Vars>,
  ) -> Result<ResponseData, MinaMeshError>
  where
    Vars: serde::Serialize,
    ResponseData: serde::de::DeserializeOwned + 'static,
  {
    let endpoint = match network {
      MinaNetwork::Mainnet => &self.mainnet_endpoint,
      MinaNetwork::Devnet => &self.devnet_endpoint,
    };
    let response = self.client.post(endpoint).run_graphql(operation).await?;
    if let Some(errors) = response.errors {
      Err(MinaMeshError::GraphqlMinaQuery(errors.into_iter().map(|err| err.message).collect::<Vec<_>>().join("\n\n")))
    } else if let Some(data) = response.data {
      Ok(data)
    } else {
      Err(MinaMeshError::GraphqlMinaQuery("".to_string()))
    }
  }
}
