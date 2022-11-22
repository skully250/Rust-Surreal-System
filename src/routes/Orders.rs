use rocket::{http::Status, Route, State, serde::json};
use surrealdb::sql::Value;

use crate::{controllers, SurrealRepo, models};

pub fn orderRoutes() -> Vec<Route> {
    let routes = routes![get_orders, add_orders];
    return routes;
}

#[get("/orders")]
async fn get_orders(db: &State<SurrealRepo>) -> Result<serde_json::Value, Status> {
    let orders = controllers::Orders::get_orders(db).await;
    return match orders {
        Ok(order_output) => {
            let order_result = order_output[0].output().unwrap();
            if let Value::Array(rows) = order_result {
                Ok(serde_json::json!(rows))
            } else {
                panic!("Issue formatting JSON for DB Data")
            }
        }
        Err(_) => Err(Status::InternalServerError),
    };
}

#[post("/orders", format="json", data="<order>")]
fn add_orders(order: Json<models::Order::OrderDTO>) -> Result<serde_json::Value, Status> {
    return Ok(serde_json::json!("Not Yet Implemented"))
}