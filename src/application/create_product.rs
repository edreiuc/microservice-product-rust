use crate::domain::{entity::product_entity::Product,
repository::product_repository::ProductRepository
};
use std::sync::Arc;

pub struct CreateProductUseCase{
    repository: Arc<dyn ProductRepository>,
}

impl CreateProductUseCase {
    pub fn new(repository: Arc<dyn ProductRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, name: String, price: f64, stock: i32, category: String) -> Result<Product, String> {
        let new_product = Product::new(String::new(), name, price, stock, category);

        let created_product = self.repository.create(&new_product)
            .await
            .map_err(|e| e.to_string())?;

        Ok(created_product)
    }
}