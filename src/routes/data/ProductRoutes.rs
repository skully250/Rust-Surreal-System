use rocket::{http::Status, serde::json::Json, Route, State};

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
    ];
    return routes;
}

fn action_exists(action_name: &String, action_list: &State<ActionList>) -> bool {
    let action_list = action_list.actions.read().unwrap();
    action_list.contains(action_name)
}

//Models

#[get("/models")]
async fn get_models(db: &State<SurrealRepo>) -> Result<Json<Vec<ProductModels::DBModel>>, Status> {
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

//Actions

#[get("/actions")]
async fn get_actions(action_list: &State<ActionList>) -> Json<Vec<String>> {
    let actions = action_list.actions.read().unwrap().to_vec();
    return Json(actions);
}

#[post("/actions", data = "<action_name>")]
async fn create_action<'a>(
    db: &State<SurrealRepo>,
    action: &State<ActionList>,
    action_name: &str,
) -> Result<JsonStatus<&'a str>, Status> {
    return controllers::ActionController::create_action(db, action, action_name).await;
}

#[put("/actions", data = "<action_name>")]
async fn update_action(db: &State<SurrealRepo>,
action: &State<ActionList>,
action_name: &str) {
    
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
            let query =
                controllers::ActionController::action_product(db, action, product.into_inner())
                    .await;
            match query {
                Ok(status) => Ok(JsonStatus {
                    status_code: status.0,
                    status: true,
                    message: status.1,
                }),
                Err(e) => Err(e),
            }
        }
        false => Err(Status::NotFound),
    }
}
