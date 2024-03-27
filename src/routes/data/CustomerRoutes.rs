use rocket::{http::Status, serde::json::Json, Route};

use crate::{
    controllers,
    util::responders::JsonStatus,
};

pub fn customer_routes() -> Vec<Route> {
    let routes = routes![
        get_customers,
        add_customer,
        update_customer,
        delete_customer
    ];
    return routes;
}

#[get("/?<removed>")]
async fn get_customers(removed: Option<bool>) -> Result<Json<Vec<DBCustomer>>, Status> {
    let controller_customers = controllers::CustomerController::get_customers(db, removed).await;
    return match controller_customers {
        Ok(customers) => Ok(Json(customers)),
        Err(err) => Err(err),
    };
}

#[post("/", format = "json", data = "<customer>")]
async fn add_customer(customer: Json<CustomerDTO>) -> Result<JsonStatus<&str>, Status> {
    return controllers::CustomerController::add_customer(db, customer.into_inner()).await;
}

#[put("/<customer_id>", format = "json", data = "<customer>")]
async fn update_customer(
    customer: Json<CustomerDTO>,
    customer_id: String,
) -> Result<JsonStatus<&str>, Status> {
    return controllers::CustomerController::edit_customer(db, customer.into_inner(), customer_id)
        .await;
}

#[delete("/<customer_id>")]
async fn delete_customer(customer_id: String) -> Result<JsonStatus<&str>, Status> {
    return controllers::CustomerController::remove_customer(db, customer_id).await;
}
