use mesh::models::{
  AccountIdentifier, Amount, BlockIdentifier, Currency, OperationIdentifier, Transaction, TransactionIdentifier,
};
pub use mesh::models::{BlockTransaction, Operation, SearchTransactionsRequest, SearchTransactionsResponse};
use serde_json::json;
use sqlx::{FromRow, Type};

pub use crate::{MinaMesh, MinaMeshError};

#[derive(Type, Debug)]
#[sqlx(type_name = "chain_status_type", rename_all = "lowercase")]
pub enum ChainStatus {
  Canonical,
  Pending,
  Orphaned,
}

// implement Display for ChainStatus
impl std::fmt::Display for ChainStatus {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ChainStatus::Canonical => write!(f, "canonical"),
      ChainStatus::Pending => write!(f, "pending"),
      ChainStatus::Orphaned => write!(f, "orphaned"),
    }
  }
}

#[derive(Debug, Type)]
#[sqlx(type_name = "user_command_type", rename_all = "lowercase")]
pub enum UserCommandType {
  Payment,
  Delegation,
}

// implement Display for UserCommandType
impl std::fmt::Display for UserCommandType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      UserCommandType::Payment => write!(f, "payment"),
      UserCommandType::Delegation => write!(f, "delegation"),
    }
  }
}

#[derive(Type, Debug)]
#[sqlx(type_name = "transaction_status", rename_all = "lowercase")]
pub enum TransactionStatus {
  Applied,
  Failed,
}

// implement Display for TransactionStatus
impl std::fmt::Display for TransactionStatus {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      TransactionStatus::Applied => write!(f, "applied"),
      TransactionStatus::Failed => write!(f, "failed"),
    }
  }
}

#[derive(Debug, FromRow)]
pub struct UserCommand {
  pub id: Option<i32>,
  pub command_type: UserCommandType,
  pub fee_payer_id: Option<i32>,
  pub source_id: Option<i32>,
  pub receiver_id: Option<i32>,
  pub nonce: i64,
  pub amount: Option<String>,
  pub fee: Option<String>,
  pub valid_until: Option<i64>,
  pub memo: Option<String>,
  pub hash: String,
  pub block_id: Option<i32>,
  pub sequence_no: Option<i32>,
  pub status: TransactionStatus,
  pub failure_reason: Option<String>,
  pub state_hash: Option<String>,
  pub chain_status: Option<ChainStatus>,
  pub height: Option<i64>,
  pub total_count: Option<i64>,
  pub fee_payer: String,
  pub source: String,
  pub receiver: String,
  pub creation_fee: Option<String>,
}

impl UserCommand {
  pub fn into_block_transaction(self) -> BlockTransaction {
    // Construct BlockIdentifier from UserCommand
    let block_identifier =
      BlockIdentifier { index: self.height.unwrap_or_default(), hash: self.state_hash.unwrap_or_default() };

    // Construct TransactionIdentifier from UserCommand hash
    let transaction_identifier = TransactionIdentifier { hash: self.hash.clone() };

    // Construct Operations based on UserCommand
    let operations = vec![Operation {
      operation_identifier: Box::new(OperationIdentifier { index: 0, network_index: None }),
      r#type: self.command_type.to_string(),
      status: Some(self.status.to_string()),
      account: Some(Box::new(AccountIdentifier { address: self.source.clone(), metadata: None, sub_account: None })),
      amount: Some(Box::new(Amount {
        value: self.amount.unwrap_or_else(|| "0".to_string()),
        metadata: None,
        currency: Box::new(Currency { symbol: "MINA".to_string(), decimals: 9, metadata: None }),
      })),
      coin_change: None,
      metadata: Some(json!({
          "fee_payer": self.fee_payer,
          "receiver": self.receiver,
          "fee": self.fee.unwrap_or_else(|| "0".to_string()),
          "creation_fee": self.creation_fee.unwrap_or_else(|| "0".to_string()),
      })),
      related_operations: None,
    }];

    // Construct Transaction
    let transaction = Transaction {
      transaction_identifier: Box::new(transaction_identifier),
      operations,
      related_transactions: None,
      metadata: None,
    };

    // Construct BlockTransaction
    BlockTransaction { block_identifier: Box::new(block_identifier), transaction: Box::new(transaction) }
  }
}

impl MinaMesh {
  pub async fn search_transactions(
    &self,
    req: SearchTransactionsRequest,
  ) -> Result<SearchTransactionsResponse, MinaMeshError> {
    let user_commands = self.fetch_user_commands(&req).await?;
    let user_commands_len = user_commands.len();
    let next_offset = req.offset.unwrap_or(0) + user_commands_len as i64;

    // Extract the total count from the first user command, or default to 0
    let user_commands_total_count = user_commands.first().and_then(|uc| uc.total_count).unwrap_or(0);

    // Map user commands into block transactions
    let user_commands_bt = user_commands.into_iter().map(|uc| uc.into_block_transaction()).collect();

    let response = SearchTransactionsResponse {
      transactions: user_commands_bt,
      total_count: user_commands_total_count,
      next_offset: match next_offset {
        offset if offset < user_commands_total_count => Some(offset),
        _ => None,
      },
    };

    Ok(response)
  }

  pub async fn fetch_user_commands(&self, req: &SearchTransactionsRequest) -> Result<Vec<UserCommand>, MinaMeshError> {
    let max_block = req.max_block;
    let txn_hash = req.transaction_identifier.as_ref().map(|t| &t.hash);
    let account_identifier = req.account_identifier.as_ref().map(|a| &a.address);
    let token_id = req.account_identifier.as_ref().and_then(|a| a.metadata.as_ref().map(|meta| meta.to_string()));
    let status = match req.status.as_deref() {
      Some("applied") => Some(TransactionStatus::Applied),
      Some("failed") => Some(TransactionStatus::Failed),
      Some(_other) => None,
      None => None,
    };
    let success_status = match req.success {
      Some(true) => Some(TransactionStatus::Applied),
      Some(false) => Some(TransactionStatus::Failed),
      None => None,
    };
    let address = req.address.as_ref();
    let limit = req.limit.unwrap_or(100);
    let offset = req.offset.unwrap_or(0);

    let user_commands = sqlx::query_file_as!(
      UserCommand,
      "sql/indexer_user_commands.sql",
      max_block,
      txn_hash,
      account_identifier,
      token_id,
      status as Option<TransactionStatus>,
      success_status as Option<TransactionStatus>,
      address,
      limit,
      offset,
    )
    .fetch_all(&self.pg_pool)
    .await;

    match user_commands {
      Ok(user_commands) => Ok(user_commands),
      Err(e) => {
        tracing::error!("Failed to fetch user commands: {:?}", e);
        Err(MinaMeshError::Sql(e.to_string()))
      }
    }
  }

  #[allow(dead_code)]
  async fn fetch_internal_commands(
    &self,
    _req: &SearchTransactionsRequest,
  ) -> Result<Vec<BlockTransaction>, MinaMeshError> {
    unimplemented!()
  }

  #[allow(dead_code)]
  async fn fetch_zkapp_commands(
    &self,
    _req: &SearchTransactionsRequest,
  ) -> Result<Vec<BlockTransaction>, MinaMeshError> {
    unimplemented!()
  }
}
