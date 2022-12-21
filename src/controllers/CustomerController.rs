use rocket::http::Status;
use surrealdb::sql::Value;

use crate::{
    models::UserModels::{CustomerDTO, DBCustomer},
    util::responders::JsonMessage,
    SurrealRepo,
};

pub async fn get_customers(db: &SurrealRepo) -> Result<Vec<DBCustomer>, Status> {
    let customers = db.find(None, "Customers").await;
    return match customers {
        Ok(query) => {
            let query_result = query[0].output().unwrap();
            if let Value::Array(rows) = query_result {
                let customers: Vec<DBCustomer> = serde_json::from_value(serde_json::json!(&rows))
                    .expect("Failed to parse customer data");
                Ok(customers)
            } else {
                Err(Status::BadRequest)
            }
        }
        Err(e) => Err(Status::InternalServerError),
    };
}

pub async fn add_customer(db: &SurrealRepo, customer: CustomerDTO) -> Result<JsonMessage, Status> {
    let query = db.create("customers", customer, None).await;
    return match query {
        Ok(query) => {
            let result_entry = query[0].output();
            if result_entry.is_ok() {
                Ok(JsonMessage {
                    status_code: Status::Ok,
                    status: true,
                    message: "Successfully created customer",
                })
            } else {
                Err(Status::BadRequest)
            }
        }
        Err(e) => Err(Status::InternalServerError),
    };
}
