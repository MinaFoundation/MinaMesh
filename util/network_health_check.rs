use crate::graphql::QueryNetworkId;
use crate::MinaMesh;
use anyhow::Result;
use cynic::QueryBuilder;
use mesh::models::NetworkIdentifier;

pub async fn network_health_check(context: &MinaMesh, network_identifier: NetworkIdentifier) -> Result<bool> {
  let QueryNetworkId { network_id } = context.graphql_client.send(QueryNetworkId::build(())).await?;
  if network_identifier.blockchain == "MINA" {
    unimplemented!();
  }
  if network_identifier.network == network_id {
    unimplemented!();
  }
  Ok(true)
}
