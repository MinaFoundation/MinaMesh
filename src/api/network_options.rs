// TODO: double-check the data is correct
// TODO: why do long string literals in the error metadata break rustfmt?

use coinbase_mesh::models::{Allow, Case, Error, NetworkOptionsResponse, NetworkRequest, OperationStatus, Version};

use crate::{operation_types, MinaMesh, MinaMeshError};

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/network.ml#L444
impl MinaMesh {
  pub async fn network_options(&self, req: NetworkRequest) -> Result<NetworkOptionsResponse, MinaMeshError> {
    self.validate_network(&req.network_identifier).await?;
    let errors: Vec<Error> = MinaMeshError::all_errors().into_iter().map(Error::from).collect();

    Ok(NetworkOptionsResponse::new(Version::new("1.4.9".to_string(), "1.0.0".to_string()), Allow {
      operation_statuses: vec![
        OperationStatus::new("Success".to_string(), true),
        OperationStatus::new("Failed".to_string(), false),
      ],
      operation_types: operation_types(),
      errors,
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
