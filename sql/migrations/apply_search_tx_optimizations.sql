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
  user_command_id INT NOT NULL,
  CONSTRAINT user_commands_aggregated_unique UNIQUE (id, block_id, sequence_no)
);

-- NEXT --
CREATE INDEX idx_user_commands_aggregated_hash ON user_commands_aggregated (hash);

-- NEXT --
-- Populate the table with the existing data
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
  INNER JOIN blocks_user_commands AS buc ON u.id=buc.user_command_id;

-- NEXT --
-- Create the trigger function to insert a new row into user_commands_aggregated
CREATE
OR REPLACE function add_to_user_commands_aggregated () returns trigger AS $$
BEGIN
  -- Insert a new row into user_commands_aggregated only if the corresponding entry doesn't already exist
  INSERT INTO user_commands_aggregated (
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
    NEW.block_id,
    NEW.sequence_no,
    NEW.status,
    NEW.failure_reason,
    NEW.user_command_id
  FROM
    user_commands AS u
  WHERE u.id = NEW.user_command_id 
  ON CONFLICT (id, block_id, sequence_no) DO NOTHING; 

  RETURN NEW;
END;
$$ language plpgsql;

-- NEXT --
-- Create the trigger that fires after each insert into blocks_user_commands
CREATE
OR REPLACE trigger trigger_add_to_user_commands_aggregated
AFTER insert ON blocks_user_commands FOR each ROW
EXECUTE function add_to_user_commands_aggregated ();
