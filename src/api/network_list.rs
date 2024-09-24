use anyhow::Result;
use cynic::QueryBuilder;
pub use mesh::models::{NetworkIdentifier, NetworkListResponse};

use crate::{MinaMesh, graphql::QueryNetworkId};

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/network.ml#L162
impl MinaMesh {
  pub async fn network_list(&self) -> Result<NetworkListResponse> {
    let QueryNetworkId { network_id } = self.graphql_client.send(QueryNetworkId::build(())).await?;
    Ok(NetworkListResponse::new(vec![NetworkIdentifier::new("mina".to_string(), network_id.to_string())]))
  }
}
