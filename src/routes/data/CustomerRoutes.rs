use rocket::{serde::json::Json, Route};

use crate::{
    controllers,
    models::UserModels::Customer,
    util::responders::{ApiResult, Jsonstr},
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

#[get("/?<find_all>")]
async fn get_customers(find_all: Option<bool>) -> ApiResult<Json<Vec<Customer>>> {
    let controller_customers = controllers::CustomerController::get_customers(find_all).await;
    return match controller_customers {
        Ok(customers) => Ok(Json(customers)),
        Err(err) => Err(err),
    };
}

#[post("/", format = "json", data = "<customer>")]
async fn add_customer<'a>(customer: Json<Customer>) -> Jsonstr<'a> {
    return controllers::CustomerController::add_customer(customer.into_inner()).await;
}

#[put("/<customer_id>", format = "json", data = "<customer>")]
async fn update_customer(customer: Json<Customer>, customer_id: &str) -> Jsonstr {
    return controllers::CustomerController::edit_customer(customer.into_inner(), customer_id)
        .await;
}

#[delete("/<customer_id>")]
async fn delete_customer(customer_id: &str) -> Jsonstr {
    return controllers::CustomerController::remove_customer(customer_id).await;
}
