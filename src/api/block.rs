use anyhow::Result;
use coinbase_mesh::models::{
  AccountIdentifier, Block, BlockIdentifier, BlockRequest, BlockResponse, Operation, PartialBlockIdentifier,
  Transaction, TransactionIdentifier,
};
use serde::Serialize;
use serde_json::json;
use sqlx::FromRow;

use crate::{
  generate_operations_user_command, operation, sql_to_mesh::zkapp_commands_to_transactions, util::DEFAULT_TOKEN_ID,
  ChainStatus, InternalCommandMetadata, InternalCommandType, MinaMesh, MinaMeshError, OperationType, TransactionStatus,
  UserCommandMetadata, UserCommandType, ZkAppCommand,
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
    let (mut user_commands, internal_commands, zkapp_commands) = tokio::try_join!(
      self.user_commands(&metadata),
      self.internal_commands(&metadata),
      self.zkapp_commands(&metadata)
    )?;
    user_commands.extend(internal_commands.into_iter());
    user_commands.extend(zkapp_commands.into_iter());
    Ok(BlockResponse {
      block: Some(Box::new(Block {
        block_identifier: Box::new(block_identifier),
        parent_block_identifier: Box::new(parent_block_identifier),
        timestamp: metadata.timestamp.parse()?,
        transactions: user_commands,
        metadata: Some(json!({ "creator": metadata.creator })),
      })),
      other_transactions: None,
    })
  }

  // TODO: use default token value, check how to best handle this
  pub async fn user_commands(&self, metadata: &BlockMetadata) -> Result<Vec<Transaction>, MinaMeshError> {
    let metadata =
      sqlx::query_file_as!(UserCommandMetadata, "sql/queries/user_commands.sql", metadata.id, DEFAULT_TOKEN_ID)
        .fetch_all(&self.pg_pool)
        .await?;
    let transactions = metadata
      .into_iter()
      .map(|item| {
        Transaction::new(TransactionIdentifier::new(item.hash.clone()), generate_operations_user_command(&item))
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
        internal_command_metadata_to_operation(&item)
          .map(|operation| Transaction::new(TransactionIdentifier::new(item.hash.clone()), operation))
      })
      .collect::<Result<Vec<Transaction>, MinaMeshError>>()?;
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

#[derive(Debug, PartialEq, Eq, FromRow, Serialize)]
pub struct ZkappCommandMetadata {
  id: i64,
  memo: Option<String>,
  hash: String,
  fee_payer: String,
  fee: String,
  valid_until: Option<i64>,
  nonce: i64,
  sequence_no: i64,
  status: TransactionStatus,
  failure_reasons: Option<Vec<String>>,
  balance_change: String,
  account: String,
  token: String,
}

#[derive(Debug, PartialEq, Eq, FromRow, Serialize)]
pub struct ZkappAccountUpdateMetadata {
  account_identifier_id: i32,
  update_id: i32,
  balance_change: String,
  increment_nonce: bool,
  events_id: i32,
  actions_id: i32,
  call_data_id: i32,
  call_depth: i32,
  zkapp_network_precondition_id: i32,
  zkapp_account_precondition_id: i32,
  zkapp_valid_while_precondition_id: Option<i32>,
  use_full_commitment: bool,
  implicit_account_creation_fee: bool,
  may_use_token: String,
  authorization_kind: String,
  verification_key_hash_id: Option<i32>,
  account: String,
  token: String,
}

fn internal_command_metadata_to_operation(metadata: &InternalCommandMetadata) -> Result<Vec<Operation>, MinaMeshError> {
  let receiver_account_id = &AccountIdentifier::new(metadata.receiver.clone());
  let mut operations = Vec::new();
  if let Some(creation_fee) = &metadata.creation_fee {
    operations.push(operation(
      0,
      Some(creation_fee),
      receiver_account_id,
      OperationType::AccountCreationFeeViaFeeReceiver,
      None,
      None,
      None,
      None,
    ));
  }
  match metadata.command_type {
    InternalCommandType::Coinbase => {
      operations.push(operation(
        2,
        Some(&metadata.fee),
        receiver_account_id,
        OperationType::CoinbaseInc,
        None,
        None,
        None,
        None,
      ));
    }
    InternalCommandType::FeeTransfer => {
      operations.push(operation(
        2,
        Some(&metadata.fee),
        receiver_account_id,
        OperationType::FeeReceiverInc,
        None,
        None,
        None,
        None,
      ));
    }
    InternalCommandType::FeeTransferViaCoinbase => {
      if let Some(coinbase_receiver) = &metadata.coinbase_receiver {
        operations.push(operation(
          2,
          Some(&metadata.fee),
          receiver_account_id,
          OperationType::FeeReceiverInc,
          None,
          None,
          None,
          None,
        ));
        operations.push(operation(
          3,
          Some(&metadata.fee),
          &AccountIdentifier::new(coinbase_receiver.to_string()),
          OperationType::FeePayerDec,
          None,
          None,
          None,
          None,
        ));
      } else {
        return Err(MinaMeshError::InvariantViolation);
      }
    }
  }
  Ok(operations)
}
