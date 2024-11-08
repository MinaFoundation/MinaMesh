// TODO: double-check the data is correct
// TODO: why do long string literals in the error metadata break rustfmt?

use coinbase_mesh::models::{Allow, Case, Error, NetworkOptionsResponse, NetworkRequest, OperationStatus, Version};

use crate::{MinaMesh, MinaMeshError};

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/network.ml#L444
impl MinaMesh {
  pub async fn network_options(&self, _req: NetworkRequest) -> Result<NetworkOptionsResponse, MinaMeshError> {
    Ok(NetworkOptionsResponse::new(Version::new("1.4.9".to_string(), "1.0.0".to_string()), Allow {
      operation_statuses: vec![
        OperationStatus::new("Success".to_string(), true),
        OperationStatus::new("Failed".to_string(), false),
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
          message: "SQL failure".to_string(),
          description: Some("We encountered a SQL failure.".to_string()),
          retriable: false,
          details: None,
        },
        Error {
          code: 2,
          message: "JSON parse error".to_string(),
          description: Some("We encountered an error while parsing JSON.".to_string()),
          retriable: false,
          details: None,
        },
        Error {
          code: 3,
          message: "GraphQL query failed".to_string(),
          description: Some("The GraphQL query failed.".to_string()),
          retriable: true,
          details: None,
        },
        Error {
          code: 4,
          message: "Network doesn't exist".to_string(),
          description: Some("The network doesn't exist.".to_string()),
          retriable: false,
          details: None,
        },
        Error {
          code: 5,
          message: "Chain info missing".to_string(),
          description: Some("Some chain info is missing.".to_string()),
          retriable: true,
          details: None,
        },
        Error {
          code: 6,
          message: "Account not found".to_string(),
          description: Some("That account could not be found.".to_string()),
          retriable: true,
          details: None,
        },
        Error {
          code: 7,
          message: "Internal invariant violation (you found a bug)".to_string(),
          description: Some("One of our internal invariants was violated. (That means you found a bug!)".to_string()),
          retriable: false,
          details: None,
        },
        Error {
          code: 8,
          message: "Transaction not found".to_string(),
          description: Some("That transaction could not be found.".to_string()),
          retriable: true,
          details: None,
        },
        Error {
          code: 9,
          message: "Block not found".to_string(),
          description: Some(
            "We couldn't find the block in the archive node, specified by . Ask a friend for the missing data."
              .to_string(),
          ),
          retriable: true,
          details: None,
        },
        Error {
          code: 10,
          message: "Malformed public key".to_string(),
          description: Some("The public key you provided was malformed.".to_string()),
          retriable: false,
          details: None,
        },
        Error {
          code: 11,
          message: "Cannot convert operations to valid transaction".to_string(),
          description: Some("We could not convert those operations to a valid transaction.".to_string()),
          retriable: false,
          details: None,
        },
        Error {
          code: 12,
          message: "Unsupported operation for construction".to_string(),
          description: Some("An operation you provided isn't supported for construction.".to_string()),
          retriable: false,
          details: None,
        },
        Error {
          code: 13,
          message: "Signature missing".to_string(),
          description: Some("Your request is missing a signature.".to_string()),
          retriable: false,
          details: None,
        },
        Error {
          code: 14,
          message: "Invalid public key format".to_string(),
          description: Some("The public key you provided had an invalid format.".to_string()),
          retriable: false,
          details: None,
        },
        Error {
          code: 15,
          message: "No options provided".to_string(),
          description: Some("Your request is missing options.".to_string()),
          retriable: false,
          details: None,
        },
        Error {
          code: 16,
          message: "Exception".to_string(),
          description: Some(
            "We encountered an internal exception while processing your request. (That means you found a bug!)"
              .to_string(),
          ),
          retriable: false,
          details: None,
        },
        Error {
          code: 17,
          message: "Invalid signature".to_string(),
          description: Some("Your request has an invalid signature.".to_string()),
          retriable: false,
          details: None,
        },
        Error {
          code: 18,
          message: "Invalid memo".to_string(),
          description: Some("Your request has an invalid memo.".to_string()),
          retriable: false,
          details: None,
        },
        Error {
          code: 19,
          message: "No GraphQL URI set".to_string(),
          description: Some(
            "This Rosetta instance is running without a GraphQL URI set but this request requires one.".to_string(),
          ),
          retriable: false,
          details: None,
        },
        Error {
          code: 20,
          message: "Can't send transaction: No sender found in ledger".to_string(),
          description: Some(
            #[allow(clippy::useless_vec)]
            vec![
              "This could occur because the node isn't fully synced",
              "or the account doesn't actually exist in the ledger yet.",
            ]
            .join(" "),
          ),
          retriable: true,
          details: None,
        },
        Error {
          code: 21,
          message: "Can't send transaction: A duplicate is detected".to_string(),
          description: Some(
            #[allow(clippy::useless_vec)]
            vec![
              "This could occur if you've already sent this transaction.",
              "Please report a bug if you are confident you didn't already send this exact transaction.",
            ]
            .join(" "),
          ),
          retriable: false,
          details: None,
        },
        Error {
          code: 22,
          message: "Can't send transaction: Nonce invalid".to_string(),
          description: Some(
            #[allow(clippy::useless_vec)]
            vec![
              "You must use the current nonce in your account in the ledger",
              "or one that is inferred based on pending transactions in the transaction pool.",
            ]
            .join(" "),
          ),
          retriable: false,
          details: None,
        },
        Error {
          code: 23,
          message: "Can't send transaction: Fee too small".to_string(),
          description: Some(
            "The minimum fee on transactions is 0.001 . Please increase your fee to at least this amount.".to_string(),
          ),
          retriable: false,
          details: None,
        },
        Error {
          code: 24,
          message: "Can't send transaction: Invalid signature".to_string(),
          description: Some("An invalid signature is attached to this transaction".to_string()),
          retriable: false,
          details: None,
        },
        Error {
          code: 25,
          message: "Can't send transaction: Insufficient balance".to_string(),
          description: Some(
            "This account do not have sufficient balance perform the requested transaction.".to_string(),
          ),
          retriable: false,
          details: None,
        },
        Error {
          code: 26,
          message: "Can't send transaction: Expired".to_string(),
          description: Some("This transaction is expired. Please try again with a larger valid_until.".to_string()),
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
    }))
  }
}
