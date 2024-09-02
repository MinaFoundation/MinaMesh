use crate::MinaMesh;
use anyhow::anyhow;
use anyhow::Result;
pub use mesh::models::{Block, BlockRequest, BlockResponse, PartialBlockIdentifier};

/// https://github.com/MinaProtocol/mina/blob/985eda49bdfabc046ef9001d3c406e688bc7ec45/src/app/rosetta/lib/block.ml#L7
impl MinaMesh {
  pub async fn block(&self, request: BlockRequest) -> Result<BlockResponse> {
    let PartialBlockIdentifier { index, hash } = *request.block_identifier;
    let _metadata = if let (Some(index), Some(hash)) = (&index, &hash) {
      sqlx::query_file!("sql/query_both.sql", index.to_string(), hash.parse::<i64>()?)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(anyhow!(""))?;
    } else if let Some(index) = index {
      let record = sqlx::query_file!("sql/max_canonical_height.sql")
        .fetch_one(&self.pool)
        .await?;
      if index <= record.max_canonical_height.unwrap() {
        sqlx::query_file!("sql/query_canonical.sql", index)
          .fetch_optional(&self.pool)
          .await?
          .ok_or(anyhow!(""))?;
      } else {
        sqlx::query_file!("sql/query_pending.sql", index)
          .fetch_optional(&self.pool)
          .await?
          .ok_or(anyhow!(""))?;
      }
    } else if let Some(hash) = &hash {
      sqlx::query_file!("sql/query_hash.sql", hash)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(anyhow!(""))?;
    } else {
      sqlx::query_file!("sql/query_best.sql")
        .fetch_optional(&self.pool)
        .await?
        .ok_or(anyhow!(""))?;
    };
    // Check if the block exists (metadata is Some(...))

    // Fetch transactions from DB
    // Internal commands, user commands, and zkapps commands

    // SQL command -> Rosetta/mesh transaction
    // Each command will originate multiple atomic Rosetta/mesh operations

    // Populate the block response from the fetched metadata, if any.

    unimplemented!();
    // Ok(BlockResponse {
    //   block: Some(Box::new(Block::new(
    //     BlockIdentifier::new(0, "".to_string()),
    //     BlockIdentifier::new(0, "".to_string()),
    //     0,
    //     vec![],
    //   ))),
    //   other_transactions: Some(vec![]),
    // })
  }
}
