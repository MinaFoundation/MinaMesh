use coinbase_mesh::models::{
  AccountIdentifier, BlockIdentifier, BlockTransaction, SearchTransactionsRequest, SearchTransactionsResponse,
  Transaction, TransactionIdentifier,
};
use convert_case::{Case, Casing};
use serde_json::json;
use sqlx::FromRow;

use crate::{
  operation, util::DEFAULT_TOKEN_ID, ChainStatus, InternalCommandType, MinaMesh, MinaMeshError, OperationType,
  TransactionStatus, UserCommandType,
};

impl MinaMesh {
  pub async fn search_transactions(
    &self,
    req: SearchTransactionsRequest,
  ) -> Result<SearchTransactionsResponse, MinaMeshError> {
    let original_offset = req.offset.unwrap_or(0);
    let mut offset = original_offset;
    let mut limit = req.limit.unwrap_or(100);
    let mut transactions = Vec::new() as Vec<BlockTransaction>;
    let mut txs_len = 0;
    let mut total_count = 0;

    // User Commands
    let user_commands = self.fetch_user_commands(&req, offset, limit).await?;
    let user_commands_len = user_commands.len() as i64;
    let user_commands_total_count = user_commands.first().and_then(|uc| uc.total_count).unwrap_or(0);
    transactions.extend(user_commands.into_iter().map(|ic| ic.into()));
    total_count += user_commands_total_count;
    txs_len += user_commands_len;

    // Internal Commands
    if limit > total_count {
      // if we are below the limit, fetch internal commands
      (offset, limit) = adjust_limit_and_offset(limit, offset, txs_len);
      let internal_commands = self.fetch_internal_commands(&req, offset, limit).await?;
      let internal_commands_len = internal_commands.len() as i64;
      let internal_commands_total_count = internal_commands.first().and_then(|ic| ic.total_count).unwrap_or(0);
      transactions.extend(internal_commands.into_iter().map(|uc| uc.into()));
      txs_len += internal_commands_len;
      total_count += internal_commands_total_count;
    } else {
      // otherwise only fetch the first internal command to get the total count
      let internal_commands = self.fetch_internal_commands(&req, 0, 1).await?;
      let internal_commands_total_count = internal_commands.first().and_then(|ic| ic.total_count).unwrap_or(0);
      total_count += internal_commands_total_count;
    }

    let next_offset = original_offset + txs_len;
    let response = SearchTransactionsResponse {
      transactions,
      total_count,
      next_offset: match next_offset {
        offset if offset < total_count => Some(offset),
        _ => None,
      },
    };

    Ok(response)
  }

  pub async fn fetch_user_commands(
    &self,
    req: &SearchTransactionsRequest,
    offset: i64,
    limit: i64,
  ) -> Result<Vec<UserCommand>, MinaMeshError> {
    let query_params = SearchTransactionsQueryParams::try_from(req.clone())?;

    let user_commands = sqlx::query_file_as!(
      UserCommand,
      "sql/indexer_user_commands.sql",
      query_params.max_block,
      query_params.transaction_hash,
      query_params.account_identifier,
      query_params.token_id,
      query_params.status as Option<TransactionStatus>,
      query_params.success_status as Option<TransactionStatus>,
      query_params.address,
      limit,
      offset,
    )
    .fetch_all(&self.pg_pool)
    .await?;

    Ok(user_commands)
  }

  pub async fn fetch_internal_commands(
    &self,
    req: &SearchTransactionsRequest,
    offset: i64,
    limit: i64,
  ) -> Result<Vec<InternalCommand>, MinaMeshError> {
    let query_params = SearchTransactionsQueryParams::try_from(req.clone())?;

    let internal_commands = sqlx::query_file_as!(
      InternalCommand,
      "sql/indexer_internal_commands.sql",
      query_params.max_block,
      query_params.transaction_hash,
      query_params.account_identifier,
      query_params.token_id,
      query_params.status as Option<TransactionStatus>,
      query_params.success_status as Option<TransactionStatus>,
      query_params.address,
      limit,
      offset
    )
    .fetch_all(&self.pg_pool)
    .await?;

    Ok(internal_commands)
  }

