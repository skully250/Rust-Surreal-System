mod controllers;
mod models;
mod repository;
mod routes;
mod util;

#[macro_use]
extern crate rocket;
extern crate dotenv;

use dotenv::dotenv;
use rocket::serde::json::Json;

use repository::SurrealRepo::{DBConfig, SurrealRepo};
use routes::data::{CustomerRoutes, OrderRoutes, ProductRoutes, UserRoutes};
use util::responders::JsonMessage;

#[catch(422)]
fn mangled_data() -> Json<JsonMessage> {
    Json(JsonMessage {
        status: false,
        message: "Data sent to the client was incorrect".to_string(),
    })
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
        .mount("/api/orders", OrderRoutes::order_routes())
        .mount("/api/products", ProductRoutes::product_routes())
        .mount("/api/customers", CustomerRoutes::customer_routes())
        .mount("/api/users", UserRoutes::user_routes())
        .register("/api", catchers![mangled_data])
}
