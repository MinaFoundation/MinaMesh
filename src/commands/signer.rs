use anyhow::{anyhow, Result};
use clap::Parser;
use mina_hasher::roinput;
use mina_signer::{Keypair, NetworkId, SecKey, Signer};
use o1_utils::FieldHelpers;

use crate::{MinaMeshError, TransactionUnsigned, UserCommandPayload};

#[derive(Debug, Parser)]
pub struct SignCommand {
  #[arg(long, required = true, help = "Unsigned transaction string returned from Rosetta")]
  unsigned_transaction: String,

  #[arg(long, required = true, help = "Private key hex bytes")]
  private_key: String,
}

impl SignCommand {
  pub async fn run(&self) -> Result<()> {
    // let keypair = Keypair::rand(&mut rand::rngs::OsRng).expect("failed to
    // generate keypair"); println!("Keypair: {:?}", keypair);
    let (_, secret) = Self::pack(&self.private_key)?;
    let keypair = Keypair::from_secret_key(secret)?;

    let unsigned_transaction = TransactionUnsigned::from_json_string(&self.unsigned_transaction)?;
    let user_command_payload: UserCommandPayload = (&unsigned_transaction).into();
    // let roinput = user_command_payload.to_random_oracle_input();
    // let roinput2 = user_command_payload.to_roinput();
    // assert_eq!(roinput.to_bytes(), roinput2.to_bytes());

    // println!("ROInput: {:?}", hex::encode(roinput.serialize_mesh_1()));

    let mut ctx = mina_signer::create_legacy::<UserCommandPayload>(NetworkId::TESTNET);
    let sig = ctx.sign(&keypair, &user_command_payload);
    // println!("{}", format!("{}", sig).to_uppercase());
    println!("{}", format!("{}{}", hex::encode(sig.rx.to_bytes()), hex::encode(sig.s.to_bytes())).to_uppercase());

    Ok(())
  }

  /// Converts a hex-encoded private key string into a scalar
  fn pack(hex_str: &str) -> Result<(bool, SecKey)> {
    // Ensure the input is exactly 64 hex chars (32 bytes)
    if hex_str.len() != 64 {
      return Err(anyhow!("Invalid private key length, expected 64 hex chars"));
    }

    // Convert hex string to raw bytes (big-endian)
    let mut raw_bytes = hex::decode(hex_str)?;

    if raw_bytes.len() != 32 {
      return Err(anyhow!("Decoded private key length is incorrect"));
    }

    // Reverse byte order to match OCaml behavior
    raw_bytes.reverse();

    // Extract the padding bit (first bit of the last byte)
    let padding_bit = (raw_bytes[0] & 0b10000000) != 0;

    // Remove padding bit by setting it to 0
    raw_bytes[0] &= 0b01111111;

    // Create a `SecKey` from processed bytes
    let secret_key = SecKey::from_bytes(&raw_bytes)?;

    Ok((padding_bit, secret_key))
  }
}
