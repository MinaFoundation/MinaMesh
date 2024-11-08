use mina_mesh::{models::NetworkRequest, MinaNetwork};

use super::CompareGroup;
use crate::make_loc;

pub fn network_status<'a>() -> CompareGroup<'a> {
  ("/network/status", vec![(
    make_loc!(),
    Box::new(NetworkRequest { network_identifier: Box::new(MinaNetwork::Devnet.into()), metadata: None }),
  )])
}
