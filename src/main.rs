mod models;
mod repository;

#[macro_use]
extern crate rocket;
use models::umbrella_model::Umbrella;
use repository::{MongoRepo::MongoRepo, SurrealRepo::SurrealRepo};
use rocket::{http::Status, serde::json::Json, State};
use surrealdb::{sql::Value};

pub mod response_types {
    #[derive(Responder)]
    #[response(status = 200, content_type = "text")]
    pub struct TextResponse(pub &'static str);

    #[derive(Responder)]
    #[response(status = 200, content_type = "json")]
    pub struct JSONResponse(pub &'static str);
}

use response_types::{JSONResponse, TextResponse};

#[get("/")]
fn index() -> TextResponse {
    return TextResponse("Hello World");
}

#[get("/test")]
fn teapot() -> JSONResponse {
    return JSONResponse("{\"api\": \"hello\"}");
}

#[get("/umbrellas")]
fn get_umbrellas(db: &State<MongoRepo>) -> Result<Json<Vec<Umbrella>>, Status> {
    let umbrellas = db.get_umbrellas();
    return match umbrellas {
        Ok(umbrellas) => Ok(Json(umbrellas)),
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
async fn get_surreal_items(
    db: &State<SurrealRepo>,
) -> Result<serde_json::Value, Status> {
    let query = db.query("SELECT * FROM person").await;
    return match query {
        Ok(query) => {
            let query_result = query[0].output().unwrap();
            if let Value::Array(rows) = query_result {
                Ok(serde_json::json!(rows))
            } else {
                panic!("DB did not return")
            }
        },
        Err(_) => Err(Status::InternalServerError),
    };
}

#[launch]
async fn rocket() -> _ {
    let db = MongoRepo::init();
    let surreal = SurrealRepo::init("test", "test").await;
    rocket::build()
        .manage(db)
        .manage(surreal)
        .mount("/api", routes![index, teapot, get_umbrellas, add_surreal_item, get_surreal_items])
}
