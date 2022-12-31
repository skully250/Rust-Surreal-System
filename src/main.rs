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
use surrealdb::sql::Value;
use util::responders::JsonStatus;

use crate::models::{UserModels::UserDTO, AuthModels::AuthUser};

//Come back to responders and find a better way to handle them
#[catch(422)]
fn mangled_data() -> JsonStatus<'static> {
    JsonStatus {
        status_code: Status::UnprocessableEntity,
        status: false,
        message: "Data sent to the client was incorrect",
    }
}

#[catch(401)]
fn unauthorized() -> JsonStatus<'static> {
    JsonStatus {
        status_code: Status::Unauthorized,
        status: false,
        message: "User is unauthorized",
    }
}

#[catch(400)]
fn bad_request() -> JsonStatus<'static> {
    JsonStatus {
        status_code: Status::BadRequest,
        status: false,
        message: "Request failed to go through",
    }
}

#[catch(500)]
fn internal_error() -> JsonStatus<'static> {
    JsonStatus {
        status_code: Status::InternalServerError,
        status: false,
        message: "Internal error processing request",
    }
}

#[catch(501)]
fn not_implemented() -> JsonStatus<'static> {
    JsonStatus {
        status_code: Status::NotImplemented,
        status: false,
        message: "Not yet implemented",
    }
}

//Function to test the DB in dev, will be removed for prod
#[post("/query", data = "<query>")]
async fn exec_query(db: &State<SurrealRepo>, query: &str) -> Result<serde_json::Value, Status> {
    let exec = db.query(query).await;
    return match exec {
        Ok(query) => {
            println!("{:?}", query);
            let query_result = query[0].output().unwrap();
            if let Value::Array(rows) = query_result {
                let query_return = serde_json::json!(&rows);
                Ok(query_return)
            } else {
                Err(Status::BadRequest)
            }
        }
        Err(e) => {
            println!("{:?}", e);
            Err(Status::BadRequest)
        }
    };
}

#[post("/login", format = "json", data = "<user>")]
async fn login_user<'a>(
    db: &State<SurrealRepo>,
    user: Json<UserDTO>,
    cookies: &CookieJar<'_>,
) -> Result<JsonStatus<'a>, Status> {
    return controllers::UserController::login_user(db, cookies, user.into_inner()).await;
}

#[get("/")]
async fn logged_in<'a>(_user: AuthUser) -> JsonStatus<'a> {
    JsonStatus {
        status_code: Status::Accepted,
        status: true,
        message: "You are currently logged in"
    }
}

#[get("/", rank = 2)]
fn not_logged_in<'a>() -> JsonStatus<'a> {
    JsonStatus {
        status_code: Status::Unauthorized,
        status: false,
        message: "You are not current logged in"
    }
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
        .mount("/api", routes![login_user, exec_query, logged_in, not_logged_in])
        .mount("/api/orders", OrderRoutes::order_routes())
        .mount("/api/products", ProductRoutes::product_routes())
        .mount("/api/customers", CustomerRoutes::customer_routes())
        .mount("/api/users", UserRoutes::user_routes())
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
