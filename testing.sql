SELECT COUNT(*)
FROM blocks
WHERE height = ?
  AND chain_status = 'canonical';
