use coinbase_mesh::models::{
  BlockIdentifier, BlockTransaction, SearchTransactionsRequest, SearchTransactionsResponse, Transaction,
  TransactionIdentifier,
};

use crate::{
  generate_internal_command_transaction_identifier, generate_operations_internal_command,
  generate_operations_user_command, generate_operations_zkapp_command, generate_transaction_metadata, ChainStatus,
  InternalCommand, InternalCommandType, MinaMesh, MinaMeshError, TransactionStatus, UserCommand, UserCommandType,
  ZkAppCommand,
};

impl MinaMesh {
  pub async fn search_transactions(
    &self,
    req: SearchTransactionsRequest,
  ) -> Result<SearchTransactionsResponse, MinaMeshError> {
    self.validate_network(&req.network_identifier).await?;
    let original_offset = req.offset.unwrap_or(0);
    let limit = req.limit.unwrap_or(100);
    let mut total_count = 0;

    // Fetch total counts concurrently
    let (user_commands_total, internal_commands_total, zkapp_commands_total) = tokio::try_join!(
      async {
        self.fetch_user_commands(&req, 0, 1).await.map(|cmds| cmds.first().and_then(|uc| uc.total_count).unwrap_or(0))
      },
      async {
        self
          .fetch_internal_commands(&req, 0, 1)
          .await
          .map(|cmds| cmds.first().and_then(|ic| ic.total_count).unwrap_or(0))
      },
      async {
        self.fetch_zkapp_commands(&req, 0, 1).await.map(|cmds| cmds.first().and_then(|zc| zc.total_count).unwrap_or(0))
      }
    )?;

    total_count = user_commands_total + internal_commands_total + zkapp_commands_total;

    // Adjust offset and limit for each category
    let user_command_offset = original_offset;
    let user_command_limit = limit.min(user_commands_total.saturating_sub(user_command_offset));

    let internal_command_offset = (user_command_offset + user_command_limit).saturating_sub(user_commands_total);
    let internal_command_limit = limit.min(internal_commands_total.saturating_sub(internal_command_offset));

    let zkapp_command_offset =
      (internal_command_offset + internal_command_limit).saturating_sub(internal_commands_total);
    let zkapp_command_limit = limit.min(zkapp_commands_total.saturating_sub(zkapp_command_offset));

    // Fetch data concurrently
    let (user_commands, internal_commands, zkapp_commands) = tokio::try_join!(
      async { self.fetch_user_commands(&req, user_command_offset, user_command_limit).await },
      async { self.fetch_internal_commands(&req, internal_command_offset, internal_command_limit).await },
      async { self.fetch_zkapp_commands(&req, zkapp_command_offset, zkapp_command_limit).await }
    )?;

    // Aggregate transactions
    let mut transactions = Vec::new();
    transactions.extend(user_commands.into_iter().map(|uc| uc.into()));
    transactions.extend(internal_commands.into_iter().map(|ic| ic.into()));
    transactions.extend(zkapp_commands_to_block_transactions(zkapp_commands));

    // Determine the next offset
    let next_offset = original_offset + transactions.len() as i64;
    let response = SearchTransactionsResponse {
      transactions,
      total_count,
      next_offset: if next_offset < total_count { Some(next_offset) } else { None },
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

    if !self.search_tx_optimized {
      let user_commands = sqlx::query_file_as!(
        UserCommand,
        "sql/queries/indexer_user_commands.sql",
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
    } else {
      let user_commands = sqlx::query_file_as!(
        UserCommand,
        "sql/queries/indexer_user_commands_optimized.sql",
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
  }

  pub async fn fetch_internal_commands(
    &self,
    req: &SearchTransactionsRequest,
    offset: i64,
    limit: i64,
  ) -> Result<Vec<InternalCommand>, MinaMeshError> {
    let query_params = SearchTransactionsQueryParams::try_from(req.clone())?;

    if !self.search_tx_optimized {
      let internal_commands = sqlx::query_file_as!(
        InternalCommand,
        "sql/queries/indexer_internal_commands.sql",
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
    } else {
      let internal_commands = sqlx::query_file_as!(
        InternalCommand,
        "sql/queries/indexer_internal_commands_optimized.sql",
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
  }

  async fn fetch_zkapp_commands(
    &self,
    req: &SearchTransactionsRequest,
    offset: i64,
    limit: i64,
  ) -> Result<Vec<ZkAppCommand>, MinaMeshError> {
    let query_params = SearchTransactionsQueryParams::try_from(req.clone())?;

    if !self.search_tx_optimized {
      let zkapp_commands = sqlx::query_file_as!(
        ZkAppCommand,
        "sql/queries/indexer_zkapp_commands.sql",
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

      Ok(zkapp_commands)
    } else {
      let zkapp_commands = sqlx::query_file_as!(
        ZkAppCommand,
        "sql/queries/indexer_zkapp_commands_optimized.sql",
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

      Ok(zkapp_commands)
    }
  }
}

pub fn zkapp_commands_to_block_transactions(commands: Vec<ZkAppCommand>) -> Vec<BlockTransaction> {
  let block_map = generate_operations_zkapp_command(commands);

  let mut result = Vec::new();
  for ((block_index, block_hash), tx_map) in block_map {
    let block_index = block_index.unwrap_or(0);
    let block_hash = block_hash.unwrap_or_default();
    for (tx_hash, operations) in tx_map {
      let transaction = BlockTransaction {
        block_identifier: Box::new(BlockIdentifier { index: block_index, hash: block_hash.clone() }),
        transaction: Box::new(Transaction {
          transaction_identifier: Box::new(TransactionIdentifier { hash: tx_hash }),
          operations,
          metadata: None,
          related_transactions: None,
        }),
      };
      result.push(transaction);
    }
  }

  result
}

impl From<InternalCommand> for BlockTransaction {
  fn from(internal_command: InternalCommand) -> Self {
    // Derive transaction_identifier by combining command_type, sequence numbers,
    // and the hash
    let transaction_identifier = generate_internal_command_transaction_identifier(
      &internal_command.command_type,
      internal_command.sequence_no,
      internal_command.secondary_sequence_no,
      &internal_command.hash,
    );

    let operations = generate_operations_internal_command(&internal_command);

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

impl From<UserCommand> for BlockTransaction {
  fn from(user_command: UserCommand) -> Self {
    let metadata = generate_transaction_metadata(&user_command);
    let operations = generate_operations_user_command(&user_command);

    let block_identifier =
      BlockIdentifier::new(user_command.height.unwrap_or_default(), user_command.state_hash.unwrap_or_default());
    let transaction = Transaction {
      transaction_identifier: Box::new(TransactionIdentifier::new(user_command.hash)),
      operations,
      metadata,
      related_transactions: None,
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

impl std::fmt::Display for SearchTransactionsQueryParams {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "max_block: {:?}, transaction_hash: {:?}, account_identifier: {:?}, token_id: {:?}, status: {:?}, success_status: {:?}, address: {:?}",
      self.max_block, self.transaction_hash, self.account_identifier, self.token_id, self.status, self.success_status, self.address
    )
  }
}

impl TryFrom<SearchTransactionsRequest> for SearchTransactionsQueryParams {
  type Error = MinaMeshError;

  fn try_from(req: SearchTransactionsRequest) -> Result<Self, Self::Error> {
    let max_block = req.max_block;
    let transaction_hash = req.transaction_identifier.map(|t| t.hash);
    // token_id can be found in the metadata of the account_identifier
    let token_id = req
      .account_identifier
      .as_ref()
      .and_then(|a| a.metadata.as_ref())
      .and_then(|m| m.get("token_id"))
      .map(|t| t.as_str().unwrap().to_string());
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
    let st = SearchTransactionsQueryParams {
      max_block,
      transaction_hash,
      account_identifier,
      token_id,
      status,
      success_status,
      address,
    };
    Ok(st)
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
