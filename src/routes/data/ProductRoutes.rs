use rocket::{http::Status, serde::json::Json, Route, State, futures::SinkExt};

/*
 * Products will always be created by the order
 * Products will not be created independently but will have indices to act upon
 */
use crate::{
    controllers,
    models::ProductModels::{self, ActionList, DBAction},
    util::responders::JsonStatus,
    SurrealRepo,
};

pub fn product_routes() -> Vec<Route> {
    let routes = routes![
        get_models,
        add_model,
        action_product,
        create_action,
        get_actions,
        get_managed_actions,
        get_named_managed
    ];
    return routes;
}

fn action_exists(action_name: &String, actions: &State<ActionList>) -> bool {
    let action_list = actions.actions.read().unwrap();
    action_list.contains(action_name)
}

//Models

#[get("/models")]
async fn get_models(
    db: &State<SurrealRepo>,
) -> Result<Json<Vec<ProductModels::DBModel>>, Status> {
    let query = controllers::ModelController::get_models(db).await;
    return match query {
        Ok(query) => Ok(Json(query)),
        Err(err) => Err(err),
    };
}

#[post("/models", format = "json", data = "<model>")]
async fn add_model<'a>(
    db: &State<SurrealRepo>,
    model: Json<ProductModels::ModelDTO>,
) -> Result<JsonStatus<&'a str>, Status> {
    let query = controllers::ModelController::add_model(db, model.into_inner()).await;
    return match query {
        Ok(query) => Ok(query),
        Err(err) => Err(err),
    };
}

#[get("/actions")]
async fn get_actions(db: &State<SurrealRepo>) -> Result<Json<Vec<DBAction>>, Status> {
    let action_result = controllers::ActionController::get_actions(db).await;
    return match action_result {
        Ok(actions) => Ok(Json(actions)),
        Err(e) => Err(e),
    };
}

#[get("/managed")]
fn get_managed_actions(
    actions: &State<ProductModels::ActionList>,
) -> JsonStatus<&ProductModels::ActionList> {
    JsonStatus {
        status_code: Status::Ok,
        status: true,
        message: actions.inner(),
    }
}

#[post("/managed", data="<action_name>")]
fn get_named_managed(actions: &State<ProductModels::ActionList>, action_name: String) -> JsonStatus<String> {
    let actions = actions.actions.read().unwrap();
    let entry = actions.contains(&action_name);
    JsonStatus {
        status_code: Status::Ok,
        status: true,
        message: entry.to_string()
    }
}

#[post("/actions", data = "<action_name>")]
async fn create_action<'a>(
    db: &State<SurrealRepo>,
    action: &State<ActionList>,
    action_name: &str,
) -> Result<JsonStatus<&'a str>, Status> {
    return controllers::ActionController::create_action(db, action, action_name).await;
}

//Products

#[post("/<action>", format = "json", data = "<product>")]
async fn action_product<'a>(
    db: &State<SurrealRepo>,
    action_list: &State<ActionList>,
    action: String,
    product: Json<ProductModels::ActionDTO<'a>>,
) -> Result<JsonStatus<&'a str>, Status> {
    let contains = action_exists(&action, action_list);
    match contains {
        true => {
            let query = controllers::ActionController::action_product(db, action, product.into_inner()).await;
            match query {
                Ok(status) => Ok(JsonStatus {
                    status_code: status.0,
                    status: true,
                    message: status.1,
                }),
                Err(e) => Err(e),
            }
        },
        false => Err(Status::NotFound)
    }
}
