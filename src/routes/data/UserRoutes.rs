use rocket::{http::Status, serde::json::Json, Route, State};

use crate::{
    controllers,
    models::UserModels::{DBUser, UserDTO},
    util::responders::JsonStatus,
    SurrealRepo,
};

pub fn user_routes() -> Vec<Route> {
    let routes = routes![get_users, add_users];
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
async fn add_users(db: &State<SurrealRepo>, user: Json<UserDTO>) -> Result<JsonStatus<&str>, Status> {
    controllers::UserController::add_user(db, user.into_inner()).await
}
