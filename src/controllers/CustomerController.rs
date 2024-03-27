use rocket::http::Status;

use crate::{
    models::UserModels::Customer,
    repository::SurrealRepo,
    util::responders::{ApiResult, JsonStatus, Jsonstr},
};

pub async fn get_customers(find_all: Option<bool>) -> ApiResult<Vec<Customer>> {
    let customers: Result<Vec<Customer>, surrealdb::Error> =
        match find_all.is_some_and(|find| find == true) {
            true => SurrealRepo::find("customers", "*").await,
            false => Customer::find_removed().await,
        };

    return match customers {
        Ok(query) => Ok(query),
        Err(_) => Err(Status::InternalServerError),
    };
}

pub async fn add_customer<'a>(customer: Customer) -> Jsonstr<'a> {
    let query = SurrealRepo::create("customers", customer).await;
    return match query {
        Ok(query) => {
                Ok(JsonStatus::success("Successfully created customer"))
        }
        Err(_) => Err(Status::InternalServerError),
    };
}

pub async fn edit_customer(customer: Customer, customer_id: &str) -> Jsonstr {
    let query = SurrealRepo::update("customers", &customer_id, customer).await;
    return match query {
        Ok(query) => {
            if query.is_some() {
                Ok(JsonStatus::success("Successfully updated customer"))
            } else {
                Ok(JsonStatus::custom(Status::NotFound, false, "Customer does not exist"))
            }
        }
        Err(_) => Err(Status::InternalServerError),
    };
}

pub async fn remove_customer(customer_id: &str) -> Jsonstr {
    let query = Customer::remove_customer(customer_id).await;
    return match query {
        Ok(_) => Ok(JsonStatus::success("Customer removed successfully")),
        Err(_) => Err(Status::InternalServerError),
    };
}
