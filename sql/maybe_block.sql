SELECT
  HEIGHT,
  state_hash,
  global_slot_since_genesis
FROM
  blocks
WHERE
  HEIGHT=$1
  AND chain_status='canonical'
