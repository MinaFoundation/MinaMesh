mod generated;

pub use generated::*;
use mesh::models::SyncStatus as MeshSyncStatus;

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
