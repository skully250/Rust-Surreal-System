mod controllers;
mod models;
mod repository;
mod routes;
mod util;

#[macro_use]
extern crate rocket;
extern crate dotenv;

use dotenv::dotenv;
use rocket::{
    http::{CookieJar, Status},
    serde::json::Json,
    State,
};

use repository::SurrealRepo::{DBConfig, SurrealRepo};
use routes::data::{CustomerRoutes, OrderRoutes, ProductRoutes, UserRoutes};
use util::responders::JsonMessage;

use crate::models::UserModels::UserDTO;

//Come back to responders and find a better way to handle them
#[catch(422)]
fn mangled_data() -> JsonMessage<'static> {
    JsonMessage {
        status_code: Status::UnprocessableEntity,
        status: false,
        message: "Data sent to the client was incorrect",
    }
}

#[catch(401)]
fn unauthorized() -> JsonMessage<'static> {
    JsonMessage {
        status_code: Status::Unauthorized,
        status: false,
        message: "User is unauthorized",
    }
}

#[catch(400)]
fn bad_request() -> JsonMessage<'static> {
    JsonMessage {
        status_code: Status::BadRequest,
        status: false,
        message: "Request failed to go through",
    }
}

#[catch(500)]
fn internal_error() -> JsonMessage<'static> {
    JsonMessage {
        status_code: Status::InternalServerError,
        status: false,
        message: "Internal error processing request",
    }
}

#[catch(501)]
fn not_implemented() -> JsonMessage<'static> {
    JsonMessage {
        status_code: Status::NotImplemented,
        status: false,
        message: "Not yet implemented",
    }
}

#[post("/login", format = "json", data = "<user>")]
async fn login_user<'a>(
    db: &State<SurrealRepo>,
    user: Json<UserDTO>,
    cookies: &CookieJar<'_>,
) -> Result<JsonMessage<'a>, Status> {
    return controllers::UserController::login_user(db, cookies, user.into_inner()).await;
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let config = DBConfig {
        path: "memory",
        ns: "test",
        db: "test",
    };
    let surreal = SurrealRepo::init(config).await;
    rocket::build()
        .manage(surreal)
        .mount("/api", routes![login_user])
        .mount("/api/orders", OrderRoutes::order_routes())
        .mount("/api/products", ProductRoutes::product_routes())
        .mount("/api/customers", CustomerRoutes::customer_routes())
        .mount("/api/users", UserRoutes::user_routes())
        .register(
            "/api",
            catchers![mangled_data, unauthorized, bad_request, internal_error],
        )
}
