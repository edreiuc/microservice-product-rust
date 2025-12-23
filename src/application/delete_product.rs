use crate::domain::repository::product_repository::ProductRepository;
use bson::oid::ObjectId;
use std::sync::Arc;

pub struct DeleteProductUseCase {
    repository: Arc<dyn ProductRepository>,
}

impl DeleteProductUseCase {
    pub fn new(repository: Arc<dyn ProductRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: ObjectId) -> Result<(), String> {
        let _ = self.repository.delete(id).await;
        Ok(())
    }
}
