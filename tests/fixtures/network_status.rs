use mina_mesh::models::{NetworkIdentifier, NetworkRequest};

use super::CompareGroup;
use crate::make_loc;

pub fn network_status<'a>() -> CompareGroup<'a> {
  ("/network/status", vec![(
    make_loc!(),
    Box::new(NetworkRequest {
      network_identifier: Box::new(NetworkIdentifier {
        blockchain: "mina".to_string(),
        network: "debug".to_string(),
        sub_network_identifier: None,
      }),
      metadata: None,
    }),
  )])
}
