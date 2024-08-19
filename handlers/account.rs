use super::Context;
use crate::graphql_generated::archive::{SomeDoc, SomeDocVariables};
use crate::graphql_generated::mina::{Account, BalanceQuery, BalanceQueryVariables, PublicKey};
use anyhow::Result;
use cynic::{http::ReqwestExt, QueryBuilder};
use mesh::models::{AccountBalanceResponse, Amount, BlockIdentifier, Currency};
use serde::Serialize;

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/account.ml#L11
async fn balance(public_key: String) -> Result<AccountBalanceResponse> {
  let context = Context::from_env()?;
  let operation = BalanceQuery::build(BalanceQueryVariables {
    public_key: PublicKey(public_key),
  });
  println!("{:?}", operation);
  let result = context
    .client
    .post(context.config.mina_proxy_url)
    .run_graphql(operation)
    .await?;
  if let Some(BalanceQuery {
    account: Some(Account { balance, .. }),
  }) = result.data
  {
    println!("Balance: {:?}", balance);
    return Ok(AccountBalanceResponse {
      balances: vec![],
      block_identifier: Box::new(BlockIdentifier {
        hash: "".into(),
        index: 1,
      }),
      metadata: None,
    });
  } else if result.errors.is_some() {
    anyhow::bail!("Failed to get balance: {:?}", result.errors);
  }
  anyhow::bail!("Failed to get balance");
}

fn coins() {}

#[cfg(test)]
mod test {
  use super::*;

  #[tokio::test]
  async fn first() {
    let x = balance("".into()).await;
    println!("{:?}", x);
  }
}
