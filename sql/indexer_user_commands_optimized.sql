WITH
  user_command_info AS (
    SELECT DISTINCT
      ON (block_id, user_command_id, sequence_no) id,
      command_type AS "command_type: UserCommandType",
      fee_payer_id,
      source_id,
      receiver_id,
      nonce,
      amount,
      fee,
      valid_until,
      memo,
      hash,
      block_id,
      sequence_no,
      status AS "status: TransactionStatus",
      failure_reason,
      state_hash,
      chain_status AS "chain_status: ChainStatus",
      HEIGHT
    FROM
      user_command_aggregated_info
    WHERE
      (
        $1>=HEIGHT
        OR $1 IS NULL
      )
      AND (
        $2=hash
        OR $2 IS NULL
      )
      AND (
        $3=pk_value
        AND $4=''
        OR (
          $3 IS NULL
          AND $4 IS NULL
        )
      )
      AND (
        $5=status
        OR $5 IS NULL
      )
      AND (
        $6=status
        OR $6 IS NULL
      )
      AND (
        $7=pk_value
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
