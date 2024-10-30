use std::{borrow::Borrow, thread::sleep, time::Duration};

use anyhow::Result;
use axum::serve::Serve;
use mina_mesh::{
  models::{
    AccountBalanceRequest, AccountBalanceResponse, AccountIdentifier, NetworkIdentifier, PartialBlockIdentifier,
  },
  MinaMeshConfig, ServeCommand,
};
use pretty_assertions::assert_eq;
use reqwest::Client;
use tokio::{
  sync::{oneshot, watch},
  task,
};

const LEGACY_ENDPOINT: &str = "https://rosetta-online-mainnet.minaprotocol.network";
const SOME_ACCOUNT: &str = "B62qkYHGYmws5CYa3phYEKoZvrENTegEhUJYMhzHUQe5UZwCdWob8zv";

#[tokio::test]
async fn main() -> Result<()> {
  let client = Client::new();
  let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
  let server_fut = ServeCommand::new("0.0.0.0".to_string(), 3000, false)?.run(async move {
    let _ = shutdown_rx.await;
  });
  let server_handle = task::spawn(server_fut);

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

  // let legacy_result = client
  //   .post(format!("{LEGACY_ENDPOINT}/account/balance"))
  //   .json(&account_balance_request)
  //   .send()
  //   .await?
  //   .json::<AccountBalanceResponse>()
  //   .await?;

  let legacy_result = client
    .post("http://0.0.0.0:3000/account/balance")
    .json(&account_balance_request)
    .send()
    .await?
    .json::<AccountBalanceResponse>()
    .await?;

  println!("{:?}", legacy_result);

  let _ = shutdown_tx.send(());

  let _ = server_handle.await;

  Ok(())
}
