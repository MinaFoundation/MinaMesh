use anyhow::Result;
use cynic::QueryBuilder;
use mesh::models::NetworkIdentifier;

use crate::{MinaMesh, graphql::QueryNetworkId};

impl MinaMesh {
  pub async fn network_health_check(self, network_identifier: NetworkIdentifier) -> Result<bool> {
    let QueryNetworkId { network_id } = self.graphql_client.send(QueryNetworkId::build(())).await?;
    if network_identifier.blockchain == "MINA" {
      unimplemented!();
    }
    if network_identifier.network == network_id {
      unimplemented!();
    }
    Ok(true)
  }
}
