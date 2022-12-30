use rocket::{http::Status, serde::json::Json, Route, State};
use surrealdb::sql::Value;

//POTENTIALLY DEPRECATED
/*
 * Products will always be created by the order
 * so products may not need to be created independently
 */
use crate::{controllers, models, util::responders::JsonStatus, SurrealRepo};

pub fn product_routes() -> Vec<Route> {
    let routes = routes![get_models, add_model, get_products, add_product];
    return routes;
}

#[get("/models")]
pub async fn get_models(
    db: &State<SurrealRepo>,
) -> Result<Json<Vec<models::ProductModels::Model>>, Status> {
    let query = controllers::ModelController::get_models(db).await;
    return match query {
        Ok(query) => Ok(Json(query)),
        Err(err) => Err(err),
    };
}

#[post("/models", format = "json", data = "<model>")]
pub async fn add_model(
    db: &State<SurrealRepo>,
    model: Json<models::ProductModels::ModelDTO>,
) -> Result<JsonStatus, Status> {
    let query = controllers::ModelController::add_model(db, model.into_inner()).await;
    return match query {
        Ok(query) => Ok(query),
        Err(err) => Err(err),
    };
}

#[get("/")]
pub async fn get_products(db: &State<SurrealRepo>) -> Result<serde_json::Value, Status> {
    let query = controllers::ProductController::get_products(db).await;
    return match query {
        Ok(get_output) => {
            let get_result = get_output[0].output().unwrap();
            if let Value::Array(rows) = get_result {
                Ok(serde_json::json!(rows))
            } else {
                Err(Status::BadRequest)
            }
        }
        Err(_) => Err(Status::InternalServerError),
    };
}

#[post("/", format = "json", data = "<product>")]
pub async fn add_product(
    db: &State<SurrealRepo>,
    product: Json<models::ProductModels::ProductDTO>,
) -> Result<JsonStatus, Status> {
    let query = db.create("product", &product.into_inner(), None).await;
    return match query {
        Ok(product_output) => {
            if product_output[0].output().is_ok() {
                Ok(JsonStatus {
                    status_code: Status::Ok,
                    status: true,
                    message: "Successfully added product to DB",
                })
            } else {
                Err(Status::BadRequest)
            }
        }
        Err(_) => Err(Status::InternalServerError),
    };
}
