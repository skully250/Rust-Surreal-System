use rocket::{http::Status, serde::json::Json, Route};

use crate::{
    controllers,
    models::UserModels::{DBUser, UserDTO},
    util::responders::JsonStatus,
};

pub fn user_routes() -> Vec<Route> {
    let routes = routes![get_users, add_users, edit_user, delete_user];
    return routes;
}

//TODO: Guarded routes to protect against data leaking
#[get("/")]
async fn get_users() -> Result<Json<Vec<DBUser>>, Status> {
    let users = controllers::UserController::get_users().await;
    return match users {
        Ok(users) => Ok(Json(users)),
        Err(e) => Err(e),
    };
}

#[post("/", format = "json", data = "<user>")]
async fn add_users(user: Json<UserDTO>) -> Result<JsonStatus<String>, Status> {
    return controllers::UserController::add_user(user.into_inner()).await;
}

#[put("/<user_id>", format = "json", data = "<user>")]
async fn edit_user(user: Json<UserDTO>, user_id: String) -> Result<JsonStatus<String>, Status> {
    let db_name = format!("users:{user_id}");
    return controllers::UserController::edit_user(user.into_inner(), db_name).await;
}

#[delete("/<user_id>")]
async fn delete_user(user_id: String) -> Result<JsonStatus<String>, Status> {
    let db_name = format!("users:{user_id}");
    return controllers::UserController::delete_user(db_name).await;
}
