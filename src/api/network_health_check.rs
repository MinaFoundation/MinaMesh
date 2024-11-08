use anyhow::Result;
use coinbase_mesh::models::NetworkIdentifier;
use cynic::QueryBuilder;

use crate::{graphql::QueryNetworkId, MinaMesh};

impl MinaMesh {
  pub async fn network_health_check(&self, network_identifier: NetworkIdentifier) -> Result<bool> {
    let QueryNetworkId { network_id } =
      self.graphql_client.send(&network_identifier.clone().into(), QueryNetworkId::build(())).await?;
    if network_identifier.blockchain == "mina" {
      unimplemented!();
    }
    if network_identifier.network == network_id {
      unimplemented!();
    }
    Ok(true)
  }
}
