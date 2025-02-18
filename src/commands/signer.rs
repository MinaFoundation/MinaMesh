use anyhow::Result;
use clap::Parser;
use mina_hasher::roinput;
use mina_signer::{Keypair, NetworkId, SecKey, Signer};

use crate::{TransactionUnsigned, UserCommandPayload};

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
    let secret = SecKey::from_hex(&self.private_key)?;
    let keypair = Keypair::from_secret_key(secret)?;

    let unsigned_transaction = TransactionUnsigned::from_json_string(&self.unsigned_transaction)?;
    let user_command_payload: UserCommandPayload = (&unsigned_transaction).into();
    // let roinput = user_command_payload.to_random_oracle_input();
    // let roinput2 = user_command_payload.to_roinput();
    // assert_eq!(roinput.to_bytes(), roinput2.to_bytes());

    // println!("ROInput: {:?}", hex::encode(roinput.serialize_mesh_1()));

    let mut ctx = mina_signer::create_legacy::<UserCommandPayload>(NetworkId::TESTNET);
    let sig = ctx.sign(&keypair, &user_command_payload);
    println!("{}", format!("{}", sig).to_uppercase());
    use o1_utils::FieldHelpers;
    println!("{}{}", hex::encode(sig.rx.to_bytes()), hex::encode(sig.s.to_bytes()));

    Ok(())
  }
}
