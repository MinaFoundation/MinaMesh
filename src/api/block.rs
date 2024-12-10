use anyhow::Result;
use coinbase_mesh::models::{
  Block, BlockIdentifier, BlockRequest, BlockResponse, PartialBlockIdentifier, Transaction, TransactionIdentifier,
};
use serde::Serialize;
use serde_json::json;
use sqlx::FromRow;

use crate::{
  generate_internal_command_transaction_identifier, generate_operations_internal_command,
  generate_operations_user_command, generate_operations_zkapp_command, generate_transaction_metadata,
  util::DEFAULT_TOKEN_ID, ChainStatus, InternalCommandMetadata, InternalCommandType, MinaMesh, MinaMeshError,
  TransactionStatus, UserCommandMetadata, UserCommandType, ZkAppCommand,
};

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/block.ml#L7
impl MinaMesh {
  pub async fn block(&self, request: BlockRequest) -> Result<BlockResponse, MinaMeshError> {
    self.validate_network(&request.network_identifier).await?;
    let partial_block_identifier = *request.block_identifier;
    let metadata = match self.block_metadata(&partial_block_identifier).await? {
      Some(metadata) => metadata,
      None => return Err(MinaMeshError::BlockMissing(serde_json::to_string(&partial_block_identifier)?)),
    };
    let parent_block_metadata = match &metadata.parent_id {
      Some(parent_id) => {
        sqlx::query_file_as!(BlockMetadata, "sql/queries/query_id.sql", parent_id).fetch_optional(&self.pg_pool).await?
      }
      None => None,
    };
    let block_identifier = BlockIdentifier::new(metadata.height, metadata.state_hash.clone());
    let parent_block_identifier = match parent_block_metadata {
      Some(block_metadata) => BlockIdentifier::new(block_metadata.height, block_metadata.state_hash),
      None => block_identifier.clone(),
    };
    let (user_commands, internal_commands, zkapp_commands) = tokio::try_join!(
      self.user_commands(&metadata),
      self.internal_commands(&metadata),
      self.zkapp_commands(&metadata)
    )?;

    let all_commands: Vec<_> =
      internal_commands.into_iter().chain(user_commands.into_iter()).chain(zkapp_commands.into_iter()).collect();

    Ok(BlockResponse {
      block: Some(Box::new(Block {
        block_identifier: Box::new(block_identifier),
        parent_block_identifier: Box::new(parent_block_identifier),
        timestamp: metadata.timestamp.parse()?,
        transactions: all_commands,
        metadata: Some(json!({ "creator": metadata.creator })),
      })),
      other_transactions: None,
    })
  }

  // TODO: use default token value, check how to best handle this
  pub async fn user_commands(&self, metadata: &BlockMetadata) -> Result<Vec<Transaction>, MinaMeshError> {
    let metadata = sqlx::query_file_as!(UserCommandMetadata, "sql/queries/user_commands.sql", metadata.id)
      .fetch_all(&self.pg_pool)
      .await?;
    let transactions = metadata
      .into_iter()
      .map(|item| {
        let metadata = generate_transaction_metadata(&item);
        let operations = generate_operations_user_command(&item);

        Transaction {
          transaction_identifier: Box::new(TransactionIdentifier::new(item.hash.clone())),
          operations,
          metadata,
          related_transactions: None,
        }
      })
      .collect();
    Ok(transactions)
  }

  pub async fn internal_commands(&self, metadata: &BlockMetadata) -> Result<Vec<Transaction>, MinaMeshError> {
    let metadata =
      sqlx::query_file_as!(InternalCommandMetadata, "sql/queries/internal_commands.sql", metadata.id, DEFAULT_TOKEN_ID)
        .fetch_all(&self.pg_pool)
        .await?;

    let transactions = metadata
      .into_iter()
      .map(|item| {
        let transaction_identifier = generate_internal_command_transaction_identifier(
          &item.command_type,
          item.sequence_no,
          item.secondary_sequence_no,
          &item.hash,
        );
        Transaction::new(
          TransactionIdentifier::new(transaction_identifier),
          generate_operations_internal_command(&item),
        )
      })
      .collect();
    Ok(transactions)
  }

  pub async fn zkapp_commands(&self, metadata: &BlockMetadata) -> Result<Vec<Transaction>, MinaMeshError> {
    let metadata = sqlx::query_file_as!(ZkAppCommand, "sql/queries/zkapp_commands.sql", metadata.id, DEFAULT_TOKEN_ID)
      .fetch_all(&self.pg_pool)
      .await?;
    let transactions = zkapp_commands_to_transactions(metadata);
    Ok(transactions)
  }

  pub async fn block_metadata(
    &self,
    PartialBlockIdentifier { index, hash }: &PartialBlockIdentifier,
  ) -> Result<Option<BlockMetadata>, sqlx::Error> {
    if let (Some(index), Some(hash)) = (&index, &hash) {
      sqlx::query_file_as!(BlockMetadata, "sql/queries/query_both.sql", hash.to_string(), index)
        .fetch_optional(&self.pg_pool)
        .await
    } else if let Some(index) = index {
      let record = sqlx::query_file!("sql/queries/max_canonical_height.sql").fetch_one(&self.pg_pool).await?;
      if index <= &record.max_canonical_height.unwrap() {
        sqlx::query_file_as!(BlockMetadata, "sql/queries/query_canonical.sql", index)
          .fetch_optional(&self.pg_pool)
          .await
      } else {
        sqlx::query_file_as!(BlockMetadata, "sql/queries/query_pending.sql", index).fetch_optional(&self.pg_pool).await
      }
    } else if let Some(hash) = &hash {
      sqlx::query_file_as!(BlockMetadata, "sql/queries/query_hash.sql", hash).fetch_optional(&self.pg_pool).await
    } else {
      sqlx::query_file_as!(BlockMetadata, "sql/queries/query_best.sql").fetch_optional(&self.pg_pool).await
    }
  }
}

#[derive(Debug, PartialEq, Eq, FromRow, Serialize)]
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
  total_currency: Option<String>,
  parent_hash: String,
  parent_id: Option<i32>,
  proposed_protocol_version_id: Option<i32>,
  protocol_version_id: i32,
  snarked_ledger_hash_id: i32,
  staking_epoch_data_id: i32,
  creator: String,
  winner: String,
}

pub fn zkapp_commands_to_transactions(commands: Vec<ZkAppCommand>) -> Vec<Transaction> {
  let block_map = generate_operations_zkapp_command(commands);

  let mut result = Vec::new();
  for (_, tx_map) in block_map {
    for (tx_hash, operations) in tx_map {
      let transaction = Transaction {
        transaction_identifier: Box::new(TransactionIdentifier { hash: tx_hash }),
        operations,
        metadata: None,
        related_transactions: None,
      };
      result.push(transaction);
    }
  }

  result
}
