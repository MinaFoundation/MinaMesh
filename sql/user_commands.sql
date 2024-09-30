SELECT
  u.id,
  u.command_type AS "command_type: CommandType",
  u.fee_payer_id,
  u.source_id,
  u.receiver_id,
  u.nonce,
  u.amount,
  u.fee,
  u.valid_until,
  u.memo,
  u.hash,
  pk_payer.value AS fee_payer,
  pk_source.value AS source,
  pk_receiver.value AS receiver,
  buc.status AS "status: TransactionStatus",
  buc.failure_reason,
  ac.creation_fee
FROM
  user_commands AS u
  INNER JOIN blocks_user_commands AS buc ON u.id=buc.user_command_id
  INNER JOIN public_keys AS pk_payer ON u.fee_payer_id=pk_payer.id
  INNER JOIN public_keys AS pk_source ON u.source_id=pk_source.id
  INNER JOIN public_keys AS pk_receiver ON u.receiver_id=pk_receiver.id
  LEFT JOIN account_identifiers AS ai_receiver ON pk_receiver.id=ai_receiver.public_key_id
  /* Account creation fees are attributed to the first successful command in the
  block that mentions the account with the following LEFT JOIN */
  LEFT JOIN accounts_created AS ac ON buc.block_id=ac.block_id
  AND ai_receiver.id=ac.account_identifier_id
  AND buc.status='applied'
  AND buc.sequence_no=(
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
            AND bic2.block_id=buc.block_id
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
            AND buc2.block_id=buc.block_id
            AND buc2.status='applied'
        )
      )
  )
  LEFT JOIN tokens AS t ON ai_receiver.token_id=t.id
WHERE
  buc.block_id=$1
  AND t.value=$2
