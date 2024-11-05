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

-- NEXT --
-- Internal commands
CREATE TABLE internal_commands_aggregated (
  id INT NOT NULL,
  command_type internal_command_type NOT NULL,
  receiver_id INT NOT NULL,
  fee TEXT NOT NULL,
  hash TEXT NOT NULL,
  receiver TEXT NOT NULL,
  sequence_no INT NOT NULL,
  secondary_sequence_no INT NOT NULL,
  block_id INT NOT NULL,
  status transaction_status NOT NULL,
  CONSTRAINT internal_commands_aggregated_unique UNIQUE (id, block_id, sequence_no, secondary_sequence_no)
);

-- NEXT --
CREATE INDEX idx_internal_commands_aggregated_hash ON internal_commands_aggregated (hash);

-- NEXT --
-- Populate the table with the existing data
INSERT INTO
  internal_commands_aggregated (
    id,
    command_type,
    receiver_id,
    fee,
    hash,
    receiver,
    sequence_no,
    secondary_sequence_no,
    block_id,
    status
  )
SELECT
  i.id,
  i.command_type AS command_type,
  i.receiver_id,
  i.fee,
  i.hash,
  pk.value AS receiver,
  bic.sequence_no,
  bic.secondary_sequence_no,
  bic.block_id,
  bic.status AS status
FROM
  internal_commands AS i
  INNER JOIN blocks_internal_commands AS bic ON i.id=bic.internal_command_id
  INNER JOIN public_keys AS pk ON i.receiver_id=pk.id;

-- NEXT --
-- Create the trigger function to insert a new row into internal_commands_aggregated
CREATE
OR REPLACE function add_to_internal_commands_aggregated () returns trigger AS $$
BEGIN
  -- Insert a new row into internal_commands_aggregated only if the corresponding entry doesn't already exist
  INSERT INTO internal_commands_aggregated (
      id,
      command_type,
      receiver_id,
      fee,
      hash,
      receiver,
      sequence_no,
      secondary_sequence_no,
      block_id,
      status
  )
  SELECT
    i.id,
    i.command_type,
    i.receiver_id,
    i.fee,
    i.hash,
    pk.value AS receiver,
    NEW.sequence_no,
    NEW.secondary_sequence_no,
    NEW.block_id,
    NEW.status
  FROM
    internal_commands AS i
    INNER JOIN public_keys AS pk ON i.receiver_id = pk.id
  WHERE i.id = NEW.internal_command_id
  ON CONFLICT (id, block_id, sequence_no, secondary_sequence_no) DO NOTHING;

  RETURN NEW;
END;
$$ language plpgsql;

-- NEXT --
-- Create the trigger that fires after each insert into blocks_internal_commands
CREATE TRIGGER trigger_add_to_internal_commands_aggregated
AFTER insert ON blocks_internal_commands FOR each ROW
EXECUTE function add_to_internal_commands_aggregated ();

-- NEXT --
-- ZkApp commands
CREATE TABLE zkapp_commands_aggregated (
  id INT NOT NULL,
  memo TEXT NOT NULL,
  hash TEXT NOT NULL,
  zkapp_account_updates_ids INT[] NOT NULL,
  sequence_no INT NOT NULL,
  status transaction_status NOT NULL,
  block_id INT NOT NULL,
  failure_reasons_ids INT[],
  fee TEXT NOT NULL,
  valid_until BIGINT,
  nonce BIGINT NOT NULL,
  fee_payer TEXT NOT NULL
);

-- NEXT --
CREATE INDEX idx_zkapp_commands_aggregated_hash ON zkapp_commands_aggregated (hash);

-- NEXT --
-- Populate the table with the existing data
INSERT INTO
  zkapp_commands_aggregated (
    id,
    memo,
    hash,
    zkapp_account_updates_ids,
    sequence_no,
    status,
    block_id,
    failure_reasons_ids,
    fee,
    valid_until,
    nonce,
    fee_payer
  )
SELECT
  zc.id,
  zc.memo,
  zc.hash,
  zc.zkapp_account_updates_ids,
  bzc.sequence_no,
  bzc.status,
  bzc.block_id,
  bzc.failure_reasons_ids,
  zfpb.fee,
  zfpb.valid_until,
  zfpb.nonce,
  pk.value AS fee_payer
FROM
  zkapp_commands AS zc
  INNER JOIN blocks_zkapp_commands AS bzc ON zc.id=bzc.zkapp_command_id
  INNER JOIN zkapp_fee_payer_body AS zfpb ON zc.zkapp_fee_payer_body_id=zfpb.id
  INNER JOIN public_keys AS pk ON zfpb.public_key_id=pk.id;

-- NEXT --
-- Create the trigger function to insert a new row into zkapp_commands_aggregated
-- CREATE
-- OR REPLACE function add_to_zkapp_commands_aggregated () returns trigger AS $$
