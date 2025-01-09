WITH
  blocks AS (
    SELECT
      *
    FROM
      blocks
    WHERE
      chain_status='canonical'
    UNION ALL
    SELECT
      *
    FROM
      blocks AS b
    WHERE
      b.chain_status='pending'
      AND b.height>(
        SELECT
          max(height)
        FROM
          blocks
        WHERE
          chain_status='canonical'
      )
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
        ica.block_id,
        ica.id,
        ica.sequence_no,
        ica.secondary_sequence_no
      ) ica.id,
      ica.command_type AS "command_type: InternalCommandType",
      ica.receiver_id,
      ica.fee,
      ica.hash,
      ica.receiver AS receiver,
      cri.coinbase_receiver AS "coinbase_receiver?",
      ica.sequence_no,
      ica.secondary_sequence_no,
      ica.block_id,
      ica.status AS "status: TransactionStatus",
      b.state_hash,
      b.height,
      b.timestamp
    FROM
      internal_commands_aggregated AS ica
      INNER JOIN blocks AS b ON ica.block_id=b.id
      LEFT JOIN coinbase_receiver_info AS cri ON ica.block_id=cri.block_id
      AND ica.id=cri.internal_command_id
      AND ica.sequence_no=cri.sequence_no
      AND ica.secondary_sequence_no=cri.secondary_sequence_no
    WHERE
      (
        $1>=b.height
        OR $1 IS NULL
      )
      AND (
        $2=ica.hash
        OR $2 IS NULL
      )
      AND (
        (
          (
            $3=ica.receiver
            OR $3=cri.coinbase_receiver
          )
          OR $3 IS NULL
        )
      )
      AND (
        $4=''
        OR $4 IS NULL
      )
      AND (
        $5=ica.status
        OR $5 IS NULL
      )
      AND (
        $6=ica.status
        OR $6 IS NULL
      )
      AND (
        (
          $7=ica.receiver
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
  ac.creation_fee AS "creation_fee?"
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
      $8
    OFFSET
      $9
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