  #[allow(dead_code)]
  async fn fetch_zkapp_commands(
    &self,
    _req: &SearchTransactionsRequest,
  ) -> Result<Vec<BlockTransaction>, MinaMeshError> {
    unimplemented!()
  }
}

#[derive(Debug, FromRow)]
pub struct InternalCommand {
  pub id: Option<i32>,
  pub command_type: InternalCommandType,
  pub receiver_id: Option<i32>,
  pub fee: Option<String>,
  pub hash: String,
  pub receiver: String,
  pub coinbase_receiver: Option<String>,
  pub sequence_no: i32,
  pub secondary_sequence_no: i32,
  pub block_id: i32,
  pub status: TransactionStatus,
  pub state_hash: Option<String>,
  pub height: Option<i64>,
  pub total_count: Option<i64>,
  pub creation_fee: Option<String>,
}

impl From<InternalCommand> for BlockTransaction {
  fn from(internal_command: InternalCommand) -> Self {
    // Derive transaction_identifier by combining command_type, sequence numbers,
    // and the hash
    let transaction_identifier = format!(
      "{}:{}:{}:{}",
      internal_command.command_type.to_string().to_case(Case::Snake),
      internal_command.sequence_no,
      internal_command.secondary_sequence_no,
      internal_command.hash
    );
    let fee = internal_command.fee.unwrap_or_else(|| "0".to_string());
    let status = &internal_command.status;

    let mut operations = Vec::new();
    let mut operation_index = 0;

    // Receiver Account Identifier
    let receiver_account_id = &AccountIdentifier {
      address: internal_command.receiver.clone(),
      metadata: Some(json!({ "token_id": DEFAULT_TOKEN_ID })),
      sub_account: None,
    };

    // Handle Account Creation Fee if applicable
    if let Some(creation_fee) = &internal_command.creation_fee {
      operations.push(operation(
        operation_index,
        Some(creation_fee),
        receiver_account_id,
        OperationType::AccountCreationFeeViaFeeReceiver,
        Some(status),
        None,
        None,
      ));
      operation_index += 1;
    }

    match internal_command.command_type {
      InternalCommandType::Coinbase => {
        operations.push(operation(
          operation_index,
          Some(&fee),
          receiver_account_id,
          OperationType::CoinbaseInc,
          Some(status),
          None,
          None,
        ));
      }

      InternalCommandType::FeeTransfer => {
        operations.push(operation(
          operation_index,
          Some(&fee),
          receiver_account_id,
          OperationType::FeeReceiverInc,
          Some(status),
          None,
          None,
        ));
      }

      InternalCommandType::FeeTransferViaCoinbase => {
        if let Some(coinbase_receiver) = &internal_command.coinbase_receiver {
          operations.push(operation(
            operation_index,
            Some(&fee),
            receiver_account_id,
            OperationType::FeeReceiverInc,
            Some(status),
            None,
            None,
          ));
          operation_index += 1;

          operations.push(operation(
            operation_index,
            Some(&fee),
            &AccountIdentifier::new(coinbase_receiver.to_string()),
            OperationType::FeePayerDec,
            Some(status),
            Some(vec![operation_index - 1]),
            None,
          ));
        }
      }
    }

    let block_identifier = BlockIdentifier::new(
      internal_command.height.unwrap_or_default(),
      internal_command.state_hash.unwrap_or_default(),
    );
    let transaction = Transaction {
      transaction_identifier: Box::new(TransactionIdentifier::new(transaction_identifier)),
      operations,
      related_transactions: None,
      metadata: None,
    };

    BlockTransaction::new(block_identifier, transaction)
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
}

impl From<UserCommand> for BlockTransaction {
  fn from(user_command: UserCommand) -> Self {
    let decoded_memo = user_command.decoded_memo().unwrap_or_default();
    let amt = user_command.amount.clone().unwrap_or_else(|| "0".to_string());
    let receiver_account_id = &AccountIdentifier {
      address: user_command.receiver.clone(),
      metadata: Some(json!({ "token_id": DEFAULT_TOKEN_ID })),
      sub_account: None,
    };
    let source_account_id = &AccountIdentifier {
      address: user_command.source,
      metadata: Some(json!({ "token_id": DEFAULT_TOKEN_ID })),
      sub_account: None,
    };
    let fee_payer_account_id = &AccountIdentifier {
      address: user_command.fee_payer,
      metadata: Some(json!({ "token_id": DEFAULT_TOKEN_ID })),
      sub_account: None,
    };

    let mut operations = Vec::new();
    let mut operation_index = 0;

    // Operation 1: Fee Payment
    operations.push(operation(
      operation_index,
      Some(&format!("-{}", user_command.fee.unwrap_or_else(|| "0".to_string()))),
      fee_payer_account_id,
      OperationType::FeePayment,
      Some(&user_command.status),
      None,
      None,
    ));

    operation_index += 1;

    // Operation 2: Account Creation Fee (if applicable)
    if let Some(creation_fee) = &user_command.creation_fee {
      operations.push(operation(
        operation_index,
        Some(&format!("-{}", creation_fee)),
        receiver_account_id,
        OperationType::AccountCreationFeeViaPayment,
        Some(&user_command.status),
        None,
        None,
      ));

      operation_index += 1;
    }

    // Decide on the type of operation based on command type
    match user_command.command_type {
      // Operation 3: Payment Source Decrement
      UserCommandType::Payment => {
        operations.push(operation(
          operation_index,
          Some(&format!("-{}", amt)),
          source_account_id,
          OperationType::PaymentSourceDec,
          Some(&user_command.status),
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
          Some(&user_command.status),
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
          Some(&user_command.status),
          None,
          Some(json!({ "delegate_change_target": user_command.receiver })),
        ));
      }
    }

    let block_identifier =
      BlockIdentifier::new(user_command.height.unwrap_or_default(), user_command.state_hash.unwrap_or_default());
    let transaction = Transaction {
      transaction_identifier: Box::new(TransactionIdentifier::new(user_command.hash)),
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

pub struct SearchTransactionsQueryParams {
  pub max_block: Option<i64>,
  pub transaction_hash: Option<String>,
  pub account_identifier: Option<String>,
  pub token_id: Option<String>,
  pub status: Option<TransactionStatus>,
  pub success_status: Option<TransactionStatus>,
  pub address: Option<String>,
}

impl TryFrom<SearchTransactionsRequest> for SearchTransactionsQueryParams {
  type Error = MinaMeshError;

  fn try_from(req: SearchTransactionsRequest) -> Result<Self, Self::Error> {
    let max_block = req.max_block;
    let transaction_hash = req.transaction_identifier.map(|t| t.hash);
    let token_id = req.account_identifier.as_ref().and_then(|a| a.metadata.as_ref().map(|meta| meta.to_string()));
    let account_identifier = req.account_identifier.map(|a| a.address);

    let status = match req.status.as_deref() {
      Some("applied") => Some(TransactionStatus::Applied),
      Some("failed") => Some(TransactionStatus::Failed),
      Some(other) => {
        return Err(MinaMeshError::Exception(format!(
          "Invalid transaction status: '{}'. Valid statuses are 'applied' and 'failed'",
          other
        )));
      }
      None => None,
    };

    let success_status = match req.success {
      Some(true) => Some(TransactionStatus::Applied),
      Some(false) => Some(TransactionStatus::Failed),
      None => None,
    };

    let address = req.address;

    Ok(SearchTransactionsQueryParams {
      max_block,
      transaction_hash,
      account_identifier,
      token_id,
      status,
      success_status,
      address,
    })
  }
}

fn adjust_limit_and_offset(mut limit: i64, mut offset: i64, txs_len: i64) -> (i64, i64) {
  if offset >= txs_len {
    offset -= txs_len;
  } else {
    offset = 0;
  }
  if limit >= txs_len {
    limit -= txs_len;
  } else {
    limit = 0;
  }
  (offset, limit)
}
