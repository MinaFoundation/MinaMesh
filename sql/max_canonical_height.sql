SELECT
  max(HEIGHT) AS max_canonical_height
FROM
  blocks
WHERE
  chain_status='canonical'
