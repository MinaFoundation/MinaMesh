SELECT
  zc.id,
  zc.memo,
  zc.hash,
  pk_fee_payer.value AS fee_payer,
  zfpb.fee,
  zfpb.valid_until,
  zfpb.nonce,
  bzc.sequence_no,
  bzc.status AS "status: TransactionStatus",
  b.state_hash,
  b.height,
  b.timestamp,
  bzc.block_id,
  cast(0 AS BIGINT) AS total_count,
  ARRAY(
    SELECT
      unnest(zauf.failures)
    FROM
      zkapp_account_update_failures AS zauf
    WHERE
      zauf.id=ANY (bzc.failure_reasons_ids)
  ) AS failure_reasons,
  zaub.balance_change AS "balance_change?",
  pk_update_body.value AS "pk_update_body?",
  token_update_body.value AS "token?"
FROM
  blocks_zkapp_commands AS bzc
  INNER JOIN zkapp_commands AS zc ON bzc.zkapp_command_id=zc.id
  INNER JOIN zkapp_fee_payer_body AS zfpb ON zc.zkapp_fee_payer_body_id=zfpb.id
  INNER JOIN public_keys AS pk_fee_payer ON zfpb.public_key_id=pk_fee_payer.id
  INNER JOIN blocks AS b ON bzc.block_id=b.id
  LEFT JOIN zkapp_account_update AS zau ON zau.id=ANY (zc.zkapp_account_updates_ids)
  LEFT JOIN zkapp_account_update_body AS zaub ON zau.body_id=zaub.id
  LEFT JOIN account_identifiers AS ai_update_body ON zaub.account_identifier_id=ai_update_body.id
  LEFT JOIN public_keys AS pk_update_body ON ai_update_body.public_key_id=pk_update_body.id
  LEFT JOIN tokens AS token_update_body ON ai_update_body.token_id=token_update_body.id
WHERE
  bzc.block_id=$1
  AND (
    token_update_body.value=$2
    OR token_update_body.id IS NULL
  )
ORDER BY
  zc.id,
  bzc.sequence_no
