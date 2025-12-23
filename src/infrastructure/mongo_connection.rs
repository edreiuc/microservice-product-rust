use mongodb::{Client, Database, options::ClientOptions};
use bson::oid::ObjectId;
use std::env;

pub struct MongoConnectionManager {
    db: Database,
}

impl MongoConnectionManager {
    pub async fn new() -> Self {
        let client_uri =
            env::var("MONGO_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());

        let client_options = ClientOptions::parse(&client_uri)
            .await
            .expect("Error al parsear las opciones de MongoDB");

        let client = Client::with_options(client_options)
            .expect("Error al inicializar el cliente de MongoDB");

        let db = client.database("rust_microservice");

        MongoConnectionManager { db }
    }

    pub fn get_all(&self) -> mongodb::Collection<bson::Document> {
        self.db.collection("products")
    }

    pub async fn get_by_id(&self, id: ObjectId) -> Result<Option<bson::Document>, mongodb::error::Error> {
        self.db.collection("products").find_one(bson::doc! { "_id": id }, None).await
    }
}
