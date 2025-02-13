use bitvec::prelude::*;
use coinbase_mesh::models::Operation;
use derive_more::derive::Display;
use mina_signer::CompressedPubKey;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use serde_with::{serde_as, DisplayFromStr};
use sqlx::{FromRow, Type};
use strum::IntoEnumIterator;
use strum_macros::{Display as StrumDisplay, EnumIter, EnumString};

use crate::{
  memo::Memo,
  signer_utils::address_to_compressed_pub_key,
  util::DEFAULT_TOKEN_ID,
  MinaMeshError,
  OperationType::*,
  PartialReason::{self, *},
  ROInput,
};

#[derive(Type, Debug, PartialEq, Eq, Serialize)]
#[sqlx(type_name = "chain_status_type", rename_all = "lowercase")]
pub enum ChainStatus {
  Canonical,
  Pending,
  Orphaned,
}

#[derive(Type, Debug, PartialEq, Eq, Serialize, Deserialize, Copy, Clone)]
#[sqlx(type_name = "command_type", rename_all = "lowercase")]
pub enum UserCommandType {
  Payment,
  Delegation,
}

#[derive(Type, Debug, PartialEq, Eq, Serialize, Display)]
#[sqlx(type_name = "internal_command_type", rename_all = "snake_case")]
pub enum InternalCommandType {
  FeeTransferViaCoinbase,
  FeeTransfer,
  Coinbase,
}

#[derive(Type, Debug, PartialEq, Eq, Serialize, Display, Clone)]
#[sqlx(type_name = "transaction_status", rename_all = "lowercase")]
pub enum TransactionStatus {
  Applied,
  Failed,
}

#[derive(Debug, Display)]
pub enum OperationStatus {
  Success,
  Failed,
}

impl From<TransactionStatus> for OperationStatus {
  fn from(status: TransactionStatus) -> Self {
    match status {
      TransactionStatus::Applied => OperationStatus::Success,
      TransactionStatus::Failed => OperationStatus::Failed,
    }
  }
}

#[derive(Debug, StrumDisplay, EnumIter, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum OperationType {
  FeePayerDec,
  FeeReceiverInc,
  CoinbaseInc,
  AccountCreationFeeViaPayment,
  AccountCreationFeeViaFeeReceiver,
  PaymentSourceDec,
  PaymentReceiverInc,
  FeePayment,
  DelegateChange,
  ZkappFeePayerDec,
  ZkappBalanceUpdate,
}

pub fn operation_types() -> Vec<String> {
  OperationType::iter().map(|variant| variant.to_string()).collect()
}

#[derive(Type, Debug, PartialEq, Eq, Serialize, Display)]
#[sqlx(type_name = "may_use_token")]
pub enum MayUseToken {
  No,
  ParentsOwnToken,
  InheritFromParent,
}

#[derive(Type, Debug, PartialEq, Eq, Serialize, Display)]
#[sqlx(type_name = "authorization_kind_type")]
pub enum AuthorizationKindType {
  #[sqlx(rename = "None_given")]
  NoneGiven,
  #[sqlx(rename = "Signature")]
  Signature,
  #[sqlx(rename = "Proof")]
  Proof,
}

#[allow(dead_code)]
#[derive(FromRow)]
pub struct ZkAppCommand {
  pub id: Option<i32>,
  pub memo: Option<String>,
  pub hash: String,
  pub fee_payer: String,
  pub pk_update_body: Option<String>,
  pub fee: String,
  pub valid_until: Option<i64>,
  pub nonce: Option<i64>,
  pub sequence_no: i32,
  pub status: TransactionStatus,
  pub balance_change: Option<String>,
  pub state_hash: Option<String>,
  pub failure_reasons: Option<Vec<String>>,
  pub token: Option<String>,
  pub height: Option<i64>,
  pub total_count: Option<i64>,
  pub block_id: Option<i32>,
  pub timestamp: Option<String>,
}

// Used in search transactions
#[derive(Debug, FromRow)]
pub struct UserCommand {
  pub id: Option<i32>,
  pub command_type: UserCommandType,
  pub fee_payer_id: Option<i32>,
  pub source_id: Option<i32>,
  pub receiver_id: Option<i32>,
  pub nonce: i64,
  pub amount: Option<String>,
  pub fee: Option<String>,
  pub valid_until: Option<i64>,
  pub memo: Option<String>,
  pub hash: String,
  pub block_id: Option<i32>,
  pub sequence_no: Option<i32>,
  pub status: TransactionStatus,
  pub failure_reason: Option<String>,
  pub state_hash: Option<String>,
  pub chain_status: Option<ChainStatus>,
  pub height: Option<i64>,
  pub total_count: Option<i64>,
  pub fee_payer: String,
  pub source: String,
  pub receiver: String,
  pub creation_fee: Option<String>,
  pub timestamp: Option<String>,
}

