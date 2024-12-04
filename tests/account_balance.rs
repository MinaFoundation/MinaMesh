use anyhow::Result;
use futures::future::try_join_all;
use insta::assert_debug_snapshot;
use mina_mesh::{
  models::{AccountBalanceRequest, AccountBalanceResponse, AccountIdentifier, PartialBlockIdentifier},
  test::network_id,
  MinaMeshConfig, MinaMeshError,
};

#[tokio::test]
async fn responses() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let futures: Vec<_> = [
    // cspell:disable
    "B62qkYHGYmws5CYa3phYEKoZvrENTegEhUJYMhzHUQe5UZwCdWob8zv",
    "B62qnEdPB1V5YPEcGaETb19naLJV6sWdveCZEjSLhcVyrPcPWHkGGax",
    // "B62qooos8xGyqtJGpT7eaoyGrABCf4vcAnzCtxPLNrf26M7FwAxHg1i",
    // cspell:enable
    // TODO: reenable
  ]
  .into_iter()
  .map(|address| {
    mina_mesh.account_balance(AccountBalanceRequest {
      account_identifier: Box::new(AccountIdentifier { address: address.into(), sub_account: None, metadata: None }),
      block_identifier: Some(Box::new(PartialBlockIdentifier { index: Some(6265), hash: None })),
      currencies: None,
      network_identifier: Box::new(network_id()),
    })
  })
  .collect();
  let results: Vec<AccountBalanceResponse> = try_join_all(futures).await?;
  assert_debug_snapshot!(results);
  Ok(())
}

#[tokio::test]
async fn account_not_found_error() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let response = mina_mesh
    .account_balance(AccountBalanceRequest {
      account_identifier: Box::new(AccountIdentifier {
        //cspell:disable-next-line
        address: "B62qp3LaAUKQ76DdFYaQ7bj46HDTgpCaFpwhDqbjNJUC79Rf6x8CxV3".into(),
        sub_account: None,
        metadata: None,
      }),
      block_identifier: None,
      currencies: None,
      network_identifier: Box::new(network_id()),
    })
    .await;
  assert!(matches!(response, Err(MinaMeshError::AccountNotFound(_))));

  Ok(())
}
