use anyhow::Result;
use coinbase_mesh::models::{NetworkIdentifier, NetworkListResponse};
use cynic::QueryBuilder;

use crate::{graphql::QueryNetworkId, MinaMesh, MinaMeshError};

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/network.ml#L162
impl MinaMesh {
  pub async fn network_list(&self) -> Result<NetworkListResponse, MinaMeshError> {
    let QueryNetworkId { network_id } = self.graphql_client.send(QueryNetworkId::build(())).await?;
    let (chain_id, network_id) = network_id.split_once(':').map_or_else(
      || ("unknown".to_string(), "unknown".to_string()),
      |(chain, network)| (chain.to_string(), network.to_string()),
    );
    Ok(NetworkListResponse::new(vec![NetworkIdentifier::new(chain_id, network_id)]))
  }
}
