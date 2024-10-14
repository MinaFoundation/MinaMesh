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
  zkapp_commands_info AS (
    SELECT
      zc.id,
      zc.memo,
      zc.hash,
      pk_fee_payer.value AS fee_payer,
      pk_update_body.value AS pk_update_body,
      zfpb.fee,
      zfpb.valid_until,
      zfpb.nonce,
      bzc.sequence_no,
      bzc.status AS "status: TransactionStatus",
      zaub.account_identifier_id,
      zaub.update_id,
      zaub.balance_change,
      zaub.increment_nonce,
      zaub.events_id,
      zaub.actions_id,
      zaub.call_data_id,
      zaub.call_depth,
      zaub.zkapp_network_precondition_id,
      zaub.zkapp_account_precondition_id,
      zaub.zkapp_valid_while_precondition_id,
      zaub.use_full_commitment,
      zaub.implicit_account_creation_fee,
      zaub.may_use_token AS "may_use_token: MayUseToken",
      zaub.authorization_kind AS "authorization_kind: AuthorizationKindType",
      zaub.verification_key_hash_id,
      bzc.block_id,
      b.state_hash,
      b.height,
      ARRAY(
        SELECT
          unnest(zauf.failures)
        FROM
          zkapp_account_update_failures AS zauf
        WHERE
          zauf.id=ANY (bzc.failure_reasons_ids)
      ) AS failure_reasons
    FROM
      zkapp_commands AS zc
      INNER JOIN blocks_zkapp_commands AS bzc ON zc.id=bzc.zkapp_command_id
      INNER JOIN zkapp_fee_payer_body AS zfpb ON zc.zkapp_fee_payer_body_id=zfpb.id
      INNER JOIN public_keys AS pk_fee_payer ON zfpb.public_key_id=pk_fee_payer.id
      INNER JOIN blocks AS b ON bzc.block_id=b.id
      LEFT JOIN zkapp_account_update AS zau ON zau.id=ANY (zc.zkapp_account_updates_ids)
      INNER JOIN zkapp_account_update_body AS zaub ON zau.body_id=zaub.id
      INNER JOIN account_identifiers AS ai_update_body ON zaub.account_identifier_id=ai_update_body.id
      INNER JOIN public_keys AS pk_update_body ON ai_update_body.public_key_id=pk_update_body.id
      INNER JOIN tokens AS token_update_body ON ai_update_body.token_id=token_update_body.id
    WHERE
      (
        $1>=b.height
        OR $1 IS NULL
      )
      AND (
        $2=zc.hash
        OR $2 IS NULL
      )
      AND (
        (
          (
            $3=pk_fee_payer.value
            AND $4=''
          )
          OR (
            $3=pk_update_body.value
            AND $4=token_update_body.value
          )
        )
        OR (
          $3 IS NULL
          AND $4 IS NULL
        )
      )
      AND (
        $5=bzc.status
        OR $5 IS NULL
      )
      AND (
        $6=bzc.status
        OR $6 IS NULL
      )
      AND (
        (
          $7=pk_fee_payer.value
          OR $7=pk_update_body.value
        )
        OR $7 IS NULL
      )
  ),
  zkapp_commands_ids AS (
    SELECT DISTINCT
      id,
      block_id,
      sequence_no
    FROM
      zkapp_commands_info
  ),
  id_count AS (
    SELECT
      count(*) AS total_count
    FROM
      zkapp_commands_ids
  )
SELECT
  zc.*,
  id_count.total_count
FROM
  id_count,
  (
    SELECT
      *
    FROM
      zkapp_commands_ids
    ORDER BY
      block_id,
      id,
      sequence_no
    LIMIT
      $8
    OFFSET
      $9
  ) AS ids
  INNER JOIN zkapp_commands_info AS zc ON ids.id=zc.id
  AND ids.block_id=zc.block_id
  AND ids.sequence_no=zc.sequence_no
ORDER BY
  ids.block_id,
  ids.id,
  ids.sequence_no
