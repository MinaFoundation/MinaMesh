use coinbase_mesh::models::{
  AccountIdentifier, Amount, Currency, Operation, OperationIdentifier, SyncStatus as MeshSyncStatus,
};

use super::{AccountNonce, Fee, PublicKey, SyncStatus, UserCommand};

impl From<SyncStatus> for MeshSyncStatus {
  fn from(value: SyncStatus) -> Self {
    let (stage, synced) = match value {
      SyncStatus::Bootstrap => ("Bootstrap", false),
      SyncStatus::Catchup => ("Catchup", false),
      SyncStatus::Connecting => ("Connecting", false),
      SyncStatus::Listening => ("Listening", false),
      SyncStatus::Offline => ("Offline", false),
      SyncStatus::Synced => ("Synced", true),
    };
    Self { stage: Some(stage.to_string()), synced: Some(synced), ..Default::default() }
  }
}

impl From<UserCommand> for Operation {
  fn from(value: UserCommand) -> Self {
    let operation_identifier = Box::new(OperationIdentifier::new(0 /* TODO */));
    Operation {
      r#type: value.kind.0,
      status: Some("pending".to_string()),
      account: Some(Box::new(AccountIdentifier::new(value.source.public_key.0))),
      amount: Some(Box::new(Amount::new(value.amount.0, Currency::new("mina".to_string(), 9)))),
      coin_change: None,
      metadata: None,
      operation_identifier,
      related_operations: None,
    }
  }
}

impl From<String> for PublicKey {
  fn from(value: String) -> Self {
    Self(value)
  }
}

impl From<String> for AccountNonce {
  fn from(value: String) -> Self {
    Self(value)
  }
}

impl From<String> for Fee {
  fn from(value: String) -> Self {
    Self(value)
  }
}

impl Fee {
  pub fn to_u64(&self) -> Option<u64> {
    self.0.parse().ok() // Convert String -> u64 safely
  }
}
