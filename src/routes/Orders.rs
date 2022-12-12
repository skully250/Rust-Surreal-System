use rocket::{http::Status, Route, State, serde::json::Json};
use serde::{Deserialize};
use surrealdb::sql::Value;

use crate::{controllers, SurrealRepo, models, util::responders};

#[derive(Debug, Deserialize)]
struct OrderDetails {
    order_no: u32,
    order: models::Order::OrderDTO
}

pub fn order_routes() -> Vec<Route> {
    let routes = routes![get_orders, add_orders, update_order];
    return routes;
}

#[get("/")]
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

#[post("/", format="json", data="<order>")]
async fn add_orders(db: &State<SurrealRepo>, order: Json<models::Order::OrderDTO>) -> Result<Json<responders::ServerMessage>, Status> {
    let result = controllers::Orders::create_order(db, &order.into_inner()).await;
    return match result {
        Ok(result_output) => {
            let result_entry = result_output[0].output();
            if result_entry.is_ok() {
                Ok(Json(responders::ServerMessage{message: "Successfully created order"}))
            } else {
                Err(Status::BadRequest)
            }
        },
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/", format="json", data="<order>")]
async fn update_order(db: &State<SurrealRepo>, order: Json<OrderDetails>) -> Result<Json<responders::ServerMessage>, Status> {
    let update = controllers::Orders::update_order(db, order.order_no, &order.order).await;
    return match update {
        Ok(update_output) => {
            let result = update_output[0].output();
            if (result.is_ok()) {
                Ok(Json(responders::ServerMessage{ message: "Successfully related documents" }))
            } else {
                println!("{:?}", result);
                panic!("Issue doing db operations")
            }
        }
        Err(_) => Err(Status::InternalServerError),
    };
}