mod controllers;
mod models;
mod repository;
mod routes;
mod util;

#[macro_use]
extern crate rocket;
use repository::SurrealRepo::{DBConfig, SurrealRepo};
use rocket::{http::Status, serde::json::Json, State};
use routes::{Orders, Products};
use surrealdb::sql::Value;

//TODO: Refactor MVC names to include their affix (Controller, Model, Route)

#[get("/")]
fn index() {}

#[get("/test")]
async fn teapot(db: &State<SurrealRepo>) -> Result<serde_json::Value, Status> {
    let query = db
        .query("SELECT ->created->order.* as orders FROM user:fae")
        .await;
    return match query {
        Ok(query) => {
            let query_result = query[0].output().unwrap();
            if let Value::Array(rows) = query_result {
                Ok(serde_json::json!(rows))
            } else {
                panic!("DB did not return");
            }
        }
        Err(_) => Err(Status::InternalServerError),
    };
}

#[get("/addItem")]
async fn add_surreal_item(
    db: &State<SurrealRepo>,
) -> Result<Json<Vec<surrealdb::Response>>, Status> {
    let query = db
        .query(
            "CREATE person CONTENT {
            name: 'Fae'
        }",
        )
        .await;
    return match query {
        Ok(query) => Ok(Json(query)),
        Err(_) => Err(Status::InternalServerError),
    };
}

#[get("/getItems")]
async fn get_surreal_items(db: &State<SurrealRepo>) -> Result<serde_json::Value, Status> {
    let query = db.find(None, "person").await;
    return match query {
        Ok(query) => {
            let query_result = query[0].output().unwrap();
            if let Value::Array(rows) = query_result {
                Ok(serde_json::json!(rows))
            } else {
                panic!("DB did not return")
            }
        }
        Err(_) => Err(Status::InternalServerError),
    };
}

#[catch(422)]
fn mangled_data() {
    //Todo: Send back server message about unprocessable data here
}

#[launch]
async fn rocket() -> _ {
    let config = DBConfig {
        path: "memory",
        ns: "test",
        db: "test",
    };
    let surreal = SurrealRepo::init(config).await;
    rocket::build()
        .manage(surreal)
        .mount(
            "/api",
            routes![index, teapot, add_surreal_item, get_surreal_items],
        )
        .mount("/api/orders", Orders::order_routes())
        .mount("/api/products", Products::product_routes())
}
