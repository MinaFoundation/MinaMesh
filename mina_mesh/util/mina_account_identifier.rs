use anyhow::{anyhow, Context, Result};
use mesh::models::AccountIdentifier;

#[derive(Debug)]
pub struct MinaAccountIdentifier {
  pub public_key: String,
  pub token_id: String,
}

// cspell:disable-next-line
const DEFAULT_TOKEN_ID: &str = "wSHV2S4qX9jFsLjQo8r1BsMLH2ZRKsZx6EJd1sbozGPieEC4Jf";

impl TryInto<MinaAccountIdentifier> for AccountIdentifier {
  type Error = anyhow::Error;
  fn try_into(self) -> Result<MinaAccountIdentifier> {
    let token_id = match self.metadata {
      None => DEFAULT_TOKEN_ID.to_string(),
      Some(serde_json::Value::Object(map)) => map.get("token_id").map(|v| v.to_string()).context("")?,
      _ => Err(anyhow!(""))?,
    };
    Ok(MinaAccountIdentifier {
      public_key: self.address,
      token_id,
    })
  }
}
