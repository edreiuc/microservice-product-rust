use crate::domain::{
    entity::product_entity::Product,
    repository::product_repository::ProductRepository};

use std::sync::Arc;

pub struct GetProductUseCase {
    repository: Arc<dyn ProductRepository>,
}

impl GetProductUseCase {
    pub fn new(repository: Arc<dyn ProductRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self) -> Result<Vec<Product>, String> {
        let products = self.repository.find_all().await;
        products.map_err(|e| e.to_string())
    }
}