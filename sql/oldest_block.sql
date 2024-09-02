SELECT
  HEIGHT,
  state_hash
FROM
  blocks
ORDER BY
  TIMESTAMP ASC,
  state_hash ASC
LIMIT
  1
