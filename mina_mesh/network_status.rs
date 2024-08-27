// TODO: get genesis block identifier from env

use crate::common::MinaMeshContext;
use anyhow::{Context, Result};
use cynic::QueryBuilder;
use mesh::models::{BlockIdentifier, NetworkStatusResponse, Peer};
use mina_mesh_graphql::{Block2, DaemonStatus, QueryNetworkStatus};

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/network.ml#L201
pub async fn status(context: &MinaMeshContext) -> Result<NetworkStatusResponse> {
  let QueryNetworkStatus {
    best_chain,
    daemon_status: DaemonStatus { peers },
    sync_status,
  } = context.graphql(QueryNetworkStatus::build(())).await?;
  let blocks = best_chain.context("")?;
  let first_block = blocks.first().context("")?;
  let Block2 {
    protocol_state,
    state_hash,
  } = first_block;
  let oldest_block = sqlx::query_file!("sql/oldest_block.sql")
    .fetch_one(&context.pool)
    .await?;
  Ok(NetworkStatusResponse {
    peers: Some(peers.into_iter().map(|peer| Peer::new(peer.peer_id)).collect()),
    current_block_identifier: Box::new(BlockIdentifier::new(
      protocol_state.consensus_state.block_height.0.parse::<i64>()?,
      state_hash.0.clone(),
    )),
    current_block_timestamp: protocol_state.blockchain_state.utc_date.0.parse::<i64>()?,
    genesis_block_identifier: Box::new(BlockIdentifier::new(
      context.config.genesis_block_identifier_height,
      context.config.genesis_block_identifier_state_hash.clone(),
    )),
    oldest_block_identifier: Some(Box::new(BlockIdentifier::new(
      oldest_block.height,
      oldest_block.state_hash,
    ))),
    sync_status: Some(Box::new(sync_status.into())),
  })
}
