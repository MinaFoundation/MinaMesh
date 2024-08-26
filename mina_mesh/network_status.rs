// TODO: get genesis block identifier from env

use crate::common::MinaMeshContext;
use anyhow::Result;
use cynic::QueryBuilder;
use mesh::models::{BlockIdentifier, NetworkStatusResponse, Peer};
use mina_mesh_graphql::{Block2, QueryNetworkStatus};

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/network.ml#L201
pub async fn status() -> Result<NetworkStatusResponse> {
  let context = MinaMeshContext::from_env().await?;
  let QueryNetworkStatus {
    best_chain,
    daemon_status,
    sync_status,
  } = context.graphql(QueryNetworkStatus::build(())).await?;
  let Block2 {
    protocol_state,
    state_hash,
  } = &best_chain.unwrap()[0];
  let oldest_block = sqlx::query_file!("sql/oldest_block.sql")
    .fetch_one(&context.pool)
    .await?;
  Ok(NetworkStatusResponse {
    peers: Some(
      daemon_status
        .peers
        .iter()
        .map(|peer| Peer::new(peer.peer_id.clone()))
        .collect(),
    ),
    current_block_identifier: Box::new(BlockIdentifier::new(
      protocol_state.consensus_state.block_height.0.parse::<i64>()?,
      state_hash.0.clone(),
    )),
    current_block_timestamp: protocol_state.blockchain_state.utc_date.0.parse::<i64>()?,
    genesis_block_identifier: Box::new(BlockIdentifier::new(
      359605,
      "3NK4BpDSekaqsG6tx8Nse2zJchRft2JpnbvMiog55WCr5xJZaKeP".into(),
    )),
    oldest_block_identifier: Some(Box::new(BlockIdentifier::new(
      oldest_block.height,
      oldest_block.state_hash,
    ))),
    sync_status: Some(Box::new(sync_status.into())),
  })
}
