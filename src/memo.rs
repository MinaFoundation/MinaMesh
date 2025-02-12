use crate::MinaMeshError;

const MEMO_LENGTH: usize = 34;
const TAG_INDEX: usize = 0;
const LENGTH_INDEX: usize = 1;
const BYTES_TAG: u8 = 0x01;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Memo(pub [u8; MEMO_LENGTH]);

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/lib/rosetta_lib/user_command_info.ml#L152
/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/lib/mina_base/signed_command_memo.ml#L126
impl Memo {
  /// Creates a memo from a string, enforcing a 34-byte length memo (but 32-byte
  /// input length).
  pub fn from_string(s: &str) -> Result<Self, MinaMeshError> {
    let input_bytes = s.as_bytes();
    let input_len = input_bytes.len();

    if input_len > MEMO_LENGTH - 2 {
      return Err(MinaMeshError::MemoInvalid);
    }

    let mut memo_bytes = [0u8; MEMO_LENGTH];
    memo_bytes[TAG_INDEX] = BYTES_TAG;
    memo_bytes[LENGTH_INDEX] = input_len as u8;
    memo_bytes[2 .. 2 + input_len].copy_from_slice(input_bytes);

    Ok(Self(memo_bytes))
  }

  /// Creates an empty memo (all zeros)
  pub fn empty() -> Self {
    Self([0u8; MEMO_LENGTH])
  }

  /// Converts the memo to a string (ignoring trailing zeros)
  pub fn to_string(&self) -> String {
    let len = self.0[LENGTH_INDEX] as usize;
    String::from_utf8_lossy(&self.0[2..2 + len]).into_owned()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use bitvec::prelude::*;

  #[tokio::test]
  async fn from_string() {
    let memo: Memo = Memo::from_string("hello").unwrap();
    assert_eq!(
      memo.0.view_bits::<Lsb0>(),
      bits![u8, Lsb0; 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    );
  }
}
