use rocket::{Route, State, serde::json::Json};
use serde::{Deserialize};
use surrealdb::sql::Value;

use crate::{controllers, SurrealRepo, models};
use crate::util::responders::{ServerMessage, RequestResponse};

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
async fn get_orders(db: &State<SurrealRepo>) -> Result<serde_json::Value, RequestResponse> {
    let orders = controllers::Orders::get_orders(db).await;
    return match orders {
        Ok(order_output) => {
            let order_result = order_output[0].output().unwrap();
            if let Value::Array(rows) = order_result {
                Ok(serde_json::json!(rows))
            } else {
                Err(RequestResponse::BadRequest(ServerMessage::new("Error fetching orders from DB".to_string())))
            }
        }
        Err(_) => Err(RequestResponse::InternalErrorRequest(ServerMessage::new("Error fetching orders".to_string()))),
    };
}

#[post("/", format="json", data="<order>")]
async fn add_orders(db: &State<SurrealRepo>, order: Json<models::Order::OrderDTO>) -> Result<RequestResponse, RequestResponse> {
    let result = controllers::Orders::create_order(db, &order.into_inner()).await;
    return match result {
        Ok(result_output) => {
            let result_entry = result_output[0].output();
            if result_entry.is_ok() {
                Ok(RequestResponse::OKRequest(ServerMessage::new("Successfully created order".to_string())))
            } else {
                Err(RequestResponse::BadRequest(ServerMessage::new("Issue creating order in DB".to_string())))
            }
        },
        Err(_) => Err(RequestResponse::InternalErrorRequest(ServerMessage::new("Error Creating order".to_string()))),
    }
}

#[put("/", format="json", data="<order>")]
async fn update_order(db: &State<SurrealRepo>, order: Json<OrderDetails>) -> Result<RequestResponse, RequestResponse> {
    let update = controllers::Orders::update_order(db, order.order_no, &order.order).await;
    return match update {
        Ok(update_output) => {
            let result = update_output[0].output();
            if (result.is_ok()) {
                Ok(RequestResponse::OKRequest(ServerMessage::new("Successfully related documents".to_string())))
            } else {
                println!("{:?}", result);
                Err(RequestResponse::BadRequest(ServerMessage::new("Issue updating order in DB".to_string())))
            }
        }
        Err(_) => Err(RequestResponse::InternalErrorRequest(ServerMessage::new("Error updating order".to_string()))),
    };
}