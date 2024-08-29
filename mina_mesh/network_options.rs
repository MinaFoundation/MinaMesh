// TODO: double-check the data is correct
// TODO: why do long string literals in the error metadata break rustfmt?

use crate::MinaMesh;
use anyhow::Result;
pub use mesh::models::{Allow, Case, Error, NetworkOptionsResponse, OperationStatus, Version};

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/network.ml#L444
impl MinaMesh {
  pub fn options(&self) -> Result<NetworkOptionsResponse> {
    Ok(NetworkOptionsResponse::new(
      Version::new("1.4.9".into(), "1.0.0".into()),
      Allow {
        operation_statuses: vec![
          OperationStatus::new("Success".into(), true),
          OperationStatus::new("Failed".into(), false),
        ],
        operation_types: vec![
          "fee_payer_dec",
          "fee_receiver_inc",
          "coinbase_inc",
          "account_creation_fee_via_payment",
          "account_creation_fee_via_fee_payer",
          "account_creation_fee_via_fee_receiver",
          "payment_source_dec",
          "payment_receiver_inc",
          "fee_payment",
          "delegate_change",
          "create_token",
          "mint_tokens",
          "zkapp_fee_payer_dec",
          "zkapp_balance_update",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect(),
        errors: vec![
          Error {
            code: 1,
            message: "SQL failure".into(),
            description: Some("We encountered a SQL failure.".into()),
            retriable: false,
            details: None,
          },
          Error {
            code: 2,
            message: "JSON parse error".into(),
            description: Some("We encountered an error while parsing JSON.".into()),
            retriable: false,
            details: None,
          },
          Error {
            code: 3,
            message: "GraphQL query failed".into(),
            description: Some("The GraphQL query failed.".into()),
            retriable: true,
            details: None,
          },
          Error {
            code: 4,
            message: "Network doesn't exist".into(),
            description: Some("The network doesn't exist.".into()),
            retriable: false,
            details: None,
          },
          Error {
            code: 5,
            message: "Chain info missing".into(),
            description: Some("Some chain info is missing.".into()),
            retriable: true,
            details: None,
          },
          Error {
            code: 6,
            message: "Account not found".into(),
            description: Some("That account could not be found.".into()),
            retriable: true,
            details: None,
          },
          Error {
            code: 7,
            message: "Internal invariant violation (you found a bug)".into(),
            description: Some("One of our internal invariants was violated. (That means you found a bug!)".into()),
            retriable: false,
            details: None,
          },
          Error {
            code: 8,
            message: "Transaction not found".into(),
            description: Some("That transaction could not be found.".into()),
            retriable: true,
            details: None,
          },
          Error {
            code: 9,
            message: "Block not found".into(),
            description: Some(
              "We couldn't find the block in the archive node, specified by . Ask a friend for the missing data."
                .into(),
            ),
            retriable: true,
            details: None,
          },
          Error {
            code: 10,
            message: "Malformed public key".into(),
            description: Some("The public key you provided was malformed.".into()),
            retriable: false,
            details: None,
          },
          Error {
            code: 11,
            message: "Cannot convert operations to valid transaction".into(),
            description: Some("We could not convert those operations to a valid transaction.".into()),
            retriable: false,
            details: None,
          },
          Error {
            code: 12,
            message: "Unsupported operation for construction".into(),
            description: Some("An operation you provided isn't supported for construction.".into()),
            retriable: false,
            details: None,
          },
          Error {
            code: 13,
            message: "Signature missing".into(),
            description: Some("Your request is missing a signature.".into()),
            retriable: false,
            details: None,
          },
          Error {
            code: 14,
            message: "Invalid public key format".into(),
            description: Some("The public key you provided had an invalid format.".into()),
            retriable: false,
            details: None,
          },
          Error {
            code: 15,
            message: "No options provided".into(),
            description: Some("Your request is missing options.".into()),
            retriable: false,
            details: None,
          },
          Error {
            code: 16,
            message: "Exception".into(),
            description: Some(
              "We encountered an internal exception while processing your request. (That means you found a bug!)"
                .into(),
            ),
            retriable: false,
            details: None,
          },
          Error {
            code: 17,
            message: "Invalid signature".into(),
            description: Some("Your request has an invalid signature.".into()),
            retriable: false,
            details: None,
          },
          Error {
            code: 18,
            message: "Invalid memo".into(),
            description: Some("Your request has an invalid memo.".into()),
            retriable: false,
            details: None,
          },
          Error {
            code: 19,
            message: "No GraphQL URI set".into(),
            description: Some(
              "This Rosetta instance is running without a GraphQL URI set but this request requires one.".into(),
            ),
            retriable: false,
            details: None,
          },
          Error {
            code: 20,
            message: "Can't send transaction: No sender found in ledger".into(),
            description: Some(
              vec![
                "This could occur because the node isn't fully synced",
                "or the account doesn't actually exist in the ledger yet.",
              ]
              .join(" ")
              .into(),
            ),
            retriable: true,
            details: None,
          },
          Error {
            code: 21,
            message: "Can't send transaction: A duplicate is detected".into(),
            description: Some(
              vec![
                "This could occur if you've already sent this transaction.",
                "Please report a bug if you are confident you didn't already send this exact transaction.",
              ]
              .join(" ")
              .into(),
            ),
            retriable: false,
            details: None,
          },
          Error {
            code: 22,
            message: "Can't send transaction: Nonce invalid".into(),
            description: Some(
              vec![
                "You must use the current nonce in your account in the ledger",
                "or one that is inferred based on pending transactions in the transaction pool.",
              ]
              .join(" ")
              .into(),
            ),
            retriable: false,
            details: None,
          },
          Error {
            code: 23,
            message: "Can't send transaction: Fee too small".into(),
            description: Some(
              "The minimum fee on transactions is 0.001 . Please increase your fee to at least this amount.".into(),
            ),
            retriable: false,
            details: None,
          },
          Error {
            code: 24,
            message: "Can't send transaction: Invalid signature".into(),
            description: Some("An invalid signature is attached to this transaction".into()),
            retriable: false,
            details: None,
          },
          Error {
            code: 25,
            message: "Can't send transaction: Insufficient balance".into(),
            description: Some("This account do not have sufficient balance perform the requested transaction.".into()),
            retriable: false,
            details: None,
          },
          Error {
            code: 26,
            message: "Can't send transaction: Expired".into(),
            description: Some("This transaction is expired. Please try again with a larger valid_until.".into()),
            retriable: false,
            details: None,
          },
        ],
        historical_balance_lookup: true,
        timestamp_start_index: None,
        call_methods: vec![],
        balance_exemptions: vec![],
        mempool_coins: false,
        block_hash_case: Some(Some(Case::CaseSensitive)),
        transaction_hash_case: Some(Some(Case::CaseSensitive)),
      },
    ))
  }
}
