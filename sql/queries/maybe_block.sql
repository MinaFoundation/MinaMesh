SELECT
  height,
  state_hash,
  global_slot_since_genesis
FROM
  blocks
WHERE
  height=$1
  AND chain_status='canonical'