// Used in search transactions
#[derive(Debug, FromRow)]
pub struct InternalCommand {
  pub id: Option<i32>,
  pub command_type: InternalCommandType,
  pub receiver_id: Option<i32>,
  pub fee: Option<String>,
  pub hash: String,
  pub receiver: String,
  pub coinbase_receiver: Option<String>,
  pub sequence_no: i32,
  pub secondary_sequence_no: i32,
  pub block_id: i32,
  pub status: TransactionStatus,
  pub state_hash: Option<String>,
  pub height: Option<i64>,
  pub total_count: Option<i64>,
  pub creation_fee: Option<String>,
  pub timestamp: Option<String>,
}

pub trait HasTimestamp {
  fn timestamp(&self) -> Option<String>;
}

impl HasTimestamp for UserCommand {
  fn timestamp(&self) -> Option<String> {
    self.timestamp.clone()
  }
}

impl HasTimestamp for InternalCommand {
  fn timestamp(&self) -> Option<String> {
    self.timestamp.clone()
  }
}

// Used in block
#[derive(Debug, PartialEq, Eq, FromRow, Serialize)]
pub struct UserCommandMetadata {
  pub command_type: UserCommandType,
  pub nonce: i64,
  pub amount: Option<String>,
  pub fee: Option<String>,
  pub valid_until: Option<i64>,
  pub memo: Option<String>,
  pub hash: String,
  pub fee_payer: String,
  pub source: String,
  pub receiver: String,
  pub status: TransactionStatus,
  pub failure_reason: Option<String>,
  pub creation_fee: Option<String>,
}

// Common trait for producing operations from user commands
pub trait UserCommandOperationsData {
  fn command_type(&self) -> &UserCommandType;
  fn fee_payer(&self) -> &str;
  fn source(&self) -> &str;
  fn receiver(&self) -> &str;
  fn nonce(&self) -> i64;
  fn memo(&self) -> Option<String>;
  fn amount(&self) -> Option<&str>;
  fn fee(&self) -> &str;
  fn status(&self) -> &TransactionStatus;
  fn failure_reason(&self) -> Option<&str>;
  fn creation_fee(&self) -> Option<&str>;
}

impl UserCommandOperationsData for UserCommand {
  fn command_type(&self) -> &UserCommandType {
    &self.command_type
  }

  fn fee_payer(&self) -> &str {
    &self.fee_payer
  }

  fn source(&self) -> &str {
    &self.source
  }

  fn receiver(&self) -> &str {
    &self.receiver
  }

  fn nonce(&self) -> i64 {
    self.nonce
  }

  fn memo(&self) -> Option<String> {
    self.memo.clone()
  }

  fn amount(&self) -> Option<&str> {
    self.amount.as_deref()
  }

  fn fee(&self) -> &str {
    self.fee.as_deref().unwrap_or("0")
  }

  fn status(&self) -> &TransactionStatus {
    &self.status
  }

  fn failure_reason(&self) -> Option<&str> {
    self.failure_reason.as_deref()
  }

  fn creation_fee(&self) -> Option<&str> {
    self.creation_fee.as_deref()
  }
}

impl UserCommandOperationsData for UserCommandMetadata {
  fn command_type(&self) -> &UserCommandType {
    &self.command_type
  }

  fn fee_payer(&self) -> &str {
    &self.fee_payer
  }

  fn source(&self) -> &str {
    &self.source
  }

  fn receiver(&self) -> &str {
    &self.receiver
  }

  fn nonce(&self) -> i64 {
    self.nonce
  }

  fn memo(&self) -> Option<String> {
    self.memo.clone()
  }

  fn amount(&self) -> Option<&str> {
    self.amount.as_deref()
  }

  fn fee(&self) -> &str {
    self.fee.as_deref().unwrap_or("0")
  }

  fn status(&self) -> &TransactionStatus {
    &self.status
  }

  fn failure_reason(&self) -> Option<&str> {
    self.failure_reason.as_deref()
  }

  fn creation_fee(&self) -> Option<&str> {
    self.creation_fee.as_deref()
  }
}

// Used in block
#[derive(Debug, PartialEq, Eq, FromRow, Serialize)]
pub struct InternalCommandMetadata {
  pub command_type: InternalCommandType,
  pub receiver: String,
  pub fee: Option<String>,
  pub hash: String,
  pub creation_fee: Option<String>,
  pub sequence_no: i32,
  pub secondary_sequence_no: i32,
  pub status: TransactionStatus,
  pub coinbase_receiver: Option<String>,
}

