SELECT
  uc.id
FROM
  user_commands uc
  INNER JOIN public_keys AS pks ON pks.id=uc.source_id
  INNER JOIN public_keys AS pkr ON pkr.id=uc.receiver_id
WHERE
  uc.nonce=$1
  AND pks.value=$2
  AND pkr.value=$3
  AND uc.amount=$4
  AND uc.fee=$5
