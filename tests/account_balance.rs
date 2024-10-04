use anyhow::Result;
use futures::future::try_join_all;
use insta::assert_debug_snapshot;
use mina_mesh::{
  models::{
    AccountBalanceRequest, AccountBalanceResponse, AccountIdentifier, NetworkIdentifier, PartialBlockIdentifier,
  },
  MinaMeshConfig,
};

#[tokio::test]
async fn responses() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let futures: Vec<_> = [
    // cspell:disable
    "B62qmjJeM4Fd4FVghfhgwoE1fkEexK2Rre8WYKMnbxVwB5vtKUwvgMv",
    "B62qrQKS9ghd91shs73TCmBJRW9GzvTJK443DPx2YbqcyoLc56g1ny9",
    // "B62qooos8xGyqtJGpT7eaoyGrABCf4vcAnzCtxPLNrf26M7FwAxHg1i",
    // cspell:enable
    // TODO: reenable
  ]
  .into_iter()
  .map(|address| {
    mina_mesh.account_balance(AccountBalanceRequest {
      account_identifier: Box::new(AccountIdentifier { address: address.into(), sub_account: None, metadata: None }),
      block_identifier: Some(Box::new(PartialBlockIdentifier { index: Some(371513), hash: None })),
      currencies: None,
      network_identifier: Box::new(NetworkIdentifier {
        blockchain: "mina".into(),
        network: "mainnet".into(),
        sub_network_identifier: None,
      }),
    })
  })
  .collect();
  let results: Vec<AccountBalanceResponse> = try_join_all(futures).await?;
  assert_debug_snapshot!(results);
  Ok(())
}
