use rocket::{serde::json::Json, Route};

use crate::{
    controllers, models::UserModels::{User, UserDTO}, util::responders::{ApiResult, Jsonstr}
};

pub fn user_routes() -> Vec<Route> {
    let routes = routes![get_users, add_users, edit_user, delete_user];
    return routes;
}

//TODO: Guarded routes to protect against data leaking
#[get("/")]
async fn get_users() -> ApiResult<Json<Vec<User>>> {
    let users = controllers::UserController::get_users().await;
    return match users {
        Ok(users) => Ok(Json(users)),
        Err(e) => Err(e),
    };
}

#[post("/", format = "json", data = "<user>")]
async fn add_users<'a>(user: Json<UserDTO>) -> Jsonstr<'a> {
    return controllers::UserController::add_user(user.into_inner()).await;
}

#[put("/<user_id>", format = "json", data = "<user>")]
async fn edit_user(user: Json<UserDTO>, user_id: &str) -> Jsonstr {
    //let db_name = format!("users:{user_id}");
    return controllers::UserController::edit_user(user.into_inner(), user_id).await;
}

#[delete("/<user_id>")]
async fn delete_user(user_id: &str) -> Jsonstr {
    //let db_name = format!("users:{user_id}");
    return controllers::UserController::delete_user(user_id).await;
}
