use rocket::{State, http::Status};

use crate::{SurrealRepo, models};

#[get("/users")]
pub fn get_users(db: &State<SurrealRepo>) -> Result<serde_json::Value, Status> {
    return Ok(serde_json::json!("Not yet implemented"))
}

#[post("/users")]
pub fn add_users() -> Result<serde_json::Value, Status> {
    return Ok(serde_json::json!("Not yet implemented"))
}