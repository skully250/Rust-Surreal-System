use rocket::{http::Status, serde::json::Json, Route, State};

use crate::{
    controllers,
    models::UserModels::{CustomerDTO, DBCustomer},
    util::responders::JsonStatus,
    SurrealRepo,
};

pub fn customer_routes() -> Vec<Route> {
    let routes = routes![get_customers, add_customer, update_customer];
    return routes;
}

#[get("/")]
async fn get_customers(db: &State<SurrealRepo>) -> Result<Json<Vec<DBCustomer>>, Status> {
    let controller_customers = controllers::CustomerController::get_customers(db).await;
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

#[put("/", format = "json", data = "<customer>")]
fn update_customer(
    db: &State<SurrealRepo>,
    customer: Json<CustomerDTO>,
) -> Result<JsonStatus<&str>, Status> {
    return Ok(JsonStatus {
        status_code: Status::NotImplemented,
        status: false,
        message: "Not yet implemented",
    });
}
