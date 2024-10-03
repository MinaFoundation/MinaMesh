use bs58;
use mesh::models::{
  AccountIdentifier, Amount, BlockIdentifier, Currency, OperationIdentifier, Transaction, TransactionIdentifier,
};
pub use mesh::models::{BlockTransaction, Operation, SearchTransactionsRequest, SearchTransactionsResponse};
use serde_json::json;
use sqlx::{FromRow, Type};

pub use crate::{util::Wrapper, MinaMesh, MinaMeshError};

#[derive(Type, Debug)]
#[sqlx(type_name = "chain_status_type", rename_all = "lowercase")]
pub enum ChainStatus {
  Canonical,
  Pending,
  Orphaned,
}

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

impl std::fmt::Display for TransactionStatus {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      TransactionStatus::Applied => write!(f, "applied"),
      TransactionStatus::Failed => write!(f, "failed"),
    }
  }
}

impl TransactionStatus {
  pub fn to_status(&self) -> String {
    match self {
      TransactionStatus::Applied => "Success".to_string(),
      TransactionStatus::Failed => "Failed".to_string(),
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

#[derive(Debug)]
pub enum OperationType {
  FeePayment,
  PaymentSourceDecrement,
  PaymentReceiverIncrement,
  DelegateChange,
  FeePayerDecrement,
  AccountCreationFeeViaPayment,
  AccountCreationFeeViaFeeReceiver,
  ZkappFeePayerDecrement,
  ZkappBalanceUpdate,
  FeeReceiverIncrement,
  CoinbaseIncrement,
}

impl std::fmt::Display for OperationType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      OperationType::FeePayment => write!(f, "fee_payment"),
      OperationType::AccountCreationFeeViaPayment => write!(f, "account_creation_fee_via_payment"),
      OperationType::PaymentSourceDecrement => write!(f, "payment_source_dec"),
      OperationType::PaymentReceiverIncrement => write!(f, "payment_receiver_inc"),
      OperationType::DelegateChange => write!(f, "delegate_change"),
      OperationType::FeePayerDecrement => write!(f, "fee_payer_dec"),
      OperationType::AccountCreationFeeViaFeeReceiver => write!(f, "account_creation_fee_via_fee_receiver"),
      OperationType::ZkappFeePayerDecrement => write!(f, "zkapp_fee_payer_dec"),
      OperationType::ZkappBalanceUpdate => write!(f, "zkapp_balance_update"),
      OperationType::FeeReceiverIncrement => write!(f, "fee_receiver_inc"),
      OperationType::CoinbaseIncrement => write!(f, "coinbase_inc"),
    }
  }
}

impl UserCommand {
  pub fn decoded_memo(&self) -> Option<String> {
    let memo = self.memo.clone().unwrap_or_else(|| "".to_string());
    match bs58::decode(memo).into_vec() {
      Ok(decoded_bytes) => {
        let cleaned = &decoded_bytes[3 .. decoded_bytes[2] as usize + 3];
        Some(String::from_utf8_lossy(&cleaned).to_string())
      }
      Err(_) => None,
    }
  }

