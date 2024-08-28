use crate::common::MinaMeshContext;
use anyhow::Result;
use cynic::QueryBuilder;
pub use mesh::models::{NetworkIdentifier, NetworkListResponse};
use mina_mesh_graphql::QueryNetworkId;

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/network.ml#L162
pub async fn list() -> Result<NetworkListResponse> {
  let context = MinaMeshContext::from_env().await?;
  let QueryNetworkId { network_id } = context.graphql(QueryNetworkId::build(())).await?;
  Ok(NetworkListResponse::new(vec![NetworkIdentifier::new(
    "mina".into(),
    network_id.into(),
  )]))
}
