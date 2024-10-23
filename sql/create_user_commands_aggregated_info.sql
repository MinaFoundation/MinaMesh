CREATE TABLE user_commands_aggregated (
  id INT,
  command_type user_command_type NOT NULL,
  fee_payer_id INT NOT NULL,
  source_id INT NOT NULL,
  receiver_id INT NOT NULL,
  nonce BIGINT NOT NULL,
  amount TEXT,
  fee TEXT NOT NULL,
  valid_until BIGINT,
  memo TEXT NOT NULL,
  hash TEXT NOT NULL,
  block_id INT NOT NULL,
  sequence_no INT NOT NULL,
  status transaction_status NOT NULL,
  failure_reason TEXT,
  user_command_id INT NOT NULL
);

INSERT INTO
  user_commands_aggregated (
    id,
    command_type,
    fee_payer_id,
    source_id,
    receiver_id,
    nonce,
    amount,
    fee,
    valid_until,
    memo,
    hash,
    block_id,
    sequence_no,
    status,
    failure_reason,
    user_command_id
  )
SELECT
  u.id,
  u.command_type,
  u.fee_payer_id,
  u.source_id,
  u.receiver_id,
  u.nonce,
  u.amount,
  u.fee,
  u.valid_until,
  u.memo,
  u.hash,
  buc.block_id,
  buc.sequence_no,
  buc.status,
  buc.failure_reason,
  buc.user_command_id
FROM
  user_commands AS u
  INNER JOIN blocks_user_commands AS buc ON u.id=buc.user_command_id
