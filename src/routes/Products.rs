#[macro_use]
extern crate rocket;

use repository::{SurrealRepo::SurrealRepo};

#[get("/products")]
pub fn get_products(db: &State<SurrealRepo>) -> Result<Json<Vec<Product>>, Status> {
}

#[post("/products")]
pub fn add_products() -> Result<Json<Vec<Product>>, Status> {}
