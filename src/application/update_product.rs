use crate::domain::{
    entity::product_entity::Product, repository::product_repository::ProductRepository,
};
use bson::oid::ObjectId;
use std::sync::Arc;

pub struct UpdateProductUseCase {
    repository: Arc<dyn ProductRepository>,
}

impl UpdateProductUseCase {
    pub fn new(repository: Arc<dyn ProductRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        id: ObjectId,
        name: String,
        price: f64,
        stock: i32,
        category: String,
    ) -> Result<(), String> {
        let product = Product {
            id: Some(id),
            name,
            price,
            stock,
            category,
        };
        let _ = self.repository.update(id, product).await;
        Ok(())
    }
}
