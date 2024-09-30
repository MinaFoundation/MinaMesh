pub use mesh::models::{SearchTransactionsRequest, SearchTransactionsResponse};

pub use crate::{MinaMesh, MinaMeshError};

impl MinaMesh {
  pub async fn search_transactions(
    &self,
    _req: SearchTransactionsRequest,
  ) -> Result<SearchTransactionsResponse, MinaMeshError> {
    unimplemented!()
  }
}
