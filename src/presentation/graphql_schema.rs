use async_graphql::{Context, Object};
use crate::application::{get_product::GetProductUseCase, get_product_by_id::GetProductByIdUseCase, create_product::CreateProductUseCase};
use crate::domain::entity::product_entity::Product;
use std::sync::Arc;
use bson::oid::ObjectId;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn health_check(&self) -> &str {
        "¡El servicio está vivo! 🚀"
    }

    async fn products(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Product>> {
        let use_case = ctx.data::<Arc<GetProductUseCase>>()?;
        
        let products = use_case.execute().await;
        
        match products {
            Ok(data) => Ok(data),
            Err(e) => Err(async_graphql::Error::new(e)),
        }
    }

    async fn product_by_id(&self, ctx: &Context<'_>, id: ObjectId) -> async_graphql::Result<Product> {
        let use_case = ctx.data::<Arc<GetProductByIdUseCase>>()?;
        
        let product = use_case.execute(id).await;
        
        match product {
            Ok(data) => Ok(data),
            Err(e) => Err(async_graphql::Error::new(e)),
        }
    }
}


pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_product(
        &self,
        ctx: &Context<'_>,
        name: String,
        price: f64,
        stock: i32,
        category: String,
    ) -> async_graphql::Result<Product> {
        
        
        let use_case = ctx.data::<Arc<CreateProductUseCase>>()?;
        
        let product = use_case.execute(name, price, stock, category).await;
        
        match product {
            Ok(data) => Ok(data),
            Err(e) => Err(async_graphql::Error::new(e)),
        }
    }
}