use super::Context;
use crate::graphql_generated::mina::{BalanceQuery, BalanceQueryVariables, PublicKey};
use anyhow::Result;
use cynic::{http::ReqwestExt, QueryBuilder};
use serde::Serialize;

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/account.ml#L11
async fn balance(public_key: String) -> Result<()> {
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
  if let Some(v) = result.data {
    println!("{:?}", v);
    if let Some(w) = v.account {
      println!("{:?}", w.balance.total);
    }
  }
  if result.errors.is_some() {
    return Err(anyhow::anyhow!("Failed to get balance"));
  }
  Ok(())
}

fn coins() {}

#[cfg(test)]
mod test {
  use super::*;

  #[tokio::test]
  async fn first() {
    let x = balance("TODO".into()).await;
    println!("{:?}", x);
  }
}