pub trait InternalCommandOperationsData {
  fn command_type(&self) -> &InternalCommandType;
  fn receiver(&self) -> &str;
  fn fee(&self) -> String;
  fn creation_fee(&self) -> Option<&String>;
  fn coinbase_receiver(&self) -> Option<&str>;
  fn status(&self) -> &TransactionStatus;
}

impl InternalCommandOperationsData for InternalCommand {
  fn command_type(&self) -> &InternalCommandType {
    &self.command_type
  }

  fn receiver(&self) -> &str {
    &self.receiver
  }

  fn fee(&self) -> String {
    self.fee.clone().unwrap_or("0".to_string())
  }

  fn creation_fee(&self) -> Option<&String> {
    self.creation_fee.as_ref()
  }

  fn coinbase_receiver(&self) -> Option<&str> {
    self.coinbase_receiver.as_deref()
  }

  fn status(&self) -> &TransactionStatus {
    &self.status
  }
}

impl InternalCommandOperationsData for InternalCommandMetadata {
  fn command_type(&self) -> &InternalCommandType {
    &self.command_type
  }

  fn receiver(&self) -> &str {
    &self.receiver
  }

  fn fee(&self) -> String {
    self.fee.clone().unwrap_or("0".to_string())
  }

  fn creation_fee(&self) -> Option<&String> {
    self.creation_fee.as_ref()
  }

  fn coinbase_receiver(&self) -> Option<&str> {
    self.coinbase_receiver.as_deref()
  }

  fn status(&self) -> &TransactionStatus {
    // Assuming metadata always represents applied status
    &TransactionStatus::Applied
  }
}

#[derive(Debug, Display, Hash, PartialEq, Eq)]
pub enum CacheKey {
  NetworkId,
}

// Construction types
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PreprocessMetadata {
  pub valid_until: Option<String>,
  pub memo: Option<String>,
}

impl PreprocessMetadata {
  pub fn from_json(metadata: Option<Value>) -> Result<Option<Self>, MinaMeshError> {
    if let Some(meta) = metadata {
      serde_json::from_value(meta)
        .map(Some)
        .map_err(|e| MinaMeshError::JsonParse(Some(format!("Failed to parse metadata: {}", e))))
    } else {
      Ok(None)
    }
  }

  pub fn to_json(&self) -> Value {
    serde_json::to_value(self).unwrap_or_default()
  }

  pub fn new(valid_until: Option<String>, memo: Option<String>) -> Self {
    Self { valid_until, memo }
  }
}

#[derive(Debug, Clone)]
pub struct UserCommandPayload {
  pub fee: u64,
  pub fee_payer: CompressedPubKey,
  pub nonce: u32,
  pub valid_until: Option<u32>,
  pub memo: Memo,
  pub body: UserCommandBody,
}

impl UserCommandPayload {
  pub fn to_random_oracle_input(&self) -> ROInput {
    TransactionUnionPayload::from(self).to_random_oracle_input()
  }
}

#[derive(Debug, Clone)]
pub enum UserCommandBody {
  Payment { receiver: CompressedPubKey, amount: u64 },
  Delegation { new_delegate: CompressedPubKey },
}

impl UserCommandBody {
  pub fn is_payment(&self) -> bool {
    matches!(self, UserCommandBody::Payment { .. })
  }

  pub fn is_delegation(&self) -> bool {
    matches!(self, UserCommandBody::Delegation { .. })
  }

  pub fn receiver_is_odd(&self) -> bool {
    match self {
      UserCommandBody::Payment { receiver, .. } => receiver.is_odd,
      UserCommandBody::Delegation { new_delegate } => new_delegate.is_odd,
    }
  }

  pub fn receiver(&self) -> &CompressedPubKey {
    match self {
      UserCommandBody::Payment { receiver, .. } => receiver,
      UserCommandBody::Delegation { new_delegate } => new_delegate,
    }
  }

  pub fn amount(&self) -> u64 {
    match self {
      UserCommandBody::Payment { amount, .. } => *amount,
      _ => 0,
    }
  }
}

pub enum Tag {
  Payment,
  StakeDelegation,
  FeeTransfer,
  Coinbase,
}

impl Tag {
  pub fn to_bits(&self) -> [bool; 3] {
    let i: u8 = self.into();
    fn test_mask(i: u8, mask: u8) -> bool {
      i & mask == mask
    }
    [test_mask(i, 0b100), test_mask(i, 0b10), test_mask(i, 0b1)]
  }
}

impl From<&Tag> for u8 {
  fn from(tag: &Tag) -> u8 {
    match tag {
      Tag::Payment => 0,
      Tag::StakeDelegation => 1,
      Tag::FeeTransfer => 2,
      Tag::Coinbase => 3,
    }
  }
}

