mod fixtures;

use anyhow::Result;
use futures::future::join_all;
use mina_mesh::{test::ResponseComparisonContext, MinaMeshConfig};
use serde::Serialize;

const LEGACY_ENDPOINT: &str = "https://rosetta-devnet.minaprotocol.network";

async fn compare_responses<T: Serialize>(subpath: &str, reqs: &[T]) -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let comparison_ctx = ResponseComparisonContext::new(mina_mesh, LEGACY_ENDPOINT.to_string());
  let assertion_futures: Vec<_> = reqs
    .iter()
    .map(|r| serde_json::to_vec(r).map(|body| comparison_ctx.assert_responses_eq(subpath, Some(body))).unwrap())
    .collect();
  join_all(assertion_futures).await;
  Ok(())
}

#[tokio::test]
async fn search_transactions() -> Result<()> {
  let (subpath, reqs) = fixtures::search_transactions();
  compare_responses(subpath, &reqs).await
}
