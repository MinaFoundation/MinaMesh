use anyhow::Result;
use mina_mesh::{
  create_router,
  models::{AccountBalanceRequest, AccountIdentifier, NetworkIdentifier, PartialBlockIdentifier},
  test::LegacyComparisonContext,
  MinaMeshConfig,
};
use reqwest::Client;

#[tokio::test]
async fn main() -> Result<()> {
  tracing_subscriber::fmt::init();
  let client = Client::new();
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let router = create_router(mina_mesh, false);
  let legacy_comparison_ctx = LegacyComparisonContext { client, router };

  let req_json = serde_json::to_string(&AccountBalanceRequest {
    account_identifier: Box::new(AccountIdentifier {
      address: "B62qkYHGYmws5CYa3phYEKoZvrENTegEhUJYMhzHUQe5UZwCdWob8zv".to_string(),
      sub_account: None,
      metadata: None,
    }),
    block_identifier: Some(Box::new(PartialBlockIdentifier { index: Some(6265), hash: None })),
    currencies: None,
    network_identifier: Box::new(NetworkIdentifier {
      blockchain: "mina".into(),
      network: "testnet".into(),
      sub_network_identifier: None,
    }),
  })?
  .into_bytes();

  legacy_comparison_ctx.assert("/account/balance", Some(req_json)).await?;

  Ok(())
}
