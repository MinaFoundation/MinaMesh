use mina_mesh::{
  models::{ConstructionDeriveRequest, CurveType::Tweedle, PublicKey},
  test::network_id,
};
use serde_json::json;

use super::CompareGroup;

fn token_ids() -> Vec<String> {
  vec![
    "wSHV2S4qX9jFsLjQo8r1BsMLH2ZRKsZx6EJd1sbozGPieEC4Jf".to_string(),
    "wfG3GivPMttpt6nQnPuX9eDPnoyA5RJZY23LTc4kkNkCRH2gUd".to_string(),
    "xosVXFFDvDiKvHSDAaHvrTSRtoa5Graf2J7LM5Smb4GNTrT2Hn".to_string(),
    "wXqDrUzWtK58CaWCzN2g3zseU275dhSnRtBthcroeqT6HGKkos".to_string(),
    "xBxjFpJkbWpbGua7Lf36S1NLhffFoEChyP3pz6SYKnx7dFCTwg".to_string(),
    "wnGm7B94xkhANu5vZJPjLojRvqWypPPJBTZd1x8rsrFX1iF1Cr".to_string(),
    "wU9TAr8n2djpTPMEmqyzMFyf3DA1hVfgC1xuNgf8b8bGZz18Ri7".to_string(),
    "xNUPtFCWXyv23Rj4jWmEbcX2Hfiu3JSeDynkFq3SmMheTZwSdR".to_string(),
    "xLpobAxWSYZeyKuiEb4kzHHYQKn6X1vKmFR4Dmz9TCADLrYTD1".to_string(),
    "xvoiUpngKPVLAqjqKb5qXQvQBmk9DncPEaGJXzehoRSNrDB45r".to_string(),
    "weihj2SSP7Z96acs56ygP64Te6wauzvWWfAPHKb1gzqem9J4Ne".to_string(),
    "y96qmT865fCMGGHdKAQ448uUwqs7dEfqnGBGVrv3tiRKTC2hxE".to_string(),
  ]
}

fn public_keys() -> Vec<String> {
  vec![
    "7e406ca640115a8c44ece6ef5d0c56af343b1a993d8c871648ab7980ecaf8230".to_string(),
    "fad1d3e31aede102793fb2cce62b4f1e71a214c94ce18ad5756eba67ef398390".to_string(),
  ]
}

fn construction_derive_payloads() -> Vec<Box<dyn erased_serde::Serialize>> {
  let token_ids = token_ids();
  let public_keys = public_keys();

  let mut payloads = Vec::new();

  for token_id in token_ids.iter() {
    for public_key in public_keys.iter() {
      let payload = ConstructionDeriveRequest {
        network_identifier: Box::new(network_id()),
        public_key: Box::new(PublicKey::new(public_key.clone(), Tweedle)),
        metadata: Some(json!({ "token_id": token_id })),
      };
      payloads.push(Box::new(payload) as Box<dyn erased_serde::Serialize>);
    }
  }

  payloads
}

pub fn construction_derive<'a>() -> CompareGroup<'a> {
  let payloads = construction_derive_payloads();
  ("/construction/derive", payloads)
}
