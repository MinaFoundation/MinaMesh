use crate::common::MinaMeshContext;
use anyhow::Result;
use cynic::QueryBuilder;
use mesh::models::{
  AccountIdentifier, Amount, Currency, MempoolTransactionRequest, MempoolTransactionResponse, Operation,
  OperationIdentifier, Transaction, TransactionIdentifier,
};
use mina_mesh_graphql::{QueryMempoolTransactions, QueryMempoolTransactionsVariables};

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/mempool.ml#L137
pub async fn mempool_transaction(
  context: &MinaMeshContext,
  request: MempoolTransactionRequest,
) -> Result<MempoolTransactionResponse> {
  let QueryMempoolTransactions {
    daemon_status: _daemon_status,
    initial_peers: _initial_peers,
    pooled_user_commands,
  } = context
    .graphql(QueryMempoolTransactions::build(QueryMempoolTransactionsVariables {
      hashes: Some(vec![request.transaction_identifier.hash.as_str()]),
    }))
    .await?;
  let operations = pooled_user_commands
    .into_iter()
    .enumerate()
    .map(|(i, command)| Operation {
      r#type: command.kind.0,
      status: Some("pending".to_string()),
      account: Some(Box::new(AccountIdentifier::new(command.source.public_key.0))),
      amount: Some(Box::new(Amount::new(
        command.amount.0,
        Currency::new("mina".to_string(), 9),
      ))),
      coin_change: None,
      metadata: None,
      operation_identifier: Box::new(OperationIdentifier::new(i as i64)),
      related_operations: None,
    })
    .collect();
  Ok(MempoolTransactionResponse {
    metadata: None,
    transaction: Box::new(Transaction {
      operations,
      related_transactions: Some(vec![]),
      transaction_identifier: Box::new(TransactionIdentifier::new(request.transaction_identifier.hash)),
      metadata: None,
    }),
  })
}