#[derive(PartialEq)]
struct TagUnpacked {
  is_payment: bool,
  is_stake_delegation: bool,
  is_fee_transfer: bool,
  is_coinbase: bool,
  is_user_command: bool,
}

impl TagUnpacked {
  const PAYMENT: TagUnpacked = TagUnpacked {
    is_payment: true,
    is_user_command: true,
    is_stake_delegation: false,
    is_fee_transfer: false,
    is_coinbase: false,
  };

  const STAKE_DELEGATION: TagUnpacked = TagUnpacked {
    is_stake_delegation: true,
    is_user_command: true,
    is_payment: false,
    is_fee_transfer: false,
    is_coinbase: false,
  };

  const FEE_TRANSFER: TagUnpacked = TagUnpacked {
    is_fee_transfer: true,
    is_user_command: false,
    is_payment: false,
    is_stake_delegation: false,
    is_coinbase: false,
  };

  const COINBASE: TagUnpacked = TagUnpacked {
    is_coinbase: true,
    is_user_command: false,
    is_payment: false,
    is_stake_delegation: false,
    is_fee_transfer: false,
  };
}

impl From<Tag> for TagUnpacked {
  fn from(tag: Tag) -> TagUnpacked {
    match tag {
      Tag::Payment => TagUnpacked::PAYMENT,
      Tag::StakeDelegation => TagUnpacked::STAKE_DELEGATION,
      Tag::FeeTransfer => TagUnpacked::FEE_TRANSFER,
      Tag::Coinbase => TagUnpacked::COINBASE,
    }
  }
}

pub struct TransactionUnionPayload {
  pub common: UserCommandPayload,
  pub fee_payer_token: u64,
  pub tag: Tag,
  pub source_pk: CompressedPubKey,
  pub receiver_pk: CompressedPubKey,
  pub token_id: u64,
  pub amount: u64,
}

impl From<&UserCommandPayload> for TransactionUnionPayload {
  fn from(cmd: &UserCommandPayload) -> Self {
    let (tag, receiver_pk, amount) = match &cmd.body {
      UserCommandBody::Payment { receiver, amount } => (Tag::Payment, receiver.clone(), *amount),
      UserCommandBody::Delegation { new_delegate } => (Tag::StakeDelegation, new_delegate.clone(), 0),
    };

    TransactionUnionPayload {
      common: cmd.clone(),
      fee_payer_token: 1,
      tag,
      source_pk: cmd.fee_payer.clone(),
      receiver_pk,
      token_id: 1,
      amount,
    }
  }
}

impl TransactionUnionPayload {
  pub fn to_random_oracle_input(&self) -> ROInput {
    let [tag_bit_1, tag_bit_2, tag_bit_3] = self.tag.to_bits();
    // 1. Append common part
    ROInput::new()
    .append_u64(self.common.fee)
    .append_u64(1) // fee token
    .append_field(self.common.fee_payer.x)
    .append_bool(self.common.fee_payer.is_odd)
    .append_u32(self.common.nonce)
    .append_u32(self.common.valid_until.unwrap_or(u32::MAX))
    .append_bitstring(self.common.memo.0.view_bits::<Lsb0>())
    // 2. Append body part.
    .append_bool(tag_bit_1) // Tag
    .append_bool(tag_bit_2)
    .append_bool(tag_bit_3)
    .append_field(self.source_pk.x) // Source
    .append_bool(self.source_pk.is_odd)
    .append_field(self.receiver_pk.x) // Receiver
    .append_bool(self.receiver_pk.is_odd)
    .append_u64(1) // token id
    .append_u64(self.amount) // Amount
    .append_bool(false)
  }
}

#[tokio::test]
async fn test_transaction_union_payload() {
  let cmd = UserCommandPayload {
    fee: 100000,
    fee_payer: CompressedPubKey::from_address("B62qkUHaJUHERZuCHQhXCQ8xsGBqyYSgjQsKnKN5HhSJecakuJ4pYyk").unwrap(),
    nonce: 1984,
    valid_until: None,
    memo: Memo::from_string("hello").unwrap(),
    body: UserCommandBody::Payment {
      receiver: CompressedPubKey::from_address("B62qoDWfBZUxKpaoQCoFqr12wkaY84FrhxXNXzgBkMUi2Tz4K8kBDiv").unwrap(),
      amount: 5000000000,
    },
  };

  let roi: ROInput = cmd.to_random_oracle_input();
  let roi_hex: String = hex::encode(roi.serialize_mesh_1()).to_uppercase();
  assert_eq!(roi_hex, "0000000327EA74CB13D3F1864C2E60C967577C055FD458D5AF93A59371905B8490B6567827EA74CB13D3F1864C2E60C967577C055FD458D5AF93A59371905B8490B656785E6737A0AC0A147918437FC8C21EA57CECFB613E711CA2E4FD328401657C291C000002570561800000000000800000000000000001F000007FFFFFFFC0500B531B1B7B000000000000000000000000000000000000000000000000000000060000000000000000013E815200000000");
}

