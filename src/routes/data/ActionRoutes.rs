//Actions

use rocket::{serde::json::Json, Route, State};

use crate::{
    controllers,
    models::ActionModels::{Action, ActionList},
    util::responders::{ApiResult, Jsonstr},
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
async fn get_db_actions() -> ApiResult<Json<Vec<Action>>> {
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
async fn create_action<'a>(
    action: &State<ActionList>,
    action_name: &str,
) -> Jsonstr<'a> {
    return controllers::ActionController::create_action(action, action_name).await;
}

#[put("/<action_name>")]
async fn activate_action<'a>(
    action_list: &State<ActionList>,
    action_name: &str,
) -> Jsonstr<'a> {
    return controllers::ActionController::update_action(action_list, action_name, true).await;
}

#[delete("/<action_name>")]
async fn deactivate_action<'a>(
    action_list: &State<ActionList>,
    action_name: &str,
) -> Jsonstr<'a> {
    return controllers::ActionController::update_action(action_list, action_name, false).await;
}