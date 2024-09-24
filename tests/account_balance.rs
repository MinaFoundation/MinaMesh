use anyhow::Result;
use mina_mesh::{AccountBalanceRequest, AccountIdentifier, NetworkIdentifier, PartialBlockIdentifier, ServeCommand};

#[tokio::test]
async fn first() -> Result<()> {
  let mina_mesh = MinaMeshConfig::default().to_mina_mesh().await?;
  let result = mina_mesh
    .account_balance(AccountBalanceRequest {
      account_identifier: Box::new(AccountIdentifier {
        // cspell:disable-next-line
        address: "B62qmjJeM4Fd4FVghfhgwoE1fkEexK2Rre8WYKMnbxVwB5vtKUwvgMv".into(),
        sub_account: None,
        metadata: None,
      }),
      block_identifier: Some(Box::new(PartialBlockIdentifier {
        index: Some(371513),
        hash: None,
      })),
      currencies: None,
      network_identifier: Box::new(NetworkIdentifier {
        blockchain: "mina".into(),
        network: "mainnet".into(),
        sub_network_identifier: None,
      }),
    })
    .await;
  println!("{:?}", result);
  Ok(())
}

// TODO: add tests for accounts with similar properties to these:
// - B62qrQKS9ghd91shs73TCmBJRW9GzvTJK443DPx2YbqcyoLc56g1ny9"
// - B62qooos8xGyqtJGpT7eaoyGrABCf4vcAnzCtxPLNrf26M7FwAxHg1i"
