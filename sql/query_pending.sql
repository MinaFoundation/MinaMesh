WITH RECURSIVE
  chain AS (
    (
      SELECT
        block_winner_id,
        chain_status::TEXT AS chain_status,
        creator_id,
        global_slot_since_genesis,
        global_slot_since_hard_fork,
        HEIGHT,
        id,
        last_vrf_output,
        ledger_hash,
        min_window_density,
        next_epoch_data_id,
        parent_hash,
        parent_id,
        proposed_protocol_version_id,
        protocol_version_id,
        snarked_ledger_hash_id,
        staking_epoch_data_id,
        state_hash,
        sub_window_densities,
        TIMESTAMP,
        total_currency
      FROM
        blocks
      WHERE
        HEIGHT=(
          SELECT
            max(HEIGHT)
          FROM
            blocks
        )
      ORDER BY
        TIMESTAMP ASC,
        state_hash ASC
      LIMIT
        1
    )
    UNION ALL
    SELECT
      b.block_winner_id,
      b.chain_status::TEXT AS chain_status,
      b.creator_id,
      b.global_slot_since_genesis,
      b.global_slot_since_hard_fork,
      b.height,
      b.id,
      b.last_vrf_output,
      b.ledger_hash,
      b.min_window_density,
      b.next_epoch_data_id,
      b.parent_hash,
      b.parent_id,
      b.proposed_protocol_version_id,
      b.protocol_version_id,
      b.snarked_ledger_hash_id,
      b.staking_epoch_data_id,
      b.state_hash,
      b.sub_window_densities,
      b.timestamp,
      b.total_currency
    FROM
      blocks b
      INNER JOIN chain ON b.id=chain.parent_id
      AND chain.id<>chain.parent_id
      AND chain.chain_status<>'canonical'
  )
SELECT
  c.*,
  pk.value AS creator,
  bw.value AS winner
FROM
  chain c
  INNER JOIN public_keys pk ON pk.id=c.creator_id
  INNER JOIN public_keys bw ON bw.id=c.block_winner_id
WHERE
  c.height=$1
