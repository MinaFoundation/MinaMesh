pub use mesh::models::{BlockTransaction, Operation, SearchTransactionsRequest, SearchTransactionsResponse};
use sqlx::{FromRow, Type};

pub use crate::{MinaMesh, MinaMeshError};

#[derive(Type, Debug)]
#[sqlx(type_name = "chain_status_type", rename_all = "lowercase")]
pub enum ChainStatus {
  Canonical,
  Pending,
  Orphaned,
}

#[derive(Debug, Type)]
#[sqlx(type_name = "user_command_type", rename_all = "lowercase")]
pub enum UserCommandType {
  Payment,
  Delegation,
}

#[derive(Type, Debug)]
#[sqlx(type_name = "transaction_status", rename_all = "lowercase")]
pub enum TransactionStatus {
  Applied,
  Failed,
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

impl MinaMesh {
  pub async fn search_transactions(
    &self,
    _req: SearchTransactionsRequest,
  ) -> Result<SearchTransactionsResponse, MinaMeshError> {
    unimplemented!()
  }

  pub async fn fetch_user_commands(&self, req: &SearchTransactionsRequest) -> Result<Vec<UserCommand>, MinaMeshError> {
    // Extract parameters from `req`
    let max_block = req.max_block;
    let txn_hash = req.transaction_identifier.as_ref().map(|t| &t.hash);
    let account_identifier = req.account_identifier.as_ref().map(|a| &a.address);
    let token_id = req.account_identifier.as_ref().and_then(|a| a.metadata.as_ref().map(|meta| meta.to_string()));
    let status = match req.status.as_deref() {
      Some("applied") => Some(TransactionStatus::Applied),
      Some("failed") => Some(TransactionStatus::Failed),
      Some(other) => return Err(MinaMeshError::Sql(format!("Invalid status: {}", other))),
      None => None,
    };
    let success = req.success.unwrap_or(true);
    let success_status = if success { TransactionStatus::Applied } else { TransactionStatus::Failed };
    let address = req.address.as_ref().map(|a| a.as_str());
    let limit = req.limit.unwrap_or(5);
    let offset = req.offset.unwrap_or(0);

    let user_commands = sqlx::query_file_as!(
      UserCommand,
      "sql/indexer_user_commands.sql",
      max_block,
      txn_hash,
      account_identifier,
      token_id.unwrap_or_else(|| "".to_string()),
      status as Option<TransactionStatus>,
      Some(success_status) as Option<TransactionStatus>,
      address,
      limit,
      offset,
    )
    .fetch_all(&self.pg_pool)
    .await?;

    Ok(user_commands)
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
