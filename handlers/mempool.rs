// TODO: why does OCaml implementation query for the `daemon_status` and `initial_peers`?

use crate::MinaMesh;
use anyhow::Result;
use cynic::QueryBuilder;
pub use mesh::models::MempoolResponse;
pub use mesh::models::TransactionIdentifier;
use mina_mesh_graphql::QueryMempool;

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/mempool.ml#L56
impl MinaMesh {
  pub async fn mempool(&self) -> Result<MempoolResponse> {
    let QueryMempool {
      daemon_status: _0,
      initial_peers: _1,
      pooled_user_commands,
    } = self.graphql_client.send(QueryMempool::build(())).await?;
    let hashes = pooled_user_commands
      .into_iter()
      .map(|command| TransactionIdentifier::new(command.hash.0))
      .collect::<Vec<TransactionIdentifier>>();
    Ok(MempoolResponse::new(hashes))
  }
}
