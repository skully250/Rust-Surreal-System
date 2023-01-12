use rocket::{http::Status, serde::json::Json, Route, State};

use crate::{
    controllers,
    models::UserModels::{DBUser, UserDTO},
    util::responders::JsonStatus,
    SurrealRepo,
};

pub fn user_routes() -> Vec<Route> {
    let routes = routes![get_users, add_users, edit_user, delete_user];
    return routes;
}

//TODO: Guarded routes to protect against data leaking
#[get("/")]
async fn get_users(db: &State<SurrealRepo>) -> Result<Json<Vec<DBUser>>, Status> {
    let users = controllers::UserController::get_users(db).await;
    return match users {
        Ok(users) => Ok(Json(users)),
        Err(e) => Err(e),
    };
}

#[post("/", format = "json", data = "<user>")]
async fn add_users(
    db: &State<SurrealRepo>,
    user: Json<UserDTO>,
) -> Result<JsonStatus<&str>, Status> {
    return controllers::UserController::add_user(db, user.into_inner()).await;
}

#[put("/<user_id>", format = "json", data = "<user>")]
async fn edit_user(
    db: &State<SurrealRepo>,
    user: Json<UserDTO>,
    user_id: String,
) -> Result<JsonStatus<&str>, Status> {
    let db_name = format!("users:{user_id}");
    return controllers::UserController::edit_user(db, user.into_inner(), db_name).await;
}

#[delete("/<user_id>")]
async fn delete_user(db: &State<SurrealRepo>, user_id: String) -> Result<JsonStatus<&str>, Status> {
    let db_name = format!("users:{user_id}");
    return controllers::UserController::delete_user(db, db_name).await;
}