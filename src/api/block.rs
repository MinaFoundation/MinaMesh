use anyhow::Result;
use mesh::models::{
  AccountIdentifier, Block, BlockIdentifier, BlockRequest, BlockResponse, Operation, OperationIdentifier,
  PartialBlockIdentifier, Transaction, TransactionIdentifier,
};
use serde::Serialize;
use sqlx::FromRow;

use crate::{
  ChainStatus, InternalCommandType, MinaMesh, MinaMeshError, OperationStatus, OperationType, TransactionStatus,
  UserCommandType, Wrapper,
};

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/block.ml#L7
impl MinaMesh {
  pub async fn block(&self, request: BlockRequest) -> Result<BlockResponse, MinaMeshError> {
    let partial_block_identifier = *request.block_identifier;
    let metadata = match self.block_metadata(&partial_block_identifier).await? {
      Some(metadata) => metadata,
      None => return Err(MinaMeshError::BlockMissing(Wrapper(&partial_block_identifier).to_string())),
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
    let user_commands = self.user_commands(&metadata).await?;
    // let internal_commands = self.internal_commands(&metadata).await?;
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
    // let metadata =
    //   sqlx::query_file_as!(InternalCommandMetadata, "sql/internal_commands.sql",
    // metadata.id, DEFAULT_TOKEN_ID)     .fetch_all(&self.pg_pool)
    //     .await?;
    // let transactions = metadata
    //   .into_iter()
    //   .map(|item| {
    //     Transaction::new(TransactionIdentifier::new(item.hash.clone()),
    // internal_command_metadata_to_operation(&item))   })
    //   .collect();
    // Ok(transactions)
    unimplemented!();
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

fn user_command_metadata_to_operations(metadata: &UserCommandMetadata) -> Vec<Operation> {
  let mut operations = Vec::new();
  if metadata.fee != "0" {
    operations.push(Operation {
      operation_identifier: Box::new(OperationIdentifier::new(0)),
      amount: Wrapper(Some(metadata.fee.clone())).into(),
      account: Some(Box::new(AccountIdentifier::new(metadata.fee_payer.clone()))),
      status: Some(OperationStatus::Success.to_string()),
      related_operations: None,
      coin_change: None,
      r#type: Wrapper(OperationType::FeePayment).to_snake_case(),
      metadata: None, // TODO: get the correct metadata
    });
  }
  if metadata.failure_reason.is_none() {
    if let Some(creation_fee) = &metadata.creation_fee {
      operations.push(Operation {
        operation_identifier: Box::new(OperationIdentifier::new(1)),
        amount: Wrapper(Some(creation_fee.to_owned())).into(),
        account: Some(Box::new(AccountIdentifier::new(metadata.receiver.clone()))),
        status: Some(OperationStatus::from(metadata.status.clone()).to_string()),
        related_operations: None,
        coin_change: None,
        r#type: Wrapper(OperationType::AccountCreationFeeViaPayment).to_snake_case(),
        metadata: None, // TODO: get the correct metadata
      });
    }
    match metadata.command_type {
      UserCommandType::Delegation => {
        operations.push(Operation {
          operation_identifier: Box::new(OperationIdentifier::new(2)),
          amount: None,
          account: Some(Box::new(AccountIdentifier::new(metadata.source.clone()))),
          status: Some(OperationStatus::from(metadata.status.clone()).to_string()),
          related_operations: None,
          coin_change: None,
          r#type: Wrapper(OperationType::DelegateChange).to_snake_case(),
          metadata: None, // TODO: get the correct metadata
        });
      }
      UserCommandType::Payment => {
        operations.extend_from_slice(&[
          Operation {
            operation_identifier: Box::new(OperationIdentifier::new(2)),
            amount: Wrapper(metadata.amount.clone()).into(), // TODO: negate value
            account: Some(Box::new(AccountIdentifier::new(metadata.source.clone()))),
            status: Some(OperationStatus::from(metadata.status.clone()).to_string()),
            related_operations: None,
            coin_change: None,
            r#type: Wrapper(OperationType::PaymentSourceDec).to_snake_case(),
            metadata: None, // TODO: get the correct metadata
          },
          Operation {
            operation_identifier: Box::new(OperationIdentifier::new(3)),
            amount: Wrapper(metadata.amount.clone()).into(),
            account: Some(Box::new(AccountIdentifier::new(metadata.receiver.clone()))),
            status: Some(OperationStatus::from(metadata.status.clone()).to_string()),
            related_operations: None,
            coin_change: None,
            r#type: Wrapper(OperationType::PaymentReceiverInc).to_snake_case(),
            metadata: None, // TODO: get the correct metadata
          },
        ]);
      }
    };
  }
  operations
}

fn internal_command_metadata_to_operation(metadata: &InternalCommandMetadata) -> Result<Vec<Operation>, MinaMeshError> {
  let mut operations = Vec::new();
  if let Some(creation_fee) = &metadata.creation_fee {
    operations.push(Operation {
      operation_identifier: Box::new(OperationIdentifier::new(0)),
      amount: Wrapper(Some(creation_fee.clone())).into(),
      account: Some(Box::new(AccountIdentifier::new(metadata.receiver.clone()))),
      status: Some(OperationStatus::Success.to_string()),
      related_operations: None,
      coin_change: None,
      r#type: Wrapper(OperationType::AccountCreationFeeViaFeeReceiver).to_snake_case(),
      metadata: None, // TODO: get the correct metadata
    });
  }

  match metadata.command_type {
    InternalCommandType::Coinbase => {
      operations.push(Operation {
        operation_identifier: Box::new(OperationIdentifier::new(2)),
        amount: Wrapper(Some(metadata.fee.clone())).into(),
        account: Some(Box::new(AccountIdentifier::new(metadata.receiver.clone()))),
        status: Some(OperationStatus::Success.to_string()),
        related_operations: None,
        coin_change: None,
        r#type: Wrapper(OperationType::CoinbaseInc).to_snake_case(),
        metadata: None, // TODO: get the correct metadata
      });
    }
    InternalCommandType::FeeTransfer => {
      operations.push(Operation {
        operation_identifier: Box::new(OperationIdentifier::new(2)),
        amount: Wrapper(Some(metadata.fee.clone())).into(),
        account: Some(Box::new(AccountIdentifier::new(metadata.receiver.clone()))), // TODO: token id
        status: Some(OperationStatus::Success.to_string()),
        related_operations: None,
        coin_change: None,
        r#type: Wrapper(OperationType::FeeReceiverInc).to_snake_case(),
        metadata: None, // TODO: get the correct metadata
      });
    }
    InternalCommandType::FeeTransferViaCoinbase => {
      if let Some(coinbase_receiver) = &metadata.coinbase_receiver {
        operations.push(Operation {
          operation_identifier: Box::new(OperationIdentifier::new(2)),
          amount: Wrapper(Some(metadata.fee.clone())).into(),
          account: Some(Box::new(AccountIdentifier::new(metadata.receiver.clone()))), // TODO: token id
          status: Some(OperationStatus::Success.to_string()),
          related_operations: None,
          coin_change: None,
          r#type: Wrapper(OperationType::FeeReceiverInc).to_snake_case(),
          metadata: None, // TODO: get the correct metadata
        });
        operations.push(Operation {
          operation_identifier: Box::new(OperationIdentifier::new(3)),
          amount: Wrapper(Some(metadata.fee.clone())).into(), // TODO: negate value
          account: Some(Box::new(AccountIdentifier::new(coinbase_receiver.clone()))), // TODO: token id
          status: Some(OperationStatus::Success.to_string()),
          related_operations: None,
          coin_change: None,
          r#type: Wrapper(OperationType::FeePayerDec).to_snake_case(),
          metadata: None, // TODO: get the correct metadata
        });
      } else {
        return Err(MinaMeshError::InvariantViolation);
      }
    }
  }
  Ok(operations)
}

// cspell:disable-next-line
static DEFAULT_TOKEN_ID: &str = "wSHV2S4qX9jFsLjQo8r1BsMLH2ZRKsZx6EJd1sbozGPieEC4Jf";
