use anyhow::Result;
use cynic::QueryBuilder;
pub use mesh::models::{BlockRequest, BlockResponse, PartialBlockIdentifier};

use crate::{
  graphql::{QueryBlockTransactions, QueryBlockTransactionsVariables},
  MinaMesh, MinaMeshError, Wrapper,
};

#[derive(sqlx::Type, Debug, PartialEq, Eq)]
#[sqlx(type_name = "chain_status_type", rename_all = "lowercase")]
enum ChainStatus {
  Canonical,
  Pending,
  Orphaned,
}

#[derive(Debug, PartialEq, Eq, sqlx::FromRow)]
pub struct BlockMetadata {
  id: i32,
  block_winner_id: i32,
  chain_status: Option<ChainStatus>,
  creator_id: i32,
  global_slot_since_genesis: i64,
  global_slot_since_hard_fork: i64,
  height: i64,
  last_vrf_output: String,
  ledger_hash: String,
  min_window_density: i64,
  next_epoch_data_id: i32,
  state_hash: String,
  sub_window_densities: Vec<i64>,
  timestamp: String,
  total_currency: String,
  parent_hash: String,
  parent_id: Option<i32>,
  proposed_protocol_version_id: Option<i32>,
  protocol_version_id: i32,
  snarked_ledger_hash_id: i32,
  staking_epoch_data_id: i32,
  creator: String,
  winner: String,
}

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/block.ml#L7
impl MinaMesh {
  pub async fn block(&self, request: BlockRequest) -> Result<BlockResponse, MinaMeshError> {
    let block_identifier = *request.block_identifier;
    let metadata = match self.block_metadata(&block_identifier).await? {
      Some(metadata) => metadata,
      None => return Err(MinaMeshError::BlockMissing(Wrapper(&block_identifier).to_string())),
    };
    let block_transactions = self
      .graphql_client
      .send(QueryBlockTransactions::build(QueryBlockTransactionsVariables { state_hash: Some(&metadata.state_hash) }))
      .await
      .map_err(|_| MinaMeshError::ChainInfoMissing)?;
    println!("block_transactions: {:?}", block_transactions);
    unimplemented!()

    // Fetch transactions from DB
    // Internal commands, user commands, and zkapps commands

    // SQL command -> Rosetta/mesh transaction
    // Each command will originate multiple atomic Rosetta/mesh operations

    // Populate the block response from the fetched metadata, if any.

    // Ok(BlockResponse {
    //   block: Some(Box::new(Block::new(
    //     BlockIdentifier::new(0, "".to_string()),
    //     BlockIdentifier::new(0, "".to_string()),
    //     0,
    //     vec![],
    //   ))),
    //   other_transactions: Some(vec![]),
    // })
  }

  pub async fn block_metadata(
    &self,
    PartialBlockIdentifier { index, hash }: &PartialBlockIdentifier,
  ) -> Result<Option<BlockMetadata>, sqlx::Error> {
    if let (Some(index), Some(hash)) = (&index, &hash) {
      sqlx::query_file_as!(BlockMetadata, "sql/query_both.sql", hash.to_string(), index)
        .fetch_optional(&self.pg_pool)
        .await
    } else if let Some(index) = index {
      let record = sqlx::query_file!("sql/max_canonical_height.sql").fetch_one(&self.pg_pool).await?;
      if index <= &record.max_canonical_height.unwrap() {
        sqlx::query_file_as!(BlockMetadata, "sql/query_canonical.sql", index).fetch_optional(&self.pg_pool).await
      } else {
        sqlx::query_file_as!(BlockMetadata, "sql/query_pending.sql", index).fetch_optional(&self.pg_pool).await
      }
    } else if let Some(hash) = &hash {
      sqlx::query_file_as!(BlockMetadata, "sql/query_hash.sql", hash).fetch_optional(&self.pg_pool).await
    } else {
      sqlx::query_file_as!(BlockMetadata, "sql/query_best.sql").fetch_optional(&self.pg_pool).await
    }
  }
}
