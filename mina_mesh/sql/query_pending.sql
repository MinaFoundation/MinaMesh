WITH RECURSIVE chain AS (
  (
    SELECT
      *
    FROM
      blocks
    WHERE
      height = (
        select
          MAX(height)
        from
          blocks
      )
    ORDER BY
      timestamp ASC,
      state_hash ASC
    LIMIT
      1
  )
  UNION
  ALL
  SELECT
    b.*
  FROM
    blocks b
    INNER JOIN chain ON b.id = chain.parent_id
    AND chain.id <> chain.parent_id
    AND chain.chain_status <> 'canonical'
)
SELECT
  c.*,
  pk.value as creator,
  bw.value as winner
FROM
  chain c
  INNER JOIN public_keys pk ON pk.id = c.creator_id
  INNER JOIN public_keys bw ON bw.id = c.block_winner_id
WHERE
  c.height = $1
