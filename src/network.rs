use std::fmt::Display;

use clap::ValueEnum;
use coinbase_mesh::models::{NetworkIdentifier, NetworkRequest};

use crate::{util::Wrapper, MinaMeshError};

#[derive(ValueEnum, Clone, Debug, Copy)]
pub enum MinaNetwork {
  Mainnet,
  Devnet,
}

impl Display for Wrapper<NetworkRequest> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0.network_identifier.network)
  }
}

impl Display for Wrapper<MinaNetwork> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let network = match self.0 {
      MinaNetwork::Mainnet => "mainnet",
      MinaNetwork::Devnet => "devnet",
    };
    write!(f, "{}", network)
  }
}

impl From<MinaNetwork> for NetworkRequest {
  fn from(value: MinaNetwork) -> Self {
    Self {
      network_identifier: Box::new(NetworkIdentifier::new("mina".to_string(), Wrapper(value).to_string())),
      metadata: None,
    }
  }
}

impl From<MinaNetwork> for NetworkIdentifier {
  fn from(value: MinaNetwork) -> Self {
    Self { blockchain: "mina".into(), network: Wrapper(value).to_string(), sub_network_identifier: None }
  }
}

impl TryFrom<NetworkIdentifier> for MinaNetwork {
  type Error = MinaMeshError;
  fn try_from(value: NetworkIdentifier) -> Result<Self, Self::Error> {
    if value.network == "mainnet" {
      Ok(MinaNetwork::Mainnet)
    } else if value.network == "devnet" {
      Ok(MinaNetwork::Devnet)
    } else {
      Err(MinaMeshError::ChainInfoMissing)
    }
  }
}

impl TryFrom<&NetworkIdentifier> for MinaNetwork {
  type Error = MinaMeshError;
  fn try_from(value: &NetworkIdentifier) -> Result<Self, Self::Error> {
    value.clone().try_into()
  }
}

impl TryFrom<Box<NetworkIdentifier>> for MinaNetwork {
  type Error = MinaMeshError;
  fn try_from(value: Box<NetworkIdentifier>) -> Result<Self, Self::Error> {
    let network = *value;
    network.try_into()
  }
}
