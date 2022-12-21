use rocket::http::Status;
use surrealdb::sql::Value;

use crate::{models::OrderModels, util::responders::JsonMessage, SurrealRepo};

//Using namespaces to avoid confusiong between model and controller
pub async fn get_orders(db: &SurrealRepo) -> Result<Vec<OrderModels::DBOrder>, Status> {
    let query = db.query("SELECT *, (SELECT * FROM $parent.products[*].model LIMIT 1) as products[*].model FROM orders").await;
    println!("{:?}", query);
    return match query {
        Ok(query) => {
            let order_result = query[0].output().unwrap();
            if let Value::Array(rows) = order_result {
                println!("{0}", rows);
                let orders: Vec<OrderModels::DBOrder> =
                    serde_json::from_value(serde_json::json!(&rows))
                        .expect("Failed to parse order data");
                println!("{:?}", orders);
                Ok(orders)
            } else {
                Err(Status::BadRequest)
            }
        }
        Err(e) => Err(Status::InternalServerError),
    };
}

pub async fn create_order(
    db: &SurrealRepo,
    content: OrderModels::OrderDTO,
) -> Result<JsonMessage, Status> {
    let order = OrderModels::Order::new(content);
    let query = db.create("orders", order, None).await;
    return match query {
        Ok(query) => {
            println!("{:?}", query);
            let result_entry = query[0].output();
            if result_entry.is_ok() {
                println!("{:?}", result_entry);
                Ok(JsonMessage {
                    status_code: Status::Ok,
                    status: true,
                    message: "Successfully created order",
                })
            } else {
                Err(Status::BadRequest)
            }
        }
        Err(e) => Err(Status::InternalServerError),
    };
}

pub async fn update_order<'a>(
    db: &SurrealRepo,
    order_id: &str,
    order: &OrderModels::OrderDTO,
) -> Result<JsonMessage<'a>, Status> {
    let cur_order = format!("orders:{order_id}");
    let query = db.update(&cur_order, order).await;
    return match query {
        Ok(query) => {
            let result = query[0].output();
            if result.is_ok() {
                Ok(JsonMessage {
                    status_code: Status::Ok,
                    status: true,
                    message: "Order successfully updated",
                })
            } else {
                Err(Status::BadRequest)
            }
        }
        Err(e) => Err(Status::InternalServerError),
    };
}