#[derive(Serialize, Deserialize)]
pub struct TransactionUnsigned {
  #[serde(rename = "randomOracleInput")]
  pub random_oracle_input: String,
  #[serde(rename = "signerInput")]
  pub signer_input: SignerInput,
  pub payment: Option<Payment>,
  #[serde(rename = "stakeDelegation")]
  pub stake_delegation: Option<StakeDelegation>,
}

impl From<&UserCommandPayload> for TransactionUnsigned {
  fn from(cmd: &UserCommandPayload) -> Self {
    let random_oracle_input = cmd.to_random_oracle_input();
    let roi_hex = hex::encode(random_oracle_input.serialize_mesh_1()).to_uppercase();
    let signer_input: SignerInput = (&random_oracle_input).into();

    match &cmd.body {
      UserCommandBody::Payment { receiver, amount } => TransactionUnsigned {
        random_oracle_input: roi_hex,
        signer_input,
        payment: Some(Payment {
          to: receiver.into_address(),
          from: cmd.fee_payer.into_address(),
          fee: cmd.fee,
          token: "1".to_string(),
          nonce: cmd.nonce,
          memo: Some(cmd.memo.as_string()),
          amount: *amount,
          valid_until: cmd.valid_until,
        }),
        stake_delegation: None,
      },
      UserCommandBody::Delegation { new_delegate } => TransactionUnsigned {
        random_oracle_input: roi_hex,
        signer_input,
        payment: None,
        stake_delegation: Some(StakeDelegation {
          delegator: cmd.fee_payer.into_address(),
          new_delegate: new_delegate.into_address(),
          fee: cmd.fee,
          nonce: cmd.nonce,
          memo: Some(cmd.memo.as_string()),
          valid_until: cmd.valid_until,
        }),
      },
    }
  }
}

