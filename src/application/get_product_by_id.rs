use crate::domain::{
    entity::product_entity::Product,
    repository::product_repository::ProductRepository};
use bson::oid::ObjectId;
use std::sync::Arc;

pub struct GetProductByIdUseCase {
    repository: Arc<dyn ProductRepository>,
}

impl GetProductByIdUseCase {
    pub fn new(repository: Arc<dyn ProductRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: ObjectId) -> Result<Product, String> {
        let product = self.repository.get_by_id(id).await;
        match product {
            Ok(product) => Ok(product),
            Err(e) => Err(e.to_string()),
        }
    }
}
