use crate::{SyncStatus, UserCommand};
use mesh::models::{AccountIdentifier, Amount, Currency, Operation, OperationIdentifier, SyncStatus as MeshSyncStatus};

impl Into<MeshSyncStatus> for SyncStatus {
  fn into(self) -> MeshSyncStatus {
    let (stage, synced) = match self {
      Self::Bootstrap => ("Bootstrap", false),
      Self::Catchup => ("Catchup", false),
      Self::Connecting => ("Connecting", false),
      Self::Listening => ("Listening", false),
      Self::Offline => ("Offline", false),
      Self::Synced => ("Synced", true),
    };
    MeshSyncStatus {
      stage: Some(stage.to_string()),
      synced: Some(synced),
      ..Default::default()
    }
  }
}

impl Into<Operation> for UserCommand {
  fn into(self) -> Operation {
    let operation_identifier = Box::new(OperationIdentifier::new(0 /* TODO */));
    Operation {
      r#type: self.kind.0,
      status: Some("pending".to_string()),
      account: Some(Box::new(AccountIdentifier::new(self.source.public_key.0))),
      amount: Some(Box::new(Amount::new(
        self.amount.0,
        Currency::new("mina".to_string(), 9),
      ))),
      coin_change: None,
      metadata: None,
      operation_identifier,
      related_operations: None,
    }
  }
}
