use anyhow::Result;
use mesh::models::{
  AccountIdentifier, Block, BlockIdentifier, Currency, Operation, OperationIdentifier, Transaction,
  TransactionIdentifier,
};
pub use mesh::models::{Amount, BlockRequest, BlockResponse, PartialBlockIdentifier};
use serde::Serialize;
use sqlx::FromRow;

use crate::{ChainStatus, CommandType, MinaMesh, MinaMeshError, TransactionStatus, Wrapper};

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
  id: i32,
  command_type: CommandType,
  fee_payer_id: i32,
  source_id: i32,
  receiver_id: i32,
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

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/block.ml#L7
impl MinaMesh {
  // pub async fn block(&self, request: BlockRequest) -> Result<BlockResponse,
  // MinaMeshError> {
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
    Ok(BlockResponse {
      block: Some(Box::new(Block::new(
        block_identifier,
        parent_block_identifier,
        metadata.timestamp.parse()?,
        user_commands,
      ))),
      other_transactions: None,
    })

    // // TODO: what else here?:
    // //   - fetch user commands from the database
    // //   - SQL command -> Rosetta/mesh transaction
    // //   - Each command will originate multiple atomic Rosetta/mesh
    // operations

    // let transactions = user_commands
    //   .into_iter()
    //   .map(|user_commands|
    // Transaction::new(TransactionIdentifier::new(user_commands.hash.0),
    // vec![]))   .collect();

    // Ok(BlockResponse {
    //   block: Some(Box::new(Block::new(
    //     BlockIdentifier::new(metadata.height, metadata.state_hash),
    //     // TODO: parent block height
    //     BlockIdentifier::new(0, metadata.parent_hash),
    //     metadata.timestamp.parse()?,
    //     transactions,
    //   ))),
    //   other_transactions: Some(vec![]),
    // })
  }

  fn user_command_metadata_to_operations(metadata: &UserCommandMetadata) -> Vec<Operation> {
    let mut operations = Vec::new();
    if metadata.fee != "0" {
      operations.push(Operation {
        operation_identifier: Box::new(OperationIdentifier::new(0)),
        amount: Wrapper(Some(metadata.fee.clone())).into(),
        account: Some(Box::new(AccountIdentifier::new(metadata.fee_payer.clone()))),
        status: Some(metadata.status.to_string()),
        related_operations: None,
        coin_change: None,
        r#type: "".to_string(), // TODO: get the correct type
        metadata: None,         // TODO: get the correct metadata
      });
    }
    match &metadata.failure_reason {
      Some(_failure_reason) => {}
      None => {
        if let Some(creation_fee) = &metadata.creation_fee {
          operations.push(Operation {
            operation_identifier: Box::new(OperationIdentifier::new(1)),
            amount: Wrapper(Some(creation_fee.to_owned())).into(),
            account: Some(Box::new(AccountIdentifier::new(metadata.receiver.clone()))),
            status: Some(metadata.status.to_string()),
            related_operations: None,
            coin_change: None,
            r#type: "".to_string(), // TODO: get the correct type
            metadata: None,         // TODO: get the correct metadata
          });
        }
        match metadata.command_type {
          CommandType::Delegation => {
            operations.push(Operation {
              operation_identifier: Box::new(OperationIdentifier::new(2)),
              amount: None,
              account: Some(Box::new(AccountIdentifier::new(metadata.source.clone()))),
              status: Some(metadata.status.to_string()),
              related_operations: None,
              coin_change: None,
              r#type: "".to_string(), // TODO: get the correct type
              metadata: None,         // TODO: get the correct metadata
            });
          }
          CommandType::Payment => {
            operations.push(Operation {
              operation_identifier: Box::new(OperationIdentifier::new(2)),
              amount: Wrapper(metadata.amount.clone()).into(),
              account: Some(Box::new(AccountIdentifier::new(metadata.source.clone()))),
              status: Some(metadata.status.to_string()),
              related_operations: None,
              coin_change: None,
              r#type: "".to_string(), // TODO: get the correct type
              metadata: None,         // TODO: get the correct metadata
            });
            operations.push(Operation {
              operation_identifier: Box::new(OperationIdentifier::new(3)),
              amount: Wrapper(metadata.amount.clone()).into(),
              account: Some(Box::new(AccountIdentifier::new(metadata.receiver.clone()))),
              status: Some(metadata.status.to_string()),
              related_operations: None,
              coin_change: None,
              r#type: "".to_string(), // TODO: get the correct type
              metadata: None,         // TODO: get the correct metadata
            });
          }
        };
      }
    }
    operations
  }

  // Do we also need internal and zkapps commands?
  pub async fn user_commands(&self, metadata: &BlockMetadata) -> Result<Vec<Transaction>, MinaMeshError> {
    Ok(
      sqlx::query_file_as!(
        UserCommandMetadata,
        "sql/user_commands.sql",
        metadata.id,
        "wSHV2S4qX9jFsLjQo8r1BsMLH2ZRKsZx6EJd1sbozGPieEC4Jf" /* TODO: use default token value, check how to best
                                                              * handle this */
      )
      .fetch_all(&self.pg_pool)
      .await?
      .into_iter()
      .map(|item| {
        Transaction::new(
          TransactionIdentifier::new(item.hash.clone()),
          Self::user_command_metadata_to_operations(&item),
        )
      })
      .collect(),
    )
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
