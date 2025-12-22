use axum::{extract::Extension, response::{Html, IntoResponse}, routing::get, Router};
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use std::net::SocketAddr;
use std::sync::Arc;
use products_service::domain::repository::product_repository::ProductRepository;
use products_service::infrastructure::{mongo_connection::MongoConnectionManager, mongo_product_repository::MongoProductRepository};
use products_service::application::{get_product::GetProductUseCase, create_product::CreateProductUseCase};
use products_service::presentation::graphql_schema::{QueryRoot, MutationRoot};

type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

async fn graphql_playground() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

async fn graphql_handler(
    schema: Extension<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    println!("🚀 Iniciando servidor...");
    let mongo_manager = MongoConnectionManager::new().await;
    
    let product_repository = MongoProductRepository::new(mongo_manager);
    
    let product_repository: Arc<dyn ProductRepository> = Arc::new(product_repository);

    let create_product_use_case = Arc::new(CreateProductUseCase::new(product_repository.clone()));
    let get_products_use_case = Arc::new(GetProductUseCase::new(product_repository.clone()));

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(create_product_use_case)
        .data(get_products_use_case)
        .finish();

    let app = Router::new()
        .route("/", get(graphql_playground))
        .route("/graphql", get(graphql_handler).post(graphql_handler))
        .layer(Extension(schema));

    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    println!("✅ Servidor escuchando en http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
