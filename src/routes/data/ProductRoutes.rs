use crate::{
    controllers,
    models::{ActionModels::{ActionList, DBAction}, ProductModels},
    util::responders::JsonStatus,
};
use rocket::{http::Status, serde::json::Json, Route, State};

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
        get_actions,
        create_action,
        update_action,
        delete_action,
        get_db_actions,
    ];
    return routes;
}

//Models

#[get("/models?<fetch_all>")]
async fn get_models(
    fetch_all: Option<bool>,
) -> Result<Json<Vec<ProductModels::DBModel>>, Status> {
    let query = controllers::ModelController::get_models(fetch_all).await;
    return match query {
        Ok(query) => Ok(Json(query)),
        Err(err) => Err(err),
    };
}

#[post("/models", format = "json", data = "<model>")]
async fn add_model<'a>(
    model: Json<ProductModels::ModelDTO>,
) -> Result<JsonStatus<String>, Status> {
    return controllers::ModelController::add_model(model.into_inner()).await;
}

#[put("/models/<model_id>", format = "json", data = "<model>")]
async fn edit_model(
    model: Json<ProductModels::ModelDTO>,
    model_id: String,
) -> Result<JsonStatus<String>, Status> {
    //This may change in future depending on how frontend handles ID's
    let db_name = format!("models:{model_id}");
    return controllers::ModelController::edit_model(model.into_inner(), db_name).await;
}

#[post("/models/<model_id>")]
async fn restore_model(
    model_id: String,
) -> Result<JsonStatus<String>, Status> {
    let db_name = format!("models:{model_id}");
    return controllers::ModelController::restore_model(db_name).await;
}

#[delete("/models/<model_id>")]
async fn delete_model(
    model_id: String,
) -> Result<JsonStatus<String>, Status> {
    let db_name = format!("models:{model_id}");
    return controllers::ModelController::delete_model(db_name).await;
}

//Actions

#[get("/actions/db")]
async fn get_db_actions() -> Result<Json<Vec<DBAction>>, Status> {
    let query = controllers::ActionController::get_actions().await;
    match query {
        Ok(query) => {
            Ok(Json(query))
        },
        Err(err) => {
            Err(err)
        }
    }
}

#[get("/actions")]
async fn get_actions(action_list: &State<ActionList>) -> Json<Vec<String>> {
    let actions = action_list.actions.read().await;
    return Json(actions.to_vec());
}

#[post("/actions/<action_name>")]
async fn create_action(
    action: &State<ActionList>,
    action_name: &str,
) -> Result<JsonStatus<String>, Status> {
    return controllers::ActionController::create_action(action, action_name).await;
}

#[put("/actions/<action_name>")]
async fn update_action<'a>(
    action_list: &State<ActionList>,
    action_name: String,
) -> Result<JsonStatus<&'a str>, Status> {
    return controllers::ActionController::update_action(action_list, action_name, true).await;
}

#[delete("/actions/<action_name>")]
async fn delete_action<'a>(
    action_list: &State<ActionList>,
    action_name: String,
) -> Result<JsonStatus<&'a str>, Status> {
    return controllers::ActionController::update_action(action_list, action_name, false).await;
}
