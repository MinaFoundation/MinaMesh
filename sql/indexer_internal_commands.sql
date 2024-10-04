WITH
  canonical_blocks AS (
    SELECT
      *
    FROM
      blocks
    WHERE
      chain_status='canonical'
  ),
  max_canonical_height AS (
    SELECT
      max(HEIGHT) AS max_height
    FROM
      canonical_blocks
  ),
  pending_blocks AS (
    SELECT
      b.*
    FROM
      blocks AS b,
      max_canonical_height AS m
    WHERE
      b.height>m.max_height
      AND b.chain_status='pending'
  ),
  blocks AS (
    SELECT
      *
    FROM
      canonical_blocks
    UNION ALL
    SELECT
      *
    FROM
      pending_blocks
  ),
  coinbase_receiver_info AS (
    SELECT
      bic.block_id,
      bic.internal_command_id,
      bic.sequence_no,
      bic.secondary_sequence_no,
      coinbase_receiver_pk.value AS coinbase_receiver
    FROM
      blocks_internal_commands AS bic
      INNER JOIN internal_commands AS ic ON bic.internal_command_id=ic.id
      INNER JOIN blocks_internal_commands AS bic_coinbase_receiver ON bic.block_id=bic_coinbase_receiver.block_id
      AND (
        bic.internal_command_id<>bic_coinbase_receiver.internal_command_id
        OR bic.sequence_no<>bic_coinbase_receiver.sequence_no
        OR bic.secondary_sequence_no<>bic_coinbase_receiver.secondary_sequence_no
      )
      INNER JOIN internal_commands AS ic_coinbase_receiver ON ic.command_type='fee_transfer_via_coinbase'
      AND ic_coinbase_receiver.command_type='coinbase'
      AND bic_coinbase_receiver.internal_command_id=ic_coinbase_receiver.id
      INNER JOIN public_keys AS coinbase_receiver_pk ON ic_coinbase_receiver.receiver_id=coinbase_receiver_pk.id
  ),
  internal_commands_info AS (
    SELECT DISTINCT
      ON (
        bic.block_id,
        bic.internal_command_id,
        bic.sequence_no,
        bic.secondary_sequence_no
      ) i.id,
      i.command_type,
      i.receiver_id,
      i.fee,
      i.hash,
      pk.value AS receiver,
      cri.coinbase_receiver,
      bic.sequence_no,
      bic.secondary_sequence_no,
      bic.block_id,
      b.state_hash,
      b.height
    FROM
      internal_commands AS i
      INNER JOIN blocks_internal_commands AS bic ON i.id=bic.internal_command_id
      INNER JOIN public_keys AS pk ON i.receiver_id=pk.id
      INNER JOIN blocks AS b ON bic.block_id=b.id
      LEFT JOIN coinbase_receiver_info AS cri ON bic.block_id=cri.block_id
      AND bic.internal_command_id=cri.internal_command_id
      AND bic.sequence_no=cri.sequence_no
      AND bic.secondary_sequence_no=cri.secondary_sequence_no
    WHERE
      (
        $1<=b.height
        OR $1 IS NULL
      )
      AND (
        $2=i.hash
        OR $2 IS NULL
      )
      AND (
        (
          $3=pk.value
          OR $3=cri.coinbase_receiver
        )
        AND $4=''
        OR (
          $3 IS NULL
          AND $4 IS NULL
        )
      )
      AND (
        $5=bic.status
        OR $5 IS NULL
      )
      AND (
        $6=bic.status
        OR $6 IS NULL
      )
      AND (
        (
          $7=pk.value
          OR $7=cri.coinbase_receiver
        )
        OR $7 IS NULL
      )
  ),
  id_count AS (
    SELECT
      count(*) AS total_count
    FROM
      internal_commands_info
  )
SELECT
  i.*,
  id_count.total_count,
  ac.creation_fee
FROM
  id_count,
  (
    SELECT
      *
    FROM
      internal_commands_info
    ORDER BY
      block_id,
      id,
      sequence_no,
      secondary_sequence_no
    LIMIT
      5
    OFFSET
      0
  ) AS i
  LEFT JOIN account_identifiers AS ai ON i.receiver_id=ai.public_key_id
  LEFT JOIN accounts_created AS ac ON ai.id=ac.account_identifier_id
  AND i.block_id=ac.block_id
  AND i.sequence_no=(
    SELECT
      least(
        (
          SELECT
            min(bic2.sequence_no)
          FROM
            blocks_internal_commands AS bic2
            INNER JOIN internal_commands AS ic2 ON bic2.internal_command_id=ic2.id
          WHERE
            i.receiver_id=ic2.receiver_id
            AND bic2.block_id=i.block_id
            AND bic2.status='applied'
        ),
        (
          SELECT
            min(buc2.sequence_no)
          FROM
            blocks_user_commands AS buc2
            INNER JOIN user_commands AS uc2 ON buc2.user_command_id=uc2.id
          WHERE
            i.receiver_id=uc2.receiver_id
            AND buc2.block_id=i.block_id
            AND buc2.status='applied'
        )
      )
  )
ORDER BY
  i.block_id,
  i.id,
  i.sequence_no,
  i.secondary_sequence_no
