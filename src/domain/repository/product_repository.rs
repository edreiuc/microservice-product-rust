use crate::domain::entity::product_entity::Product;
use async_trait::async_trait;
use bson::oid::ObjectId;
use std::error::Error;

pub type ProductError = Box<dyn Error + Send + Sync>;

#[async_trait]
pub trait ProductRepository: Send + Sync {
    async fn create(&self, product: &Product) -> Result<Product, ProductError>;
    async fn update(&self, id: ObjectId, product: Product) -> Result<Product, ProductError>;
    async fn delete(&self, id: ObjectId) -> Result<(), ProductError>;
    async fn find_all(&self) -> Result<Vec<Product>, ProductError>;
    async fn get_by_id(&self, id: ObjectId) -> Result<Product, ProductError>;
}
