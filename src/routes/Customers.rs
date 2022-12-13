use rocket::{Route, State};

use crate::{
    SurrealRepo, util::responders::{RequestResponse, ServerMessage, JsonMessage},
};

pub fn customer_routes() -> Vec<Route> {
    let routes = routes![get_customers, add_customer, update_customer];
    return routes;
}

#[get("/")]
fn get_customers(db: &State<SurrealRepo>) -> Result<serde_json::Value, RequestResponse> {
    return Ok(serde_json::json!("Not yet implemented"));
}

#[post("/")]
fn add_customer(db: &State<SurrealRepo>) -> Result<RequestResponse, RequestResponse> {
    return Ok(RequestResponse::NotImplementedRequest(ServerMessage::new(
        JsonMessage {
            status: false,
            message: "Not yet implemented".to_string(),
        },
    )));
}

#[put("/")]
fn update_customer(db: &State<SurrealRepo>) -> Result<RequestResponse, RequestResponse> {
    return Ok(RequestResponse::NotImplementedRequest(ServerMessage::new(
        JsonMessage {
            status: false,
            message: "Not yet implemented".to_string(),
        },
    )));
}