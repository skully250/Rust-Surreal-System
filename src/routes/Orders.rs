use rocket::{serde::json::Json, Route, State};
use serde::Deserialize;
use surrealdb::sql::Value;

use crate::util::responders::{JsonMessage, RequestResponse, ServerMessage};
use crate::{controllers, models, SurrealRepo};

#[derive(Debug, Deserialize)]
struct OrderDetails {
    order_no: u32,
    order: models::Order::OrderDTO,
}

pub fn order_routes() -> Vec<Route> {
    let routes = routes![get_orders, add_order, update_order];
    return routes;
}

#[get("/")]
async fn get_orders(db: &State<SurrealRepo>) -> Result<serde_json::Value, RequestResponse> {
    return controllers::Orders::get_orders(db).await;
}

#[post("/", format = "json", data = "<order>")]
async fn add_order(
    db: &State<SurrealRepo>,
    order: Json<models::Order::OrderDTO>,
) -> Result<RequestResponse, RequestResponse> {
    return controllers::Orders::create_order(db, &order.into_inner()).await;
}

#[put("/", format = "json", data = "<order>")]
async fn update_order(
    db: &State<SurrealRepo>,
    order: Json<OrderDetails>,
) -> Result<RequestResponse, RequestResponse> {
    return controllers::Orders::update_order(db, order.order_no, &order.order).await;
}