use anyhow::Result;
use coinbase_mesh::models::NetworkIdentifier;
use cynic::QueryBuilder;

use crate::{graphql::QueryNetworkId, CacheKey::NetworkId, MinaMesh, MinaMeshError};

impl MinaMesh {
  // Validate that the network identifier matches the network id of the GraphQL
  // server
  pub async fn validate_network(&self, network_identifier: &NetworkIdentifier) -> Result<(), MinaMeshError> {
    // Check the cache
    if let Some(cached_network_id) = self.get_from_cache(NetworkId) {
      return self.compare_network_ids(&cached_network_id, network_identifier);
    }

    // Fetch from GraphQL if cache is empty or expired
    let QueryNetworkId { network_id } = self.graphql_client.send(QueryNetworkId::build(())).await?;
    self.insert_into_cache(NetworkId, network_id.clone());
    self.compare_network_ids(&network_id, network_identifier)
  }

  fn compare_network_ids(
    &self,
    fetched_network_id: &str,
    network_identifier: &NetworkIdentifier,
  ) -> Result<(), MinaMeshError> {
    let expected_network_id = format!("{}:{}", network_identifier.blockchain, network_identifier.network);

    if fetched_network_id != expected_network_id {
      Err(MinaMeshError::NetworkDne(expected_network_id, fetched_network_id.to_string()))
    } else {
      Ok(())
    }
  }
}
