SELECT
  b.*,
  creator_pk.value as creator,
  block_winner_pk.value as winner
FROM
  blocks b
  INNER JOIN public_keys creator_pk ON creator_pk.id = b.creator_id
  INNER JOIN public_keys block_winner_pk ON block_winner_pk.id = b.block_winner_id
WHERE
  b.height = $1
  AND b.chain_status = 'canonical'
