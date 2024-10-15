use std::collections::HashMap;

use coinbase_mesh::models::{AccountIdentifier, Operation, Transaction, TransactionIdentifier};
use serde_json::json;

use crate::{operation, util::DEFAULT_TOKEN_ID, OperationType, TransactionStatus, ZkAppCommand};

pub fn zkapp_commands_to_transactions(commands: Vec<ZkAppCommand>) -> Vec<Transaction> {
  let mut tx_map: HashMap<String, Vec<Operation>> = HashMap::new();

  for command in commands {
    let tx_hash = command.hash.clone();

    // Initialize or update the operation list for this transaction
    let operations = tx_map.entry(tx_hash.clone()).or_default();

    // Add fee operation (zkapp_fee_payer_dec)
    if operations.is_empty() {
      operations.push(operation(
        0,
        Some(&format!("-{}", command.fee)),
        &AccountIdentifier {
          address: command.fee_payer.clone(),
          metadata: Some(json!({ "token_id": DEFAULT_TOKEN_ID })),
          sub_account: None,
        },
        OperationType::ZkappFeePayerDec,
        Some(&TransactionStatus::Applied),
        None,
        None,
      ));
    }

    // Add zkapp balance update operation
    operations.push(operation(
      0,
      Some(&command.balance_change),
      &AccountIdentifier {
        address: command.pk_update_body.clone(),
        metadata: Some(json!({ "token_id": DEFAULT_TOKEN_ID })),
        sub_account: None,
      },
      OperationType::ZkappBalanceUpdate,
      Some(&command.status),
      None,
      None,
    ));
  }

  let mut result = Vec::new();
  for (tx_hash, mut operations) in tx_map {
    // Ensure the operations are correctly indexed
    for (i, operation) in operations.iter_mut().enumerate() {
      operation.operation_identifier.index = i as i64;
    }

    let transaction = Transaction {
      transaction_identifier: Box::new(TransactionIdentifier { hash: tx_hash.clone() }),
      operations,
      metadata: None,
      related_transactions: None,
    };
    result.push(transaction);
  }

  result
}
