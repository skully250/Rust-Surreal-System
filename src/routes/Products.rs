use rocket::{State, http::Status};

use crate::{SurrealRepo};

#[get("/products")]
pub fn get_products(db: &State<SurrealRepo>) -> Result<serde_json::Value, Status> {
    return Ok(serde_json::json!("Not yet implemented"));
}

#[post("/products")]
pub fn add_products() -> Result<serde_json::Value, Status> {
    return Ok(serde_json::json!("Not yet implemented"))
}
