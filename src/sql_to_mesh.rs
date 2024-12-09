use coinbase_mesh::models::{Transaction, TransactionIdentifier};

use crate::{generate_operations_zkapp_command, ZkAppCommand};

pub fn zkapp_commands_to_transactions(commands: Vec<ZkAppCommand>) -> Vec<Transaction> {
  let block_map = generate_operations_zkapp_command(commands);

  let mut result = Vec::new();
  for (_, tx_map) in block_map {
    for (tx_hash, operations) in tx_map {
      let transaction = Transaction {
        transaction_identifier: Box::new(TransactionIdentifier { hash: tx_hash }),
        operations,
        metadata: None,
        related_transactions: None,
      };
      result.push(transaction);
    }
  }

  result
}