  pub fn into_block_transaction(self) -> BlockTransaction {
    let default_token_id = Wrapper(None).to_token_id().unwrap();
    let decoded_memo = self.decoded_memo().unwrap_or_else(|| "".to_string());
    let amt = self.amount.clone().unwrap_or_else(|| "0".to_string());

    // Construct BlockIdentifier from UserCommand
    let block_identifier =
      BlockIdentifier { index: self.height.unwrap_or_default(), hash: self.state_hash.unwrap_or_default() };

    // Construct TransactionIdentifier from UserCommand hash
    let transaction_identifier = TransactionIdentifier { hash: self.hash.clone() };

    // Create a series of operations for the transaction
    let mut operations = Vec::new();

    // Index for operations
    let mut operation_index = 0;

    // Operation 1: Fee Payment
    operations.push(Operation {
      operation_identifier: Box::new(OperationIdentifier { index: operation_index, network_index: None }),
      r#type: OperationType::FeePayment.to_string(),
      status: Some(self.status.to_status()),
      account: Some(Box::new(AccountIdentifier {
        address: self.fee_payer.clone(),
        metadata: Some(json!({ "token_id": default_token_id })),
        sub_account: None,
      })),
      amount: Some(Box::new(Amount {
        value: format!("-{}", self.fee.unwrap_or_else(|| "0".to_string())), // Negative value for fee
        metadata: None,
        currency: Box::new(Currency { symbol: "MINA".to_string(), decimals: 9, metadata: None }),
      })),
      coin_change: None,
      metadata: None,
      related_operations: None,
    });

    operation_index += 1;

    // Operation 2: Account Creation Fee (if applicable)
    if let Some(creation_fee) = &self.creation_fee {
      if let Ok(fee_value) = creation_fee.parse::<i64>() {
        if fee_value > 0 {
          operations.push(Operation {
            operation_identifier: Box::new(OperationIdentifier { index: operation_index, network_index: None }),
            r#type: OperationType::AccountCreationFeeViaPayment.to_string(),
            status: Some(self.status.to_status()),
            account: Some(Box::new(AccountIdentifier {
              address: self.receiver.clone(),
              metadata: Some(json!({ "token_id": default_token_id })),
              sub_account: None,
            })),
            amount: Some(Box::new(Amount {
              value: format!("-{}", creation_fee),
              metadata: None,
              currency: Box::new(Currency { symbol: "MINA".to_string(), decimals: 9, metadata: None }),
            })),
            coin_change: None,
            metadata: None,
            related_operations: None,
          });

          operation_index += 1;
        }
      }
    }

    // Decide on the type of operation based on command type
    match self.command_type {
      UserCommandType::Payment => {
        // Operation 3: Payment Source Decrement
        operations.push(Operation {
          operation_identifier: Box::new(OperationIdentifier { index: operation_index, network_index: None }),
          r#type: OperationType::PaymentSourceDecrement.to_string(),
          status: Some(self.status.to_status()),
          account: Some(Box::new(AccountIdentifier {
            address: self.source.clone(),
            metadata: Some(json!({ "token_id": default_token_id })),
            sub_account: None,
          })),
          amount: Some(Box::new(Amount {
            value: format!("-{}", amt), // Negative value for the payment amount
            metadata: None,
            currency: Box::new(Currency { symbol: "MINA".to_string(), decimals: 9, metadata: None }),
          })),
          coin_change: None,
          metadata: None,
          related_operations: None,
        });

        operation_index += 1;

        // Operation 4: Payment Receiver Increment
        operations.push(Operation {
                operation_identifier: Box::new(OperationIdentifier { index: operation_index, network_index: None }),
                r#type: OperationType::PaymentReceiverIncrement.to_string(),
                status: Some(self.status.to_status()),
                account: Some(Box::new(AccountIdentifier {
                    address: self.receiver.clone(),
                    metadata: Some(json!({ "token_id": default_token_id })),
                    sub_account: None,
                })),
                amount: Some(Box::new(Amount {
                    value: amt, // Positive value for the payment amount
                    metadata: None,
                    currency: Box::new(Currency { symbol: "MINA".to_string(), decimals: 9, metadata: None }),
                })),
                coin_change: None,
                metadata: None,
                related_operations: Some(vec![OperationIdentifier { index: operation_index - 1, network_index: None }]), // Relate to the previous source decrement
            });
      }
      UserCommandType::Delegation => {
        // Operation 3: Delegate Change
        operations.push(Operation {
          operation_identifier: Box::new(OperationIdentifier { index: operation_index, network_index: None }),
          r#type: OperationType::DelegateChange.to_string(),
          status: Some(self.status.to_status()),
          account: Some(Box::new(AccountIdentifier {
            address: self.source.clone(),
            metadata: Some(json!({ "token_id": default_token_id })),
            sub_account: None,
          })),
          amount: None,
          coin_change: None,
          metadata: Some(json!({ "delegate_change_target": self.receiver.clone() })),
          related_operations: None,
        });
      }
    }

    // Construct Transaction
    let transaction = Transaction {
      transaction_identifier: Box::new(transaction_identifier),
      operations,
      related_transactions: None,
      metadata: match decoded_memo.as_str() {
        "" => None,
        _ => Some(json!({ "memo": decoded_memo })),
      },
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
