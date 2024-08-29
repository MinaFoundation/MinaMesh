SELECT
  b.*,
  pk.value as creator,
  bw.value as winner
FROM
  blocks b
  INNER JOIN public_keys pk ON pk.id = b.creator_id
  INNER JOIN public_keys bw ON bw.id = b.block_winner_id
WHERE
  b.id = $1
