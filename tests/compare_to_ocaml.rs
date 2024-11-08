mod fixtures;
mod test_util;

use anyhow::Result;
use futures::future::join_all;
use mina_mesh::MinaMeshConfig;
use test_util::ResponseComparisonContext;

const LEGACY_ENDPOINT: &str = "https://rosetta-devnet.minaprotocol.network";

#[tokio::test]
async fn main() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let comparison_ctx = ResponseComparisonContext::new(mina_mesh, LEGACY_ENDPOINT.to_string());
  let groups = fixtures::groups();
  let assertion_futures_result: Result<Vec<_>, _> = groups
    .into_iter()
    .map(|(subpath, reqs)| -> Result<Vec<_>, _> {
      reqs
        .into_iter()
        .map(|(i, r)| serde_json::to_vec(&r).map(|body| comparison_ctx.assert_responses_eq(i, subpath, body)))
        .collect()
    })
    .collect();
  let assertion_futures: Vec<_> = assertion_futures_result?.into_iter().flatten().collect();
  join_all(assertion_futures).await;
  Ok(())
}
