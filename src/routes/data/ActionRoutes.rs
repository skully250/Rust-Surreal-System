//Actions

use rocket::{http::Status, serde::json::Json, Route, State};

use crate::{
    controllers,
    models::ActionModels::{ActionList, Action},
    util::responders::JsonStatus,
};

pub fn action_routes() -> Vec<Route> {
    let routes = routes![
        get_actions,
        create_action,
        activate_action,
        deactivate_action,
        get_db_actions,
    ];
    return routes;
}

#[get("/db")]
async fn get_db_actions() -> Result<Json<Vec<Action>>, Status> {
    let query = controllers::ActionController::get_actions().await;
    match query {
        Ok(query) => Ok(Json(query)),
        Err(err) => Err(err),
    }
}

#[get("/")]
async fn get_actions(action_list: &State<ActionList>) -> Json<Vec<String>> {
    let actions = action_list.actions.read().await;
    return Json(actions.to_vec());
}

#[post("/<action_name>")]
async fn create_action(
    action: &State<ActionList>,
    action_name: &str,
) -> Result<JsonStatus<String>, Status> {
    return controllers::ActionController::create_action(action, action_name).await;
}

#[put("/<action_name>")]
async fn activate_action(
    action_list: &State<ActionList>,
    action_name: &str,
) -> Result<JsonStatus<String>, Status> {
    return controllers::ActionController::update_action(action_list, action_name, true).await;
}

#[delete("/<action_name>")]
async fn deactivate_action(
    action_list: &State<ActionList>,
    action_name: &str,
) -> Result<JsonStatus<String>, Status> {
    return controllers::ActionController::update_action(action_list, action_name, false).await;
}