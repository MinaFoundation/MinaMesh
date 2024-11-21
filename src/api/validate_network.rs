use anyhow::Result;
use coinbase_mesh::models::NetworkIdentifier;
use cynic::QueryBuilder;

use crate::{graphql::QueryNetworkId, MinaMesh, MinaMeshError};

impl MinaMesh {
  // Validate that the network identifier matches the network id of the GraphQL
  // server
  pub async fn validate_network(&self, network_identifier: &NetworkIdentifier) -> Result<(), MinaMeshError> {
    let QueryNetworkId { network_id } = self.graphql_client.send(QueryNetworkId::build(())).await?;
    let expected_network_id = format!("{}:{}", network_identifier.blockchain, network_identifier.network);
    if network_id != expected_network_id {
      Err(MinaMeshError::NetworkDne(expected_network_id, network_id))
    } else {
      Ok(())
    }
  }
}
