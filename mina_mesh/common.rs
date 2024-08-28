use anyhow::{anyhow, bail, Context as AnyhowContext, Result};
use cynic::{http::ReqwestExt, QueryBuilder};
pub use mesh::models::{AccountIdentifier, NetworkIdentifier};
use mina_mesh_graphql::QueryNetworkId;
use reqwest::Client;
use serde::Deserialize;
use sqlx::PgPool;
use std::vec::Vec;

#[derive(Deserialize, Debug, Default)]
pub struct MinaMeshEnv {
  #[serde(default = "default_mina_proxy_url")]
  mina_proxy_url: String,
  #[serde(default = "default_database_url")]
  database_url: String,
  #[serde(default = "default_genesis_block_identifier_height")]
  pub genesis_block_identifier_height: i64,
  #[serde(default = "default_genesis_block_identifier_state_hash")]
  pub genesis_block_identifier_state_hash: String,
}

fn default_mina_proxy_url() -> String {
  "https://mainnet.minaprotocol.network/graphql".to_string()
}

fn default_database_url() -> String {
  "postgres://mina:whatever@localhost:5432/archive".to_string()
}

fn default_genesis_block_identifier_height() -> i64 {
  359605
}

fn default_genesis_block_identifier_state_hash() -> String {
  "3NK4BpDSekaqsG6tx8Nse2zJchRft2JpnbvMiog55WCr5xJZaKeP".to_string()
}

pub struct MinaMeshContext {
  pub env: MinaMeshEnv,
  pub pool: PgPool,
  client: Client,
}

impl MinaMeshContext {
  pub async fn from_env() -> Result<Self> {
    let config = envy::from_env::<MinaMeshEnv>().with_context(|| "Failed to parse config from env")?;
    let database_url = config.database_url.clone();
    Ok(Self {
      env: config,
      pool: PgPool::connect(database_url.as_str()).await?,
      client: Client::new(),
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
      .post(&self.env.mina_proxy_url)
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

impl TryInto<MinaAccountIdentifier> for AccountIdentifier {
  type Error = anyhow::Error;
  fn try_into(self) -> Result<MinaAccountIdentifier> {
    let token_id = match self.metadata {
      None => DEFAULT_TOKEN_ID.to_string(),
      Some(serde_json::Value::Object(map)) => map.get("token_id").map(|v| v.to_string()).context("")?,
      _ => Err(anyhow!(""))?,
    };
    Ok(MinaAccountIdentifier {
      public_key: self.address,
      token_id,
    })
  }
}
