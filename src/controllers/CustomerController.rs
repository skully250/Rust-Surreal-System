use rocket::http::Status;
use surrealdb::sql::Value;

use crate::{
    models::UserModels::{CustomerDTO, DBCustomer},
    util::responders::JsonStatus,
    SurrealRepo,
};

pub async fn get_customers(
    db: &SurrealRepo,
    mut find_all: Option<bool>,
) -> Result<Vec<DBCustomer>, Status> {
    if find_all.is_none() {
        find_all = Some(false)
    };
    let customers = match find_all.unwrap() {
        true => db.find(None, "customers").await,
        false => db.find_where(None, "customers", "removed != true").await,
    };

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
        Err(_) => Err(Status::InternalServerError),
    };
}

pub async fn add_customer(
    db: &SurrealRepo,
    customer: CustomerDTO,
) -> Result<JsonStatus<&str>, Status> {
    let query = db.create("customers", customer, None).await;
    return match query {
        Ok(query) => {
            let result_entry = query[0].output();
            if result_entry.is_ok() {
                Ok(JsonStatus {
                    status_code: Status::Ok,
                    status: true,
                    message: "Successfully created customer",
                })
            } else {
                Err(Status::BadRequest)
            }
        }
        Err(_) => Err(Status::InternalServerError),
    };
}

pub async fn edit_customer(
    db: &SurrealRepo,
    customer: CustomerDTO,
    customer_id: String,
) -> Result<JsonStatus<&str>, Status> {
    let query = db.update(&customer_id, customer).await;
    return match query {
        Ok(query) => {
            let empty_query = query[0].output().unwrap().first().is_none();
            if !empty_query {
                Ok(JsonStatus {
                    status_code: Status::Ok,
                    status: true,
                    message: "Successfully updated customer",
                })
            } else {
                Ok(JsonStatus {
                    status_code: Status::NotFound,
                    status: false,
                    message: "Customer doesnt exist",
                })
            }
        }
        Err(_) => Err(Status::InternalServerError),
    };
}

pub async fn remove_customer(
    db: &SurrealRepo,
    customer_id: String,
) -> Result<JsonStatus<&str>, Status> {
    let query_string = format!("UPDATE {0} SET removed = true", customer_id);
    let query = db.query(&query_string).await;
    return match query {
        Ok(query) => {
            let empty_query = query[0].output().unwrap().first().is_none();
            if !empty_query {
                Ok(JsonStatus {
                    status_code: Status::Ok,
                    status: true,
                    message: "Customer removed successfully",
                })
            } else {
                Ok(JsonStatus {
                    status_code: Status::NotFound,
                    status: false,
                    message: "Customer doesnt exist",
                })
            }
        }
        Err(_) => Err(Status::InternalServerError),
    };
}
