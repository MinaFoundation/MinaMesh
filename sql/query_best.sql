SELECT
  b.block_winner_id,
  b.chain_status AS "chain_status: ChainStatus",
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
  b.total_currency,
  pk.value AS creator,
  bw.value AS winner
FROM
  blocks b
  INNER JOIN public_keys pk ON pk.id=b.creator_id
  INNER JOIN public_keys bw ON bw.id=b.block_winner_id
WHERE
  b.height=(
    SELECT
      max(b.height)
    FROM
      blocks b
  )
ORDER BY
  TIMESTAMP ASC,
  state_hash ASC
LIMIT
  1
