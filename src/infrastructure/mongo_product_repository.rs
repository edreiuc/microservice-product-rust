use super::mongo_connection::MongoConnectionManager;
use crate::domain::{
    entity::product_entity::Product,
    repository::product_repository::{ProductError, ProductRepository},
};
use async_trait::async_trait;
use bson::{oid::ObjectId, to_bson};
use futures::stream::StreamExt;

pub struct MongoProductRepository {
    manager: MongoConnectionManager,
}

impl MongoProductRepository {
    pub fn new(manager: MongoConnectionManager) -> Self {
        Self { manager }
    }
}

#[async_trait]
impl ProductRepository for MongoProductRepository {
    async fn create(&self, product: &Product) -> Result<Product, ProductError> {
        let collection = self.manager.get_all();
        let serialized_product = to_bson(product)?;
        let document = serialized_product
            .as_document()
            .ok_or("Error convirtiendo a documento BSON")?;

        let result = collection.insert_one(document.clone(), None).await?;

        let id = result
            .inserted_id
            .as_object_id()
            .ok_or("No se pudo obtener el ID insertado")?;

        // Return the product with the new ID
        let mut created_product = product.clone();
        created_product.id = Some(id);

        Ok(created_product)
    }

    async fn update(&self, id: ObjectId, product: Product) -> Result<Product, ProductError> {
        let collection = self.manager.get_all();
        let filter = bson::doc! { "_id": id };

        let update = bson::doc! {
            "$set": {
                "name": &product.name,
                "price": product.price,
                "stock": product.stock,
                "category": &product.category,
            }
        };

        collection.update_one(filter, update, None).await?;

        let mut updated_product = product;
        updated_product.id = Some(id);

        Ok(updated_product)
    }

    async fn get_by_id(&self, id: ObjectId) -> Result<Product, ProductError> {
        let result = self.manager.get_by_id(id).await?;

        let document = result.ok_or_else(|| {
            Box::<dyn std::error::Error + Send + Sync>::from("No se encontró el producto")
        })?;

        let product: Product = bson::from_document(document)?;

        Ok(product)
    }

    async fn delete(&self, id: ObjectId) -> Result<(), ProductError> {
        self.manager.delete(id).await?;
        Ok(())
    }

    async fn find_all(&self) -> Result<Vec<Product>, ProductError> {
        let collection = self.manager.get_all();
        let mut cursor = collection.find(None, None).await?;
        let mut products = Vec::new();

        while let Some(result) = cursor.next().await {
            let doc = result?;
            let product: Product = bson::from_document(doc)?;
            products.push(product);
        }

        Ok(products)
    }
}
