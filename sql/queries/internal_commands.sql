WITH
  internal_commands_cte AS (
    SELECT DISTINCT
      ON (
        i.hash,
        i.command_type,
        bic.sequence_no,
        bic.secondary_sequence_no
      ) i.*,
      ac.creation_fee,
      pk.value AS receiver,
      bic.sequence_no,
      bic.secondary_sequence_no,
      bic.status
    FROM
      internal_commands AS i
      INNER JOIN blocks_internal_commands AS bic ON i.id=bic.internal_command_id
      INNER JOIN public_keys AS pk ON i.receiver_id=pk.id
      INNER JOIN account_identifiers AS ai ON i.receiver_id=ai.public_key_id
      LEFT JOIN accounts_created AS ac ON ai.id=ac.account_identifier_id
      AND bic.block_id=ac.block_id
      AND bic.sequence_no=(
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
                AND bic2.block_id=bic.block_id
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
                AND buc2.block_id=bic.block_id
                AND buc2.status='applied'
            )
          )
      )
      INNER JOIN tokens AS t ON ai.token_id=t.id
    WHERE
      bic.block_id=$1
      AND t.value=$2
  )
SELECT
  ic.command_type AS "command_type: InternalCommandType",
  ic.hash,
  ic.creation_fee AS "creation_fee?",
  ic.receiver,
  ic.sequence_no,
  ic.secondary_sequence_no,
  ic.fee,
  ic.status AS "status: TransactionStatus",
  coinbase_receiver_pk.value AS coinbase_receiver
FROM
  internal_commands_cte AS ic
  LEFT JOIN internal_commands_cte AS ic_coinbase_receiver ON ic.command_type='fee_transfer_via_coinbase'
  AND ic_coinbase_receiver.command_type='coinbase'
  LEFT JOIN public_keys AS coinbase_receiver_pk ON ic_coinbase_receiver.receiver_id=coinbase_receiver_pk.id
