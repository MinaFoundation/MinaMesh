use coinbase_mesh::models::{
  AccountIdentifier, BlockIdentifier, BlockTransaction, SearchTransactionsRequest, SearchTransactionsResponse,
  Transaction, TransactionIdentifier,
};
use serde_json::json;
use sqlx::FromRow;

use crate::{
  operation, util::DEFAULT_TOKEN_ID, ChainStatus, MinaMesh, MinaMeshError, OperationType, TransactionStatus,
  UserCommandType,
};

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
  pub fn decoded_memo(&self) -> Option<String> {
    let memo = self.memo.clone().unwrap_or_default();
    match bs58::decode(memo).into_vec() {
      Ok(decoded_bytes) => {
        let cleaned = &decoded_bytes[3 .. decoded_bytes[2] as usize + 3];
        Some(String::from_utf8_lossy(cleaned).to_string())
      }
      Err(_) => None,
    }
  }

  pub fn into_block_transaction(self) -> BlockTransaction {
    let decoded_memo = self.decoded_memo().unwrap_or_default();
    let amt = self.amount.clone().unwrap_or_else(|| "0".to_string());
    let receiver_account_id = &AccountIdentifier {
      address: self.receiver.clone(),
      metadata: Some(json!({ "token_id": DEFAULT_TOKEN_ID })),
      sub_account: None,
    };
    let source_account_id = &AccountIdentifier {
      address: self.source,
      metadata: Some(json!({ "token_id": DEFAULT_TOKEN_ID })),
      sub_account: None,
    };
    let fee_payer_account_id = &AccountIdentifier {
      address: self.fee_payer,
      metadata: Some(json!({ "token_id": DEFAULT_TOKEN_ID })),
      sub_account: None,
    };

    let mut operations = Vec::new();
    let mut operation_index = 0;

    // Operation 1: Fee Payment
    operations.push(operation(
      operation_index,
      Some(&format!("-{}", self.fee.unwrap_or_else(|| "0".to_string()))),
      fee_payer_account_id,
      OperationType::FeePayment,
      Some(&self.status),
      None,
      None,
    ));

    operation_index += 1;

    // Operation 2: Account Creation Fee (if applicable)
    if let Some(creation_fee) = &self.creation_fee {
      operations.push(operation(
        operation_index,
        Some(&format!("-{}", creation_fee)),
        receiver_account_id,
        OperationType::AccountCreationFeeViaPayment,
        Some(&self.status),
        None,
        None,
      ));

      operation_index += 1;
    }

    // Decide on the type of operation based on command type
    match self.command_type {
      // Operation 3: Payment Source Decrement
      UserCommandType::Payment => {
        operations.push(operation(
          operation_index,
          Some(&format!("-{}", amt)),
          source_account_id,
          OperationType::PaymentSourceDec,
          Some(&self.status),
          None,
          None,
        ));

        operation_index += 1;

        // Operation 4: Payment Receiver Increment
        operations.push(operation(
          operation_index,
          Some(&amt),
          receiver_account_id,
          OperationType::PaymentReceiverInc,
          Some(&self.status),
          Some(vec![operation_index - 1]),
          None,
        ));
      }

      // Operation 3: Delegate Change
      UserCommandType::Delegation => {
        operations.push(operation(
          operation_index,
          None,
          source_account_id,
          OperationType::DelegateChange,
          Some(&self.status),
          None,
          Some(json!({ "delegate_change_target": self.receiver })),
        ));
      }
    }

    let block_identifier = BlockIdentifier::new(self.height.unwrap_or_default(), self.state_hash.unwrap_or_default());
    let transaction = Transaction {
      transaction_identifier: Box::new(TransactionIdentifier::new(self.hash)),
      operations,
      related_transactions: None,
      metadata: match decoded_memo.as_str() {
        "" => None,
        _ => Some(json!({ "memo": decoded_memo })),
      },
    };
    BlockTransaction::new(block_identifier, transaction)
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
