use rocket::{http::Status, Route, State, serde::json::Json};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Value;

use crate::{controllers, SurrealRepo, models};

#[derive(Debug, Serialize)]
struct ServerMessage<'a> {
    message: &'a str
}

#[derive(Debug, Deserialize)]
struct OrderDetails {
    orderNo: i32,
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
async fn add_orders(db: &State<SurrealRepo>, order: Json<models::Order::OrderDTO>) -> Result<Json<ServerMessage>, Status> {
    let result = db.create("order", order.into_inner()).await;
    return match result {
        Ok(result_output) => {
            let result_entry = result_output[0].output();
            if (result_entry.is_ok()) {
                Ok(Json(ServerMessage{message: "Successfully created order"}))
            } else {
                panic!("Failed to create entry in the DB")
            }
        },
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/", format="json", data="<order>")]
async fn update_order(db: &State<SurrealRepo>, order: Json<OrderDetails>) -> Result<serde_json::Value, Status> {
    db.create("order:32", serde_json::json!(order.order)).await.expect("Issue creating order");
    db.create("user:fae", serde_json::json!("{username: 'fae'}")).await.expect("Issue creating user");
    let update = db.relate("user:fae", "created", "order:32", "time.written = time::now()").await;
    return match update {
        Ok(update_output) => {
            let result = update_output[0].output();
            if (result.is_ok()) {
                let entries: &surrealdb::sql::Value = result.unwrap();
                let v: serde_json::Value = serde_json::json!(entries);
                println!("{}", v);
                println!("{}", v[0]["id"]);
                Ok(serde_json::json!("'message': 'Successfully related documents'"))
            } else {
                println!("{:?}", result);
                panic!("Issue doing db operations")
            }
        }
        Err(_) => Err(Status::InternalServerError),
    };
}