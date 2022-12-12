use surrealdb::Response;

use crate::{models, SurrealRepo};

pub async fn get_products(db: &SurrealRepo) -> Result<Vec<Response>, surrealdb::Error> {
    let query = db.find(None, "products").await;
    return match query {
        Ok(query) => Ok(query),
        Err(e) => panic!("DB Could not get products - Error: {:?}", e),
    };
}

pub async fn create_product(
    db: &SurrealRepo,
    content: &models::Product::ProductDTO,
) -> Result<Vec<Response>, surrealdb::Error> {
    let query = db.create("products", content).await;
    return match query {
        Ok(query) => Ok(query),
        Err(e) => panic!("DB Could not create product - Error: {:?}", e),
    };
}