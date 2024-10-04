// TODO: why does OCaml implementation query for the `daemon_status` and
// `initial_peers`?
#![allow(clippy::just_underscores_and_digits)]

use anyhow::Result;
use coinbase_mesh::models::{MempoolResponse, TransactionIdentifier};
use cynic::QueryBuilder;

use crate::{graphql::QueryMempool, MinaMesh};

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/mempool.ml#L56
impl MinaMesh {
  pub async fn mempool(&self) -> Result<MempoolResponse> {
    let QueryMempool { daemon_status: _0, initial_peers: _1, pooled_user_commands } =
      self.graphql_client.send(QueryMempool::build(())).await?;
    let hashes = pooled_user_commands
      .into_iter()
      .map(|command| TransactionIdentifier::new(command.hash.0))
      .collect::<Vec<TransactionIdentifier>>();
    Ok(MempoolResponse::new(hashes))
  }
}
