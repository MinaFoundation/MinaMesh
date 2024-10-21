CREATE TABLE user_command_aggregated_info (
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
  state_hash TEXT NOT NULL,
  chain_status chain_status_type NOT NULL,
  HEIGHT BIGINT NOT NULL,
  pk_value TEXT,
  user_command_id INT NOT NULL
);

INSERT INTO
  user_command_aggregated_info (
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
    state_hash,
    chain_status,
    HEIGHT,
    pk_value,
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
  b.state_hash,
  b.chain_status,
  b.height,
  pk.value AS "pk_value",
  buc.user_command_id
FROM
  user_commands AS u
  INNER JOIN blocks_user_commands AS buc ON u.id=buc.user_command_id
  INNER JOIN public_keys AS pk ON u.fee_payer_id=pk.id
  OR (
    buc.status='applied'
    AND (
      u.source_id=pk.id
      OR u.receiver_id=pk.id
    )
  )
  INNER JOIN blocks AS b ON buc.block_id=b.id
  AND b.chain_status='canonical'
  OR b.chain_status='pending';

-- -- Step 2: Create the trigger function to refresh user_command_info
-- CREATE
-- OR REPLACE function refresh_user_command_info () returns trigger AS $$
-- BEGIN
--   -- Refresh the user_command_info table by deleting relevant rows and reinserting them
--   DELETE FROM user_command_info 
--   WHERE block_id = NEW.block_id OR block_id = OLD.block_id;
--   INSERT INTO user_command_info 
--   SELECT DISTINCT
--     ON (
--       buc.block_id,
--       buc.user_command_id,
--       buc.sequence_no
--     ) u.id,
--       u.command_type AS "command_type",
--       u.fee_payer_id,
--       u.source_id,
--       u.receiver_id,
--       u.nonce,
--       u.amount,
--       u.fee,
--       u.valid_until,
--       u.memo,
--       u.hash,
--       buc.block_id,
--       buc.sequence_no,
--       buc.status AS "status",
--       buc.failure_reason,
--       b.state_hash,
--       b.chain_status AS "chain_status",
--       b.height
--     FROM
--       user_commands AS u
--       INNER JOIN blocks_user_commands AS buc ON u.id = buc.user_command_id
--       INNER JOIN public_keys AS pk ON u.fee_payer_id = pk.id
--       OR (
--         buc.status = 'applied'
--         AND (
--           u.source_id = pk.id
--           OR u.receiver_id = pk.id
--         )
--       )
--       INNER JOIN blocks AS b ON buc.block_id = b.id
--     WHERE b.block_id = NEW.block_id OR b.block_id = OLD.block_id;
--   RETURN NEW;
-- END;
-- $$ language plpgsql;
-- -- Step 3: Create the trigger on user_commands
-- CREATE
-- OR REPLACE trigger refresh_user_command_info_on_user_commands
-- AFTER insert
-- OR
-- UPDATE
-- OR delete ON user_commands FOR each ROW
-- EXECUTE function refresh_user_command_info ();
-- -- Step 4: Create the trigger on blocks_user_commands
-- CREATE
-- OR REPLACE trigger refresh_user_command_info_on_blocks_user_commands
-- AFTER insert
-- OR
-- UPDATE
-- OR delete ON blocks_user_commands FOR each ROW
-- EXECUTE function refresh_user_command_info ();
-- -- Step 5: Create the trigger on blocks
-- CREATE
-- OR REPLACE trigger refresh_user_command_info_on_blocks
-- AFTER insert
-- OR
-- UPDATE
-- OR delete ON blocks FOR each ROW
-- EXECUTE function refresh_user_command_info ();
