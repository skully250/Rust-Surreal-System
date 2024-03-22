use crate::{controllers, models::ProductModels, util::responders::JsonStatus};
use rocket::{http::Status, serde::json::Json, Route};

/*
 * Products will always be created by the order
 * Products will not be created independently but will have indices to act upon
 */

pub fn product_routes() -> Vec<Route> {
    let routes = routes![
        get_models,
        add_model,
        edit_model,
        restore_model,
        delete_model,
    ];
    return routes;
}

//Models

#[get("/models?<fetch_all>")]
async fn get_models(fetch_all: Option<bool>) -> Result<Json<Vec<ProductModels::ProductModel>>, Status> {
    let query = controllers::ModelController::get_models(fetch_all).await;
    return match query {
        Ok(query) => Ok(Json(query)),
        Err(err) => Err(err),
    };
}

#[post("/models", format = "json", data = "<model>")]
async fn add_model<'a>(model: Json<ProductModels::ProductModel>) -> Result<JsonStatus<String>, Status> {
    return controllers::ModelController::add_model(model.into_inner()).await;
}

#[put("/models/<model_id>", format = "json", data = "<model>")]
async fn edit_model(
    model: Json<ProductModels::ProductModel>,
    model_id: String,
) -> Result<JsonStatus<String>, Status> {
    //This may change in future depending on how frontend handles ID's
    let db_name = format!("models:{model_id}");
    return controllers::ModelController::edit_model(model.into_inner(), db_name).await;
}

#[post("/models/<model_id>")]
async fn restore_model(model_id: String) -> Result<JsonStatus<String>, Status> {
    let db_name = format!("models:{model_id}");
    return controllers::ModelController::restore_model(db_name).await;
}

#[delete("/models/<model_id>")]
async fn delete_model(model_id: String) -> Result<JsonStatus<String>, Status> {
    let db_name = format!("models:{model_id}");
    return controllers::ModelController::delete_model(db_name).await;
}