SELECT
  height,
  state_hash
FROM
  blocks
ORDER BY
  timestamp ASC,
  state_hash ASC
LIMIT
  1
