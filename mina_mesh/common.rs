use anyhow::{bail, Context as AnyhowContext, Result};
use cynic::{http::ReqwestExt, QueryBuilder};
use mesh::models::AccountIdentifier;
use mesh::models::NetworkIdentifier;
use mina_mesh_graphql::QueryNetworkId;
use reqwest::Client;
use serde::Deserialize;
use sqlx::PgPool;
use std::vec::Vec;

#[derive(Deserialize, Debug, Default)]
pub struct Config {
  #[serde(default = "default_mina_proxy_url")]
  mina_proxy_url: String,
  #[serde(default = "default_archive_url")]
  archive_url: String,
  #[serde(default = "default_database_url")]
  database_url: String,
  genesis_block_identifier: String,
}

fn default_mina_proxy_url() -> String {
  "https://mainnet.minaprotocol.network/graphql".to_string()
}

fn default_archive_url() -> String {
  "https://api.minascan.io/archive/devnet/v1/graphql".to_string()
}

fn default_database_url() -> String {
  "postgres://mina:whatever@localhost:5432/archive".to_string()
}

pub struct MinaMeshContext {
  config: Config,
  client: Client,
  pub pool: PgPool,
}

impl MinaMeshContext {
  pub async fn from_env() -> Result<Self> {
    let config = envy::from_env::<Config>().with_context(|| "Failed to parse config from env")?;
    let database_url = config.database_url.clone();
    Ok(Self {
      config,
      client: Client::new(),
      pool: PgPool::connect(database_url.as_str()).await?,
    })
  }

  pub async fn network_health_check(&self, network_identifier: NetworkIdentifier) -> Result<bool> {
    let QueryNetworkId { network_id } = self.graphql(QueryNetworkId::build(())).await?;
    if network_identifier.blockchain == "MINA" {
      unimplemented!();
    }
    if &network_identifier.network == &network_id {
      unimplemented!();
    }
    Ok(true)
  }

  pub async fn graphql<ResponseData, Vars>(
    &self,
    operation: cynic::Operation<ResponseData, Vars>,
  ) -> Result<ResponseData>
  where
    Vars: serde::Serialize,
    ResponseData: serde::de::DeserializeOwned + 'static,
  {
    let response = self
      .client
      .post(&self.config.mina_proxy_url)
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

#[derive(Debug)]
pub struct MinaAccountIdentifier {
  pub public_key: String,
  pub token_id: String,
}

// cspell:disable-next-line
const DEFAULT_TOKEN_ID: &str = "wSHV2S4qX9jFsLjQo8r1BsMLH2ZRKsZx6EJd1sbozGPieEC4Jf";

impl Into<MinaAccountIdentifier> for AccountIdentifier {
  fn into(self) -> MinaAccountIdentifier {
    let token_id = match self.metadata {
      Some(serde_json::Value::Object(map)) => map.get("token_id").map(|v| v.as_str().unwrap().to_string()),
      None => Some(DEFAULT_TOKEN_ID.to_string()),
      _ => unimplemented!(),
    }
    .unwrap();
    MinaAccountIdentifier {
      public_key: self.address,
      token_id,
    }
  }
}