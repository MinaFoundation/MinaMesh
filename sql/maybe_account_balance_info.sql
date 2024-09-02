SELECT
  b.height,
  b.global_slot_since_genesis AS block_global_slot_since_genesis,
  balance,
  nonce,
  timing_id
FROM
  blocks b
  INNER JOIN accounts_accessed ac ON ac.block_id=b.id
  INNER JOIN account_identifiers ai ON ai.id=ac.account_identifier_id
  INNER JOIN public_keys pks ON ai.public_key_id=pks.id
  INNER JOIN tokens t ON ai.token_id=t.id
WHERE
  pks.value=$1
  AND b.height<=$2
  AND b.chain_status='canonical'
  AND t.value=$3
ORDER BY
  (b.height) DESC
LIMIT
  1
