WITH RECURSIVE
  chain AS (
    (
      SELECT
        b1.block_winner_id,
        b1.chain_status::TEXT,
        b1.creator_id,
        b1.global_slot_since_genesis,
        b1.global_slot_since_hard_fork,
        b1.height,
        b1.id,
        b1.last_vrf_output,
        b1.ledger_hash,
        b1.min_window_density,
        b1.next_epoch_data_id,
        b1.parent_hash,
        b1.parent_id,
        b1.proposed_protocol_version_id,
        b1.protocol_version_id,
        b1.snarked_ledger_hash_id,
        b1.staking_epoch_data_id,
        b1.state_hash,
        b1.sub_window_densities,
        b1.timestamp,
        b1.total_currency
      FROM
        blocks b1
      WHERE
        b1.height=(
          SELECT
            max(b2.height)
          FROM
            blocks b2
        )
      ORDER BY
        b1.timestamp ASC,
        b1.state_hash ASC
      LIMIT
        1
    )
    UNION ALL
    SELECT
      b.block_winner_id,
      b.chain_status::TEXT,
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
  c.block_winner_id AS "block_winner_id!",
  c.chain_status AS "chain_status: ChainStatus",
  c.creator_id AS "creator_id!",
  c.global_slot_since_genesis AS "global_slot_since_genesis!",
  c.global_slot_since_hard_fork AS "global_slot_since_hard_fork!",
  c.height AS "height!",
  c.id AS "id!",
  c.last_vrf_output AS "last_vrf_output!",
  c.ledger_hash AS "ledger_hash!",
  c.min_window_density AS "min_window_density!",
  c.next_epoch_data_id AS "next_epoch_data_id!",
  c.parent_hash AS "parent_hash!",
  c.parent_id AS "parent_id",
  c.proposed_protocol_version_id AS "proposed_protocol_version_id",
  c.protocol_version_id AS "protocol_version_id!",
  c.snarked_ledger_hash_id AS "snarked_ledger_hash_id!",
  c.staking_epoch_data_id AS "staking_epoch_data_id!",
  c.state_hash AS "state_hash!",
  c.sub_window_densities AS "sub_window_densities!",
  c.timestamp AS "timestamp!",
  c.total_currency AS "total_currency!",
  creator_pk.value AS "creator",
  block_winner_pk.value AS "winner"
FROM
  chain c
  INNER JOIN public_keys creator_pk ON creator_pk.id=c.creator_id
  INNER JOIN public_keys block_winner_pk ON block_winner_pk.id=c.block_winner_id
WHERE
  c.height=$1
