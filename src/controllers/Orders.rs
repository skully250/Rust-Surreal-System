use surrealdb::{sql::Value, Response};

use crate::{
    models,
    util::responders::{JsonMessage, RequestResponse, ServerMessage},
    SurrealRepo,
};

//Using namespaces to avoid confusiong between model and controller
pub async fn get_orders(db: &SurrealRepo) -> Result<serde_json::Value, RequestResponse> {
    let query = db.query("SELECT * FROM orders").await;
    return match query {
        Ok(query) => {
            let order_result = query[0].output().unwrap();
            if let Value::Array(rows) = order_result {
                Ok(serde_json::json!(rows))
            } else {
                Err(RequestResponse::BadRequest(ServerMessage::new(
                    JsonMessage {
                        status: false,
                        message: "Error fetching orders from DB".to_string(),
                    },
                )))
            }
        }
        Err(e) => Err(RequestResponse::InternalErrorRequest(ServerMessage::new(
            JsonMessage {
                status: false,
                message: "Error while fetching order query".to_string(),
            },
        ))),
    };
}

pub async fn create_order(
    db: &SurrealRepo,
    content: &models::Order::OrderDTO,
) -> Result<RequestResponse, RequestResponse> {
    let query = db.create("orders", content).await;
    return match query {
        Ok(query) => {
            let result_entry = query[0].output();
            if result_entry.is_ok() {
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
        },
        Err(e) => Err(RequestResponse::InternalErrorRequest(ServerMessage::new(
            JsonMessage {
                status: false,
                message: "Error creating order query".to_string()
            }
        )))
    };
}

pub async fn update_order(
    db: &SurrealRepo,
    order_no: u32,
    order: &models::Order::OrderDTO,
) -> Result<RequestResponse, RequestResponse> {
    let cur_order = format!("orders:{order_no}");
    let query = db.update(&cur_order, order).await;
    return match query {
        Ok(query) => {
            let result = query[0].output();
            if result.is_ok() {
                Ok(RequestResponse::OKRequest(ServerMessage::new(
                    JsonMessage {
                        status: true,
                        message: "Order successfully updated".to_string()
                    }
                )))
            } else {
                Err(RequestResponse::BadRequest(ServerMessage::new(
                    JsonMessage {
                        status: false,
                        message: "Error updating Order in DB".to_string()
                    }
                )))
            }
        },
        Err(e) => Err(RequestResponse::InternalErrorRequest(ServerMessage::new(
            JsonMessage {
                status: false,
                message: "Error updating order query".to_string()
            }
        )))
    };
}
