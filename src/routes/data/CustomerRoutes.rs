use rocket::{http::Status, serde::json::Json, Route, State};

use crate::{
    controllers,
    models::UserModels::{CustomerDTO, DBCustomer},
    util::responders::JsonStatus,
    SurrealRepo,
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
async fn get_customers(db: &State<SurrealRepo>, removed: Option<bool>) -> Result<Json<Vec<DBCustomer>>, Status> {
    let controller_customers = controllers::CustomerController::get_customers(db, removed).await;
    return match controller_customers {
        Ok(customers) => Ok(Json(customers)),
        Err(err) => Err(err),
    };
}

#[post("/", format = "json", data = "<customer>")]
async fn add_customer(
    db: &State<SurrealRepo>,
    customer: Json<CustomerDTO>,
) -> Result<JsonStatus<&str>, Status> {
    return controllers::CustomerController::add_customer(db, customer.into_inner()).await;
}

#[put("/<customer_id>", format = "json", data = "<customer>")]
async fn update_customer(
    db: &State<SurrealRepo>,
    customer: Json<CustomerDTO>,
    customer_id: String,
) -> Result<JsonStatus<&str>, Status> {
    return controllers::CustomerController::edit_customer(db, customer.into_inner(), customer_id)
        .await;
}

#[delete("/<customer_id>")]
async fn delete_customer(
    db: &State<SurrealRepo>,
    customer_id: String,
) -> Result<JsonStatus<&str>, Status> {
    return controllers::CustomerController::remove_customer(db, customer_id).await;
}
