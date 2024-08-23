SELECT b.height,
  b.global_slot_since_genesis AS block_global_slot_since_genesis,
  balance,
  nonce,
  timing_id
FROM blocks b
  INNER JOIN accounts_accessed ac ON ac.block_id = b.id
  INNER JOIN account_identifiers ai on ai.id = ac.account_identifier_id
  INNER JOIN public_keys pks ON ai.public_key_id = pks.id
  INNER JOIN tokens t ON ai.token_id = t.id
WHERE pks.value = "B62qmjJeM4Fd4FVghfhgwoE1fkEexK2Rre8WYKMnbxVwB5vtKUwvgMv"
  AND b.height <= 371513
  AND b.chain_status = 'canonical'
  AND t.value = "wSHV2S4qX9jFsLjQo8r1BsMLH2ZRKsZx6EJd1sbozGPieEC4Jf"
ORDER BY (b.height) DESC
LIMIT 1