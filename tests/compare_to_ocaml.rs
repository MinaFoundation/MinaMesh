mod fixtures;

use anyhow::Result;
use futures::future::join_all;
use mina_mesh::{test::ResponseComparisonContext, MinaMeshConfig};
use serde::Serialize;

const LEGACY_ENDPOINT: &str = "https://rosetta-devnet.minaprotocol.network";

async fn assert_responses_eq<T: Serialize>(subpath: &str, reqs: &[T]) -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let comparison_ctx = ResponseComparisonContext::new(mina_mesh, LEGACY_ENDPOINT.to_string());
  let assertion_futures: Vec<_> = reqs
    .iter()
    .map(|r| serde_json::to_vec(r).map(|body| comparison_ctx.assert_responses_eq(subpath, Some(body))).unwrap())
    .collect();

  join_all(assertion_futures).await;
  Ok(())
}

async fn assert_responses_contain<T: Serialize>(subpath: &str, reqs: &[T], fragment: &str) -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let comparison_ctx = ResponseComparisonContext::new(mina_mesh, LEGACY_ENDPOINT.to_string());
  let assertion_futures: Vec<_> = reqs
    .iter()
    .map(|r| {
      serde_json::to_vec(r).map(|body| comparison_ctx.assert_responses_contain(subpath, Some(body), fragment)).unwrap()
    })
    .collect();

  join_all(assertion_futures).await;
  Ok(())
}

#[tokio::test]
async fn search_transactions_test() -> Result<()> {
  let (subpath, reqs) = fixtures::search_transactions();
  assert_responses_eq(subpath, &reqs).await
}

#[tokio::test]
async fn network_list() -> Result<()> {
  let (subpath, reqs) = fixtures::network_list();
  assert_responses_eq(subpath, &reqs).await
}

#[tokio::test]
async fn network_options() -> Result<()> {
  let (subpath, reqs) = fixtures::network_options();
  assert_responses_contain(subpath, &reqs, "node_version").await
}

#[tokio::test]
async fn network_status() -> Result<()> {
  let (subpath, reqs) = fixtures::network_status();
  assert_responses_contain(subpath, &reqs, "\"stage\": \"Synced\"").await
}

#[tokio::test]
async fn mempool() -> Result<()> {
  let (subpath, reqs) = fixtures::mempool();
  assert_responses_eq(subpath, &reqs).await
}

#[tokio::test]
async fn mempool_transaction() -> Result<()> {
  let (subpath, reqs) = fixtures::mempool_transaction();
  assert_responses_contain(subpath, &reqs, "\"message\": \"Transaction not found").await
}

#[tokio::test]
async fn account_balance() -> Result<()> {
  let (subpath, reqs) = fixtures::account_balance();
  assert_responses_eq(subpath, &reqs).await
}

#[tokio::test]
async fn account_balance_not_exists() -> Result<()> {
  let (subpath, reqs) = fixtures::account_balance_not_exists();
  assert_responses_contain(subpath, &reqs, "\"message\": \"Account not found").await
}

#[tokio::test]
async fn block() -> Result<()> {
  let (subpath, reqs) = fixtures::block();
  assert_responses_eq(subpath, &reqs).await
}

#[tokio::test]
async fn block_not_found() -> Result<()> {
  let (subpath, reqs) = fixtures::block_not_found();
  assert_responses_contain(subpath, &reqs, "\"message\": \"Block not found").await
}
