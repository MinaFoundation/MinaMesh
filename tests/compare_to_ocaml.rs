mod compare;

use anyhow::Result;
use futures::future::join_all;
use mina_mesh::{test::ResponseComparisonContext, MinaMeshConfig};

const LEGACY_ENDPOINT: &str = "https://rosetta-devnet.minaprotocol.network";

#[tokio::test]
async fn main() -> Result<()> {
  let mina_mesh = MinaMeshConfig::from_env().to_mina_mesh().await?;
  let comparison_ctx = ResponseComparisonContext::new(mina_mesh, LEGACY_ENDPOINT.to_string());
  let groups = compare::groups();
  let assertion_futures_result: Result<Vec<_>, _> = groups
    .into_iter()
    .map(|(subpath, reqs)| -> Result<Vec<_>, _> {
      reqs
        .iter()
        .map(|r| serde_json::to_vec(r).map(|body| comparison_ctx.assert_responses_eq(subpath, Some(body))))
        .collect()
    })
    .collect();
  let assertion_futures: Vec<_> = assertion_futures_result?.into_iter().flatten().collect();
  join_all(assertion_futures).await;
  Ok(())
}