impl TransactionUnsigned {
  pub fn as_json_string(&self) -> Result<String, MinaMeshError> {
    serde_json::to_string(self)
      .map_err(|e| MinaMeshError::JsonParse(Some(format!("Failed to serialize unsigned transaction: {}", e))))
  }
}

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct Payment {
  pub to: String,
  pub from: String,
  #[serde_as(as = "DisplayFromStr")]
  pub fee: u64,
  pub token: String,
  #[serde_as(as = "DisplayFromStr")]
  pub nonce: u32,
  pub memo: Option<String>,
  #[serde_as(as = "DisplayFromStr")]
  pub amount: u64,
  #[serde_as(as = "Option<DisplayFromStr>")]
  pub valid_until: Option<u32>,
}

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct StakeDelegation {
  pub delegator: String,
  pub new_delegate: String,
  #[serde_as(as = "DisplayFromStr")]
  pub fee: u64,
  #[serde_as(as = "DisplayFromStr")]
  pub nonce: u32,
  pub memo: Option<String>,
  #[serde_as(as = "Option<DisplayFromStr>")]
  pub valid_until: Option<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct SignerInput {
  pub prefix: Vec<String>,
  pub suffix: Vec<String>,
}

impl From<&ROInput> for SignerInput {
  fn from(roinput: &ROInput) -> Self {
    let prefix = roinput
          .fields_to_bytes_mesh()
          .chunks(32) // Ensure fields are split properly
          .map(|chunk| hex::encode(chunk).to_uppercase())
          .collect();

    let suffix = roinput.pack_bits_to_254_fields();

    Self { prefix, suffix }
  }
}

#[tokio::test]
async fn test_signer_input() {
  let test_cases = vec![
    (
      "Payment",
      UserCommandPayload {
        fee: 100000,
        fee_payer: CompressedPubKey::from_address("B62qkUHaJUHERZuCHQhXCQ8xsGBqyYSgjQsKnKN5HhSJecakuJ4pYyk").unwrap(),
        nonce: 1984,
        valid_until: None,
        memo: Memo::from_string("hello").unwrap(),
        body: UserCommandBody::Payment {
          receiver: CompressedPubKey::from_address("B62qoDWfBZUxKpaoQCoFqr12wkaY84FrhxXNXzgBkMUi2Tz4K8kBDiv").unwrap(),
          amount: 5000000000,
        },
      },
      vec![
        "27EA74CB13D3F1864C2E60C967577C055FD458D5AF93A59371905B8490B65678",
        "27EA74CB13D3F1864C2E60C967577C055FD458D5AF93A59371905B8490B65678",
        "5E6737A0AC0A147918437FC8C21EA57CECFB613E711CA2E4FD328401657C291C",
      ],
      vec![
        "01BDB1B195A01407FFFFFFFC00001F0000000000000000020000000000030D40",
        "0000000003000000000000000000000000000000000000000000000000000000",
        "00000000000000000000000000000000000000000000000009502F9000000000",
      ],
    ),
    (
      "Delegation",
      UserCommandPayload {
        fee: 10100000,
        fee_payer: CompressedPubKey::from_address("B62qkXajxfnicuCNtaurdAhQpkFsqjoyPJuw53aeJP848bsa3Ne3RvB").unwrap(),
        nonce: 3,
        valid_until: Some(200000),
        memo: Memo::from_string("hello").unwrap(),
        body: UserCommandBody::Delegation {
          // cspell: disable-next-line
          new_delegate: CompressedPubKey::from_address("B62qiburnzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzmp7r7UN6X")
            .unwrap(),
        },
      },
      vec![
        "34411FBC9BF58536335A3B711494DA6EC9916AFC520F389B66D00796DCD9BA7A",
        "34411FBC9BF58536335A3B711494DA6EC9916AFC520F389B66D00796DCD9BA7A",
        "0131D887E9AE69AF4D40469B25411CB7EB94CDAD60E23B71E608B0A58FBCD408",
      ],
      vec![
        "01BDB1B195A01404000C35000000000E00000000000000020000000001343A40",
        "0000000002C00000000000000000000000000000000000000000000000000000",
        "0000000000000000000000000000000000000000000000000000000000000000",
      ],
    ),
  ];

  for (label, cmd, expected_prefix, expected_suffix) in test_cases {
    let roi: ROInput = cmd.to_random_oracle_input();
    let signer_input: SignerInput = (&roi).into();

    assert_eq!(signer_input.prefix, expected_prefix, "Prefix mismatch for {}", label);
    assert_eq!(signer_input.suffix, expected_suffix, "Suffix mismatch for {}", label);
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PartialUserCommand {
  pub kind: UserCommandType,
  pub fee_payer: String,
  pub source: String,
  pub receiver: String,
  pub fee_token: String,
  pub token: String,
  pub fee: i64,
  pub amount: Option<i64>,
  pub valid_until: Option<String>,
  pub memo: Option<String>,
}

impl PartialUserCommand {
  pub fn to_user_command_payload(&self, nonce: u32) -> Result<UserCommandPayload, MinaMeshError> {
    let fee_payer_pk = address_to_compressed_pub_key("fee_payer", &self.fee_payer)?;
    let source_pk = address_to_compressed_pub_key("source", &self.source)?;
    let receiver_pk = address_to_compressed_pub_key("receiver", &self.receiver)?;

    if fee_payer_pk != source_pk {
      return Err(MinaMeshError::OperationsNotValid(vec![FeePayerAndSourceMismatch]));
    }

    let memo = self.memo.as_deref().map_or(Ok(Memo::empty()), Memo::from_string)?;

    let body = match self.kind {
      UserCommandType::Payment => {
        let amount = self.amount.ok_or(MinaMeshError::OperationsNotValid(vec![AccountNotSome]))? as u64;
        UserCommandBody::Payment { receiver: receiver_pk, amount }
      }
      UserCommandType::Delegation => UserCommandBody::Delegation { new_delegate: receiver_pk },
    };

    Ok(UserCommandPayload {
      fee: self.fee.unsigned_abs(),
      fee_payer: fee_payer_pk,
      nonce,
      valid_until: self.valid_until.as_deref().and_then(|v| v.parse::<u32>().ok()),
      memo,
      body,
    })
  }

  pub fn from_operations(
    operations: &[Operation],
    valid_until: Option<String>,
    memo: Option<String>,
  ) -> Result<Self, MinaMeshError> {
    let mut errors = Vec::new();

    match operations.len() {
      3 => Self::parse_payment_operations(operations, valid_until, memo).map_err(|err| {
        if let MinaMeshError::OperationsNotValid(reasons) = &err {
          errors.extend(reasons.clone());
        }
        MinaMeshError::OperationsNotValid(errors.clone())
      }),
      2 => Self::parse_delegation_operations(operations, valid_until, memo).map_err(|err| {
        if let MinaMeshError::OperationsNotValid(reasons) = &err {
          errors.extend(reasons.clone());
        }
        MinaMeshError::OperationsNotValid(errors.clone())
      }),
      _ => {
        errors.push(PartialReason::LengthMismatch(format!(
          "Expected 2 operations for delegation or 3 for payment, got {}",
          operations.len()
        )));
        Err(MinaMeshError::OperationsNotValid(errors))
      }
    }
  }

  fn parse_payment_operations(
    operations: &[Operation],
    valid_until: Option<String>,
    memo: Option<String>,
  ) -> Result<Self, MinaMeshError> {
    let mut errors = Vec::new();

    let fee_payment = Self::find_operation(operations, FeePayment).inspect_err(|e| {
      errors.push(e.clone());
    });

    let source_dec = Self::find_operation(operations, PaymentSourceDec).inspect_err(|e| {
      errors.push(e.clone());
    });

    let receiver_inc = Self::find_operation(operations, PaymentReceiverInc).inspect_err(|e| {
      errors.push(e.clone());
    });

    if !errors.is_empty() {
      return Err(MinaMeshError::OperationsNotValid(errors));
    }

    let fee_payment = fee_payment.unwrap();
    let source_dec = source_dec.unwrap();
    let receiver_inc = receiver_inc.unwrap();

    let fee_token = Self::token_id_from_operation(fee_payment);
    let token = Self::token_id_from_operation(source_dec);

    if fee_payment.account != source_dec.account {
      errors.push(PartialReason::FeePayerAndSourceMismatch);
    }

    //Validate source and receiver amounts match
    let source_amt = Self::parse_amount_as_i64(source_dec).map_err(|e| {
      errors.push(e.clone());
      MinaMeshError::OperationsNotValid(errors.clone())
    })?;
    let receiver_amt = Self::parse_amount_as_i64(receiver_inc).map_err(|e| {
      errors.push(e.clone());
      MinaMeshError::OperationsNotValid(errors.clone())
    })?;
    if (source_amt + receiver_amt) != 0 {
      errors.push(PartialReason::AmountIncDecMismatch);
    }

    // Validate the fee
    let fee = Self::parse_amount_as_i64(fee_payment).map_err(|e| {
      errors.push(e.clone());
      MinaMeshError::OperationsNotValid(errors.clone())
    })?;
    if fee >= 0 {
      errors.push(PartialReason::FeeNotNegative);
    }

    if !errors.is_empty() {
      return Err(MinaMeshError::OperationsNotValid(errors));
    }

    Ok(PartialUserCommand {
      kind: UserCommandType::Payment,
      fee_payer: Self::address_from_operation(fee_payment),
      source: Self::address_from_operation(source_dec),
      receiver: Self::address_from_operation(receiver_inc),
      fee_token,
      token,
      fee,
      amount: Some(receiver_amt),
      valid_until,
      memo,
    })
  }

  fn parse_delegation_operations(
    operations: &[Operation],
    valid_until: Option<String>,
    memo: Option<String>,
  ) -> Result<Self, MinaMeshError> {
    let mut errors = Vec::new();

    let fee_payment = Self::find_operation(operations, FeePayment).inspect_err(|e| {
      errors.push(e.clone());
    });

    let delegate_change = Self::find_operation(operations, DelegateChange).inspect_err(|e| {
      errors.push(e.clone());
    });

    if !errors.is_empty() {
      return Err(MinaMeshError::OperationsNotValid(errors));
    }

    let fee_payment = fee_payment.unwrap();
    let delegate_change = delegate_change.unwrap();

    let fee_token = Self::token_id_from_operation(fee_payment);
    let token = Self::token_id_from_operation(delegate_change);

    if fee_payment.account != delegate_change.account {
      errors.push(PartialReason::FeePayerAndSourceMismatch);
    }

    // Validate the fee
    let fee = Self::parse_amount_as_i64(fee_payment).map_err(|e| {
      errors.push(e.clone());
      MinaMeshError::OperationsNotValid(errors.clone())
    })?;
    if fee >= 0 {
      errors.push(PartialReason::FeeNotNegative);
    }

    if let Some(metadata) = &delegate_change.metadata {
      // Validate the delegate_change_target is present
      if metadata.get("delegate_change_target").is_none() {
        errors.push(PartialReason::InvalidMetadata(
          "Missing delegate_change_target in delegate_change metadata".to_string(),
        ));
      }
    } else {
      errors.push(PartialReason::InvalidMetadata(
        "Missing delegate_change metadata with delegate_change_target".to_string(),
      ));
    }

    if !errors.is_empty() {
      return Err(MinaMeshError::OperationsNotValid(errors));
    }

    Ok(PartialUserCommand {
      kind: UserCommandType::Delegation,
      fee_payer: Self::address_from_operation(fee_payment),
      source: Self::address_from_operation(fee_payment),
      receiver: Self::address_from_operation(delegate_change),
      fee_token,
      token,
      fee,
      amount: None,
      valid_until,
      memo,
    })
  }

  fn find_operation(operations: &[Operation], op_type: OperationType) -> Result<&Operation, PartialReason> {
    operations
      .iter()
      .find(|op| op.r#type == op_type.to_string())
      .ok_or(PartialReason::CanNotFindKind(op_type.to_string()))
  }

  fn parse_amount_as_i64(operation: &Operation) -> Result<i64, PartialReason> {
    operation
      .amount
      .as_ref()
      .ok_or(PartialReason::AmountNotSome)
      .and_then(|amount| amount.value.parse::<i64>().map_err(|_| PartialReason::AmountNotValid))
  }

  fn token_id_from_operation(operation: &Operation) -> String {
    operation
      .account
      .as_ref()
      .and_then(|account| account.metadata.as_ref())
      .and_then(|meta| meta.get("token_id").and_then(|t| t.as_str()))
      .unwrap_or(DEFAULT_TOKEN_ID)
      .to_string()
  }

  fn address_from_operation(operation: &Operation) -> String {
    if operation.r#type == DelegateChange.to_string() {
      operation
        .metadata
        .as_ref()
        .and_then(|meta| meta.get("delegate_change_target").and_then(|t| t.as_str()))
        .unwrap_or_default()
        .to_string()
    } else {
      operation.account.as_ref().map_or_else(String::new, |acc| acc.address.clone())
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransactionMetadata {
  pub sender: String,
  pub receiver: String,
  pub nonce: String,
  pub token_id: String,
  pub account_creation_fee: Option<String>,
  pub valid_until: Option<String>,
  pub memo: Option<String>,
}

impl TransactionMetadata {
  pub fn new(
    sender: impl Into<String>,
    receiver: impl Into<String>,
    nonce: impl Into<String>,
    token_id: impl Into<String>,
    account_creation_fee: Option<impl Into<String>>,
    valid_until: Option<impl Into<String>>,
    memo: Option<impl Into<String>>,
  ) -> Self {
    TransactionMetadata {
      sender: sender.into(),
      receiver: receiver.into(),
      nonce: nonce.into(),
      token_id: token_id.into(),
      account_creation_fee: account_creation_fee.map(Into::into),
      valid_until: valid_until.map(Into::into),
      memo: memo.map(Into::into),
    }
  }

  /// Convert metadata into JSON for Rosetta responses
  pub fn to_json(&self) -> Value {
    let mut map = serde_json::Map::new();
    map.insert("sender".to_string(), json!(self.sender));
    map.insert("nonce".to_string(), json!(self.nonce));
    map.insert("token_id".to_string(), json!(self.token_id));
    map.insert("receiver".to_string(), json!(self.receiver));

    if let Some(valid_until) = &self.valid_until {
      map.insert("valid_until".to_string(), json!(valid_until));
    }

    if let Some(memo) = &self.memo {
      map.insert("memo".to_string(), json!(memo));
    }

    if let Some(account_creation_fee) = &self.account_creation_fee {
      map.insert("account_creation_fee".to_string(), json!(account_creation_fee));
    }

    json!(map)
  }
}

impl TryFrom<Value> for TransactionMetadata {
  type Error = MinaMeshError;

  fn try_from(value: Value) -> Result<Self, Self::Error> {
    let sender = value
      .get("sender")
      .and_then(Value::as_str)
      .ok_or_else(|| MinaMeshError::JsonParse(Some("Missing sender in metadata".to_string())))?
      .to_string();

    let receiver = value
      .get("receiver")
      .and_then(Value::as_str)
      .ok_or_else(|| MinaMeshError::JsonParse(Some("Missing receiver in metadata".to_string())))?
      .to_string();

    let nonce = value
      .get("nonce")
      .and_then(Value::as_str)
      .ok_or_else(|| MinaMeshError::JsonParse(Some("Missing nonce in metadata".to_string())))?
      .to_string();

    let token_id = value
      .get("token_id")
      .and_then(Value::as_str)
      .ok_or_else(|| MinaMeshError::JsonParse(Some("Missing token_id in metadata".to_string())))?
      .to_string();

    let account_creation_fee = value.get("account_creation_fee").and_then(Value::as_str).map(|s| s.to_string());

    let valid_until = value.get("valid_until").and_then(Value::as_str).map(|s| s.to_string());

    let memo = value.get("memo").and_then(Value::as_str).map(|s| s.to_string());

    Ok(TransactionMetadata { sender, receiver, nonce, token_id, account_creation_fee, valid_until, memo })
  }
}
