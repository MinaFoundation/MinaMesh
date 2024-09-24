use crate::MinaMeshError;

// cspell:disable-next-line
const DEFAULT_TOKEN_ID: &str = "wSHV2S4qX9jFsLjQo8r1BsMLH2ZRKsZx6EJd1sbozGPieEC4Jf";

pub trait ToTokenId {
  fn to_token_id(self) -> Result<String, MinaMeshError>;
}

impl ToTokenId for Option<serde_json::Value> {
  fn to_token_id(self) -> Result<String, MinaMeshError> {
    match self {
      None => Ok(DEFAULT_TOKEN_ID.to_string()),
      Some(serde_json::Value::Object(map)) => Ok(
        map
          .get("token_id")
          .map(|v| v.to_string())
          .ok_or(MinaMeshError::JsonParse(None))?,
      ),
      _ => Err(MinaMeshError::JsonParse(None))?,
    }
  }
}
