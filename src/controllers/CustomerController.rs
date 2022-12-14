use surrealdb::sql::Value;

use crate::{
    models::UserModels::{CustomerDTO, DBCustomer},
    util::responders::{JsonMessage, RequestResponse, ServerMessage},
    SurrealRepo,
};

pub async fn get_customers(db: &SurrealRepo) -> Result<Vec<DBCustomer>, RequestResponse> {
    let customers = db.find(None, "Customers").await;
    return match customers {
        Ok(query) => {
            let query_result = query[0].output().unwrap();
            if let Value::Array(rows) = query_result {
                let customers: Vec<DBCustomer> = serde_json::from_value(serde_json::json!(&rows))
                    .expect("Failed to parse customer data");
                Ok(customers)
            } else {
                Err(RequestResponse::BadRequest(ServerMessage::new(
                    JsonMessage {
                        status: false,
                        message: "Error while fetching customer data".to_string(),
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

pub async fn add_customer(
    db: &SurrealRepo,
    customer: CustomerDTO,
) -> Result<RequestResponse, RequestResponse> {
    let query = db.create("customers", customer, None).await;
    return match query {
        Ok(query) => {
            let result_entry = query[0].output();
            if result_entry.is_ok() {
                Ok(RequestResponse::OKRequest(ServerMessage::new(
                    JsonMessage {
                        status: true,
                        message: "Successfully created customer".to_string(),
                    },
                )))
            } else {
                Err(RequestResponse::BadRequest(ServerMessage::new(
                    JsonMessage {
                        status: false,
                        message: "Issue creating customer in DB".to_string(),
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