use async_graphql::InputObject;

#[derive(InputObject)]
pub struct ProductInput {
    pub name: String,
    pub price: f64,
    pub stock: i32,
    pub category: String,
}
