use std::{borrow::Borrow, thread::sleep, time::Duration};

use anyhow::Result;
use axum::{
  body::HttpBody,
  extract::ConnectInfo,
  http::{self, Request, StatusCode},
  serve::Serve,
  Router,
};
use coinbase_mesh::apis::{
  self,
  configuration::{self, Configuration},
};
use http_body_util::BodyExt;
use mina_mesh::{
  create_router,
  models::{
    AccountBalanceRequest, AccountBalanceResponse, AccountIdentifier, NetworkIdentifier, PartialBlockIdentifier,
  },
  MinaMesh, MinaMeshConfig, ServeCommand,
};
use pretty_assertions::assert_eq;
use reqwest::{Body, Client};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::{
  sync::{oneshot, watch},
  task,
};
use tower::{Service, ServiceExt};
use tower_http::trace::TraceLayer;

const LEGACY_ENDPOINT: &str = "https://rosetta-online-mainnet.minaprotocol.network";
const SOME_ACCOUNT: &str = "B62qkYHGYmws5CYa3phYEKoZvrENTegEhUJYMhzHUQe5UZwCdWob8zv";

struct CompareLegacy {
  client: Client,
  router: Router,
}

impl CompareLegacy {
  fn new(mina_mesh: MinaMesh) -> Self {
    Self { client: Client::new(), router: create_router(mina_mesh, false) }
  }

  async fn compare<I, O>(self, req: &I) -> Result<O>
  where
    I: Serialize,
    O: DeserializeOwned,
  {
    let req = Request::builder()
      .method("POST")
      .uri("/account/balance")
      .header(http::header::CONTENT_TYPE, "application/json")
      .body(serde_json::to_string(req)?)
      .unwrap();
    let oneshot_result = self.router.oneshot(req).await?.into_body().collect().await?.to_bytes();
    let result = serde_json::from_slice::<O>(&oneshot_result[..])?;
    Ok(result)
  }
}

#[tokio::test]
async fn main() -> Result<()> {
  tracing_subscriber::fmt::init();
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let compare_legacy = CompareLegacy::new(mina_mesh);

  let account_balance_request = AccountBalanceRequest {
    account_identifier: Box::new(AccountIdentifier { address: SOME_ACCOUNT.into(), sub_account: None, metadata: None }),
    block_identifier: Some(Box::new(PartialBlockIdentifier { index: Some(6265), hash: None })),
    currencies: None,
    network_identifier: Box::new(NetworkIdentifier {
      blockchain: "mina".into(),
      network: "mainnet".into(),
      sub_network_identifier: None,
    }),
  };

  let result: AccountBalanceResponse = compare_legacy.compare(&account_balance_request).await?;
  tracing::info!("HELLO {:?}", result);

  // let slice = oneshot_result.into_body().collect().await.unwrap().to_bytes();
  // let result: AccountBalanceResponse = serde_json::from_slice(&slice[..])?;
  // println!("{:?}", result);

  // let legacy_result = client
  //   .post(format!("{LEGACY_ENDPOINT}/account/balance"))
  //   .json(&account_balance_request)
  //   .send()
  //   .await?
  //   .json::<AccountBalanceResponse>()
  //   .await?;

  // let result = server
  //   .post("/account/balance")
  //   .json::<AccountBalanceRequest>(&account_balance_request)
  //   .await
  //   .json::<AccountBalanceResponse>();
  // println!("{:?}", result);
  // let legacy_result = client
  //   .post("http://0.0.0.0:3000/account/balance")
  //   .json(&account_balance_request)
  //   .send()
  //   .await?
  //   .json::<AccountBalanceResponse>()
  //   .await?;

  // println!("{:?}", legacy_result);

  Ok(())
}
