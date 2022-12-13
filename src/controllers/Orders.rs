use surrealdb::sql::Value;

use crate::{
    models::Order,
    util::responders::{JsonMessage, RequestResponse, ServerMessage},
    SurrealRepo,
};

//Using namespaces to avoid confusiong between model and controller
pub async fn get_orders(db: &SurrealRepo) -> Result<Vec<Order::DBOrder>, RequestResponse> {
    let query = db.query("SELECT *, (SELECT * FROM $parent.products[*].model LIMIT 1) as products[*].model FROM orders").await;
    println!("{:?}", query);
    return match query {
        Ok(query) => {
            let order_result = query[0].output().unwrap();
            if let Value::Array(rows) = order_result {
                println!("{0}", rows);
                let orders: Vec<Order::DBOrder> = serde_json::from_value(
                    serde_json::json!(&rows)).expect("Failed to parse order data");
                println!("{:?}", orders);
                Ok(orders)
            } else {
                Err(RequestResponse::BadRequest(ServerMessage::new(
                    JsonMessage {
                        status: false,
                        message: "Error while fetching order".to_string(),
                    },
                )))
            }
        }
        Err(e) => Err(RequestResponse::InternalErrorRequest(ServerMessage::new(
            JsonMessage {
                status: false,
                message: e.to_string(),
            },
        ))),
    };
}

pub async fn create_order(
    db: &SurrealRepo,
    content: Order::OrderDTO,
) -> Result<RequestResponse, RequestResponse> {
    let order = Order::Order::new(content);
    let query = db.create("orders", order, None).await;
    return match query {
        Ok(query) => {
            println!("{:?}", query);
            let result_entry = query[0].output();
            if result_entry.is_ok() {
                println!("{:?}", result_entry);
                Ok(RequestResponse::OKRequest(ServerMessage::new(
                    JsonMessage {
                        status: true,
                        message: "Successfully created Order".to_string(),
                    },
                )))
            } else {
                Err(RequestResponse::BadRequest(ServerMessage::new(
                    JsonMessage {
                        status: false,
                        message: "Issue creating order in DB".to_string(),
                    },
                )))
            }
        }
        Err(e) => Err(RequestResponse::InternalErrorRequest(ServerMessage::new(
            JsonMessage {
                status: false,
                message: "Error creating order query".to_string(),
            },
        ))),
    };
}

pub async fn update_order(
    db: &SurrealRepo,
    order_id: &str,
    order: &Order::OrderDTO,
) -> Result<RequestResponse, RequestResponse> {
    let cur_order = format!("orders:{order_id}");
    let query = db.update(&cur_order, order).await;
    return match query {
        Ok(query) => {
            let result = query[0].output();
            if result.is_ok() {
                Ok(RequestResponse::OKRequest(ServerMessage::new(
                    JsonMessage {
                        status: true,
                        message: "Order successfully updated".to_string(),
                    },
                )))
            } else {
                Err(RequestResponse::BadRequest(ServerMessage::new(
                    JsonMessage {
                        status: false,
                        message: "Error updating Order in DB".to_string(),
                    },
                )))
            }
        }
        Err(e) => Err(RequestResponse::InternalErrorRequest(ServerMessage::new(
            JsonMessage {
                status: false,
                message: "Error updating order query".to_string(),
            },
        ))),
    };
}
