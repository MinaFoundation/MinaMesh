use anyhow::Result;
use coinbase_mesh::models::{
  AccountIdentifier, Amount, Block, BlockIdentifier, BlockRequest, BlockResponse, Currency, Operation,
  OperationIdentifier, PartialBlockIdentifier, Transaction, TransactionIdentifier,
};
use convert_case::{Case, Casing};
use serde::Serialize;
use sqlx::FromRow;

use crate::{
  operation, util::DEFAULT_TOKEN_ID, ChainStatus, InternalCommandType, MinaMesh, MinaMeshError, OperationType,
  TransactionStatus, UserCommandType,
};

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/block.ml#L7
impl MinaMesh {
  pub async fn block(&self, request: BlockRequest) -> Result<BlockResponse, MinaMeshError> {
    let partial_block_identifier = *request.block_identifier;
    let metadata = match self.block_metadata(&partial_block_identifier).await? {
      Some(metadata) => metadata,
      None => return Err(MinaMeshError::BlockMissing(serde_json::to_string(&partial_block_identifier)?)),
    };
    let parent_block_metadata = match &metadata.parent_id {
      Some(parent_id) => {
        sqlx::query_file_as!(BlockMetadata, "sql/query_id.sql", parent_id).fetch_optional(&self.pg_pool).await?
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
      block: Some(Box::new(Block::new(
        block_identifier,
        parent_block_identifier,
        metadata.timestamp.parse()?,
        user_commands,
      ))),
      other_transactions: None,
    })
  }

  // TODO: use default token value, check how to best handle this
  pub async fn user_commands(&self, metadata: &BlockMetadata) -> Result<Vec<Transaction>, MinaMeshError> {
    let metadata = sqlx::query_file_as!(UserCommandMetadata, "sql/user_commands.sql", metadata.id, DEFAULT_TOKEN_ID)
      .fetch_all(&self.pg_pool)
      .await?;
    let transactions = metadata
      .into_iter()
      .map(|item| {
        Transaction::new(TransactionIdentifier::new(item.hash.clone()), user_command_metadata_to_operations(&item))
      })
      .collect();
    Ok(transactions)
  }

  pub async fn internal_commands(&self, metadata: &BlockMetadata) -> Result<Vec<Transaction>, MinaMeshError> {
    let metadata =
      sqlx::query_file_as!(InternalCommandMetadata, "sql/internal_commands.sql", metadata.id, DEFAULT_TOKEN_ID)
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
    let metadata = sqlx::query_file_as!(ZkappCommandMetadata, "sql/zkapp_commands.sql", metadata.id, DEFAULT_TOKEN_ID)
      .fetch_all(&self.pg_pool)
      .await?;
    let transactions = metadata
      .into_iter()
      .map(|item| {
        zkapp_command_metadata_to_operation(&item)
          .map(|operation| Transaction::new(TransactionIdentifier::new(item.hash.clone()), operation))
      })
      .collect::<Result<Vec<Transaction>, MinaMeshError>>()?;
    Ok(transactions)
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
pub struct UserCommandMetadata {
  command_type: UserCommandType,
  nonce: i64,
  amount: Option<String>,
  fee: String,
  valid_until: Option<i64>,
  memo: String,
  hash: String,
  fee_payer: String,
  source: String,
  receiver: String,
  status: TransactionStatus,
  failure_reason: Option<String>,
  creation_fee: Option<String>,
}

#[derive(Debug, PartialEq, Eq, FromRow, Serialize)]
pub struct InternalCommandMetadata {
  command_type: InternalCommandType,
  receiver: String,
  fee: String,
  hash: String,
  creation_fee: Option<String>,
  sequence_no: i32,
  secondary_sequence_no: i32,
  coinbase_receiver: Option<String>,
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

fn user_command_metadata_to_operations(metadata: &UserCommandMetadata) -> Vec<Operation> {
  let mut operations = Vec::new();
  if metadata.fee != "0" {
    operations.push(operation(0, Some(&metadata.fee), &metadata.fee_payer, OperationType::FeePayment, None));
  }
  if metadata.failure_reason.is_none() {
    if let Some(creation_fee) = &metadata.creation_fee {
      operations.push(operation(
        1,
        Some(creation_fee),
        &metadata.receiver,
        OperationType::AccountCreationFeeViaPayment,
        Some(&metadata.status),
      ));
    }
    match metadata.command_type {
      UserCommandType::Delegation => {
        operations.push(operation(2, None, &metadata.source, OperationType::DelegateChange, Some(&metadata.status)));
      }
      UserCommandType::Payment => {
        operations.extend_from_slice(&[
          operation(
            2,
            metadata.amount.as_ref(),
            &metadata.source,
            OperationType::PaymentSourceDec,
            Some(&metadata.status),
          ),
          operation(
            3,
            metadata.amount.as_ref(),
            &metadata.receiver,
            OperationType::PaymentReceiverInc,
            Some(&metadata.status),
          ),
        ]);
      }
    };
  }
  operations
}

fn internal_command_metadata_to_operation(metadata: &InternalCommandMetadata) -> Result<Vec<Operation>, MinaMeshError> {
  let mut operations = Vec::new();
  if let Some(creation_fee) = &metadata.creation_fee {
    operations.push(operation(
      0,
      Some(creation_fee),
      &metadata.receiver,
      OperationType::AccountCreationFeeViaFeeReceiver,
      None,
    ));
  }
  match metadata.command_type {
    InternalCommandType::Coinbase => {
      operations.push(operation(2, Some(&metadata.fee), &metadata.receiver, OperationType::CoinbaseInc, None));
    }
    InternalCommandType::FeeTransfer => {
      operations.push(operation(2, Some(&metadata.fee), &metadata.receiver, OperationType::FeeReceiverInc, None));
    }
    InternalCommandType::FeeTransferViaCoinbase => {
      if let Some(coinbase_receiver) = &metadata.coinbase_receiver {
        operations.push(operation(2, Some(&metadata.fee), &metadata.receiver, OperationType::FeeReceiverInc, None));
        operations.push(operation(3, Some(&metadata.fee), coinbase_receiver, OperationType::FeePayerDec, None));
      } else {
        return Err(MinaMeshError::InvariantViolation);
      }
    }
  }
  Ok(operations)
}

// TODO: implement
fn zkapp_command_metadata_to_operation(_metadata: &ZkappCommandMetadata) -> Result<Vec<Operation>, MinaMeshError> {
  Ok(Vec::new())
}
