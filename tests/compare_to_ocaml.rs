use anyhow::Result;
use mina_mesh::{
  models::{AccountBalanceRequest, AccountIdentifier, NetworkIdentifier, PartialBlockIdentifier},
  test::ResponseComparisonContext,
  MinaMeshConfig,
};

const LEGACY_ENDPOINT: &str = "https://rosetta-devnet.minaprotocol.network";

#[tokio::test]
async fn main() -> Result<()> {
  tracing_subscriber::fmt::init();
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let comparison_ctx = ResponseComparisonContext::new(mina_mesh, LEGACY_ENDPOINT.to_string());

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

  comparison_ctx.assert_responses_eq("/account/balance", Some(req_json)).await?;

  Ok(())
}
