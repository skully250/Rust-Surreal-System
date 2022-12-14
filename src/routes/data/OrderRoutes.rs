use rocket::{serde::json::Json, Route, State};
use serde::Deserialize;

use crate::models::OrderModels::DBOrder;
use crate::util::responders::RequestResponse;
use crate::{controllers, models, SurrealRepo};

#[derive(Debug, Deserialize)]
struct OrderDetails {
    order_id: String,
    order: models::OrderModels::OrderDTO,
}

pub fn order_routes() -> Vec<Route> {
    let routes = routes![get_orders, add_order, update_order];
    return routes;
}

#[get("/")]
async fn get_orders(db: &State<SurrealRepo>) -> Result<Json<Vec<DBOrder>>, RequestResponse> {
    let controller_orders = controllers::OrderController::get_orders(db).await;
    return match controller_orders {
        Ok(orders) => Ok(Json(orders)),
        Err(err) => Err(err),
    };
}

#[post("/", format = "json", data = "<order>")]
async fn add_order(
    db: &State<SurrealRepo>,
    order: Json<models::OrderModels::OrderDTO>,
) -> Result<RequestResponse, RequestResponse> {
    return controllers::OrderController::create_order(db, order.into_inner()).await;
}

#[put("/", format = "json", data = "<order>")]
async fn update_order(
    db: &State<SurrealRepo>,
    order: Json<OrderDetails>,
) -> Result<RequestResponse, RequestResponse> {
    return controllers::OrderController::update_order(db, &order.order_id, &order.order).await;
}
