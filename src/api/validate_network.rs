use std::time::Instant;

use anyhow::Result;
use coinbase_mesh::models::NetworkIdentifier;
use cynic::QueryBuilder;

use crate::{graphql::QueryNetworkId, MinaMesh, MinaMeshError};

impl MinaMesh {
  // Validate that the network identifier matches the network id of the GraphQL
  // server
  pub async fn validate_network(&self, network_identifier: &NetworkIdentifier) -> Result<(), MinaMeshError> {
    let cache_key = "network_id".to_string();

    // Step 1: Check the cache
    if let Some(cached_entry) = self.cache.get(&cache_key) {
      let (cached_network_id, timestamp) = &*cached_entry;
      if timestamp.elapsed() < self.cache_ttl {
        return self.compare_network_ids(cached_network_id, network_identifier);
      }
    }

    // Step 2: Fetch from GraphQL if cache is empty or expired
    let QueryNetworkId { network_id } = self.graphql_client.send(QueryNetworkId::build(())).await?;

    // Step 3: Update the cache
    self.cache.insert(cache_key, (network_id.clone(), Instant::now()));

    // Step 4: Compare the fetched value with the expected one
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
