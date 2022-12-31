use rocket::http::Status;
use rocket::{serde::json::Json, Route, State};
use serde::Deserialize;

use crate::models::AuthModels::AuthUser;
use crate::models::OrderModels::DBOrder;
use crate::util::responders::JsonStatus;
use crate::{controllers, models, SurrealRepo};

#[derive(Debug, Deserialize)]
struct OrderDetails {
    order_id: String,
    order: models::OrderModels::OrderDTO,
}

pub fn order_routes() -> Vec<Route> {
    let routes = routes![get_orders, add_order, update_order, get_orders_by_user];
    return routes;
}

#[get("/")]
async fn get_orders(db: &State<SurrealRepo>) -> Result<Json<Vec<DBOrder>>, Status> {
    let controller_orders = controllers::OrderController::get_orders(db).await;
    return match controller_orders {
        Ok(orders) => Ok(Json(orders)),
        Err(err) => Err(err),
    };
}

#[get("/?<customer>")]
async fn get_orders_by_customer(db: &State<SurrealRepo>, customer: &str) -> Status {
    return Status::NotImplemented;
}

#[get("/?<user>")]
async fn get_orders_by_user<'a>(
    db: &State<SurrealRepo>,
    user: &str,
) -> Result<Json<Vec<DBOrder>>, JsonStatus<'a>> {
    let related_orders = controllers::OrderController::get_orders_by_user(db, user).await;
    return match related_orders {
        Ok(orders) => Ok(Json(orders)),
        Err(err) => Err(JsonStatus {
            status_code: err.0,
            status: false,
            message: err.1,
        }),
    };
}

#[post("/", format = "json", data = "<order>")]
async fn add_order(
    db: &State<SurrealRepo>,
    user: AuthUser,
    order: Json<models::OrderModels::OrderDTO>,
) -> Result<JsonStatus, Status> {
    return controllers::OrderController::create_order(db, order.into_inner(), &user).await;
}

#[put("/", format = "json", data = "<order>")]
async fn update_order(
    db: &State<SurrealRepo>,
    order: Json<OrderDetails>,
) -> Result<JsonStatus, Status> {
    return controllers::OrderController::update_order(db, &order.order_id, &order.order).await;
}
