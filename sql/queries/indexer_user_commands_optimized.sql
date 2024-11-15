WITH
  user_command_info AS (
    SELECT DISTINCT
      ON (uca.block_id, uca.id, uca.sequence_no) uca.id,
      uca.command_type AS "command_type: UserCommandType",
      uca.fee_payer_id,
      uca.source_id,
      uca.receiver_id,
      uca.nonce,
      uca.amount,
      uca.fee,
      uca.valid_until,
      uca.memo,
      uca.hash,
      uca.block_id,
      uca.sequence_no,
      uca.status AS "status: TransactionStatus",
      uca.failure_reason,
      b.state_hash,
      b.chain_status AS "chain_status: ChainStatus",
      b.height
    FROM
      user_commands_aggregated AS uca
      INNER JOIN public_keys AS pk ON uca.fee_payer_id=pk.id
      OR (
        uca.status='applied'
        AND (
          uca.source_id=pk.id
          OR uca.receiver_id=pk.id
        )
      )
      INNER JOIN blocks AS b ON uca.block_id=b.id
    WHERE
      (
        b.chain_status='canonical'
        OR b.chain_status='pending'
      )
      AND (
        $1>=b.height
        OR $1 IS NULL
      )
      AND (
        $2=uca.hash
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
        $5=uca.status
        OR $5 IS NULL
      )
      AND (
        $6=uca.status
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
