WITH
  blocks AS (
    SELECT
      height,
      state_hash,
      global_slot_since_genesis
    FROM
      blocks
    WHERE
      chain_status='canonical'
    UNION ALL
    SELECT
      height,
      state_hash,
      global_slot_since_genesis
    FROM
      blocks AS b
    WHERE
      b.chain_status='pending'
      AND b.height>(
        SELECT
          max(height)
        FROM
          blocks
        WHERE
          chain_status='canonical'
      )
  )
SELECT
  height,
  state_hash,
  global_slot_since_genesis
FROM
  blocks
WHERE
  (
    height=$1
    OR $1 IS NULL
  )
  AND (
    state_hash=$2
    OR $2 IS NULL
  )
