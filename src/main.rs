mod controllers;
mod models;
mod repository;
mod routes;
mod util;

#[macro_use]
extern crate rocket;
extern crate dotenv;

use dotenv::dotenv;
use models::ActionModels::ActionList;
use rocket::{
    http::{CookieJar, Status},
    serde::json::Json,
    tokio::sync::RwLock,
};

use repository::SurrealRepo::{self, DBConfig};
use routes::data::ProductRoutes;
use util::responders::JsonStatus;

use crate::models::{ActionModels::DBAction, AuthModels::AuthUser, UserModels::UserDTO};

//Come back to responders and find a better way to handle them
#[catch(422)]
fn mangled_data() -> JsonStatus<&'static str> {
    JsonStatus {
        status_code: Status::UnprocessableEntity,
        status: false,
        message: "Data sent to the client was incorrect",
    }
}

#[catch(401)]
fn unauthorized() -> JsonStatus<&'static str> {
    JsonStatus {
        status_code: Status::Unauthorized,
        status: false,
        message: "User is unauthorized",
    }
}

#[catch(400)]
fn bad_request() -> JsonStatus<&'static str> {
    JsonStatus {
        status_code: Status::BadRequest,
        status: false,
        message: "Request failed to go through",
    }
}

#[catch(500)]
fn internal_error() -> JsonStatus<&'static str> {
    JsonStatus {
        status_code: Status::InternalServerError,
        status: false,
        message: "Internal error processing request",
    }
}

#[catch(501)]
fn not_implemented() -> JsonStatus<&'static str> {
    JsonStatus {
        status_code: Status::NotImplemented,
        status: false,
        message: "Not yet implemented",
    }
}

#[post("/login", format = "json", data = "<user>")]
async fn login_user(
    user: Json<UserDTO>,
    cookies: &CookieJar<'_>,
) -> Result<JsonStatus<String>, Status> {
    return controllers::UserController::login_user(cookies, user.into_inner()).await;
}

#[get("/")]
async fn logged_in<'a>(_user: AuthUser) -> JsonStatus<&'a str> {
    JsonStatus {
        status_code: Status::Accepted,
        status: true,
        message: "You are currently logged in",
    }
}

async fn get_actions() -> ActionList {
    let query: Vec<DBAction> = SurrealRepo::find_all("actions")
        .await
        .expect("Unable to fetch actions from DB");
    println!("{:?}", query);
    let mut action_list: Vec<String> = vec![];
    for action in query.iter() {
        action_list.push(action.name.to_string());
    }
    return ActionList {
        actions: RwLock::new(action_list),
    };
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let config = DBConfig {
        path: "127.0.0.1:8000",
        ns: "test",
        db: "test",
    };
    SurrealRepo::connect(config).await;
    //Create a list of current actions upon the initialization of the application
    //That will be tracked and updated with
    let actions = get_actions().await;
    rocket::build()
        .manage(actions)
        .mount("/api", routes![login_user, logged_in])
        //.mount("/api/orders", OrderRoutes::order_routes())
        .mount("/api/products", ProductRoutes::product_routes())
        //.mount("/api/customers", CustomerRoutes::customer_routes())
        //.mount("/api/users", UserRoutes::user_routes())
        .register(
            "/api",
            catchers![
                mangled_data,
                unauthorized,
                bad_request,
                internal_error,
                not_implemented
            ],
        )
}
