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
          max(HEIGHT)
        FROM
          blocks
        WHERE
          chain_status='canonical'
      )
  ),
  user_command_info AS (
    SELECT DISTINCT
      ON (
        buc.block_id,
        buc.user_command_id,
        buc.sequence_no
      ) u.id,
      u.command_type AS "command_type: UserCommandType",
      u.fee_payer_id,
      u.source_id,
      u.receiver_id,
      u.nonce,
      u.amount,
      u.fee,
      u.valid_until,
      u.memo,
      u.hash,
      buc.block_id,
      buc.sequence_no,
      buc.status AS "status: TransactionStatus",
      buc.failure_reason,
      b.state_hash,
      b.chain_status AS "chain_status: ChainStatus",
      b.height,
      b.timestamp
    FROM
      user_commands AS u
      INNER JOIN blocks_user_commands AS buc ON u.id=buc.user_command_id
      INNER JOIN public_keys AS pk ON u.fee_payer_id=pk.id
      OR (
        buc.status='applied'
        AND (
          u.source_id=pk.id
          OR u.receiver_id=pk.id
        )
      )
      INNER JOIN blocks AS b ON buc.block_id=b.id
    WHERE
      (
        $1>=b.height
        OR $1 IS NULL
      )
      AND (
        $2=u.hash
        OR $2 IS NULL
      )
      AND (
        $3=pk.value
        OR $3 IS NULL
      )
      AND (
        $4=''
        OR $4 IS NULL
      )
      AND (
        $5=buc.status
        OR $5 IS NULL
      )
      AND (
        $6=buc.status
        OR $6 IS NULL
      )
      AND (
        $7=pk.value
        OR $7 IS NULL
      )
  ),
  id_count AS (
    SELECT
      count(*) AS total_count
    FROM
      user_command_info
  )
SELECT
  u.*,
  id_count.total_count,
  pk_payer.value AS fee_payer,
  pk_source.value AS source,
  pk_receiver.value AS receiver,
  ac.creation_fee AS "creation_fee?"
FROM
  id_count,
  (
    SELECT
      *
    FROM
      user_command_info
    ORDER BY
      block_id,
      id,
      sequence_no
    LIMIT
      $8
    OFFSET
      $9
  ) AS u
  INNER JOIN public_keys AS pk_payer ON u.fee_payer_id=pk_payer.id
  INNER JOIN public_keys AS pk_source ON u.source_id=pk_source.id
  INNER JOIN public_keys AS pk_receiver ON u.receiver_id=pk_receiver.id
  /* Account creation fees are attributed to the first successful command in the
  block that mentions the account with the following LEFT JOINs */
  LEFT JOIN account_identifiers AS ai_receiver ON u.receiver_id=ai_receiver.public_key_id
  LEFT JOIN accounts_created AS ac ON u.block_id=ac.block_id
  AND ai_receiver.id=ac.account_identifier_id
  AND u."status: TransactionStatus"='applied'
  AND u.sequence_no=(
    SELECT
      least(
        (
          SELECT
            min(bic2.sequence_no)
          FROM
            blocks_internal_commands AS bic2
            INNER JOIN internal_commands AS ic2 ON bic2.internal_command_id=ic2.id
          WHERE
            u.receiver_id=ic2.receiver_id
            AND bic2.block_id=u.block_id
            AND bic2.status='applied'
        ),
        (
          SELECT
            min(buc2.sequence_no)
          FROM
            blocks_user_commands AS buc2
            INNER JOIN user_commands AS uc2 ON buc2.user_command_id=uc2.id
          WHERE
            u.receiver_id=uc2.receiver_id
            AND buc2.block_id=u.block_id
            AND buc2.status='applied'
        )
      )
  )
ORDER BY
  u.block_id,
  u.id,
  u.sequence_no
