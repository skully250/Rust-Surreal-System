use rocket::{http::Status, serde::json::Json, Route, State};

/*
 * Products will always be created by the order
 * Products will not be created independently but will have indices to act upon
 */
use crate::{controllers, models::ProductModels, util::responders::JsonStatus, SurrealRepo};

pub fn product_routes() -> Vec<Route> {
    let routes = routes![get_models, add_model, action_product];
    return routes;
}

//Models

#[get("/models")]
pub async fn get_models(
    db: &State<SurrealRepo>,
) -> Result<Json<Vec<ProductModels::DBModel>>, Status> {
    let query = controllers::ModelController::get_models(db).await;
    return match query {
        Ok(query) => Ok(Json(query)),
        Err(err) => Err(err),
    };
}

#[post("/models", format = "json", data = "<model>")]
pub async fn add_model(
    db: &State<SurrealRepo>,
    model: Json<ProductModels::ModelDTO>,
) -> Result<JsonStatus, Status> {
    let query = controllers::ModelController::add_model(db, model.into_inner()).await;
    return match query {
        Ok(query) => Ok(query),
        Err(err) => Err(err),
    };
}

//Products

#[post("/products", format = "json", data = "<product>")]
pub async fn action_product<'a>(
    db: &State<SurrealRepo>,
    product: Json<ProductModels::ActionDTO<'a>>,
) -> Result<JsonStatus<'a>, Status> {
    let query = controllers::ProductController::action_product(db, product.into_inner()).await;
    match query {
        Ok(status) => Ok(JsonStatus {
            status_code: status.0,
            status: true,
            message: status.1,
        }),
        Err(e) => Err(e),
    }
}
