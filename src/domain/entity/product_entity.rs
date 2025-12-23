use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use async_graphql::SimpleObject;

#[derive(Serialize, Deserialize, Debug, Clone, SimpleObject)]
pub struct Product {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub price: f64,
    pub stock: i32,
    pub category: String,
}

impl Product {
    pub fn new(_id: String, name: String, price: f64, stock: i32, category: String) -> Self {
        Self {
            id: None,
            name,
            price,
            stock,
            category,
        }
    }

    pub fn default() -> Self {
        Self {
            id: None,
            name: "Caja equis".to_string(),
            price: 4500.0,
            stock: 300,
            category: "entretenimiento".to_string(),
        }
    }

    pub fn update(&mut self, name: String, price: f64, stock: i32, category: String) {
        self.name = name;
        self.price = price;
        self.stock = stock;
        self.category = category;
    }
    
}
