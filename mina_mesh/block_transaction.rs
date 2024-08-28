use crate::common::MinaMeshContext;
use anyhow::Result;
use cynic::QueryBuilder;
use mesh::models::{BlockTransactionRequest, BlockTransactionResponse, Transaction, TransactionIdentifier};
use mina_mesh_graphql::{Block2, QueryBlockTransactions, QueryBlockTransactionsVariables, Transactions};

pub async fn block_transaction(
  context: &MinaMeshContext,
  transaction: &BlockTransactionRequest,
) -> Result<BlockTransactionResponse> {
  let QueryBlockTransactions {
    block: Block2 {
      transactions: Transactions { user_commands },
    },
  } = context
    .graphql(QueryBlockTransactions::build(QueryBlockTransactionsVariables {
      state_hash: Some(transaction.block_identifier.hash.as_str()),
    }))
    .await?;
  Ok(BlockTransactionResponse::new(Transaction {
    operations: user_commands.into_iter().map(|command| command.into()).collect(),
    metadata: None,
    related_transactions: Some(vec![]),
    transaction_identifier: Box::new(TransactionIdentifier::new(
      transaction.transaction_identifier.to_owned().hash,
    )),
  }))
}
