use crate::common::MinaMesh;
use anyhow::Result;
pub use mesh::models::{CallRequest, CallResponse};

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/construction.ml#L849
impl MinaMesh {
  pub async fn call(&self, _request: CallRequest) -> Result<CallResponse> {
    Ok(CallResponse::new(serde_json::Value::Null, true))
  }
}
