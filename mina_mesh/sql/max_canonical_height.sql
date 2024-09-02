SELECT
  MAX(height) as max_canonical_height
FROM
  blocks
WHERE
  chain_status = 'canonical'
