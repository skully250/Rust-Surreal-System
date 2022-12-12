use rocket::{http::Status, State, serde::json::Json};
use surrealdb::sql::Value;

use crate::{
    controllers, models,
    util::responders::{JsonMessage, RequestResponse, ServerMessage},
    SurrealRepo,
};

#[get("/products")]
pub async fn get_products(db: &State<SurrealRepo>) -> Result<serde_json::Value, RequestResponse> {
    let query = controllers::Products::get_products(db).await;
    return match query {
        Ok(get_output) => {
            let get_result = get_output[0].output().unwrap();
            if let Value::Array(rows) = get_result {
                Ok(serde_json::json!(rows))
            } else {
                Err(RequestResponse::BadRequest(ServerMessage::new(
                    JsonMessage {
                        status: false,
                        message: "Error Fetching products from DB".to_string(),
                    },
                )))
            }
        }
        Err(_) => Err(RequestResponse::InternalErrorRequest(ServerMessage::new(
            JsonMessage {
                status: false,
                message: "Error fetching products".to_string(),
            },
        ))),
    };
}

#[post("/products", format = "json", data= "<product>")]
pub async fn add_products(
    db: &State<SurrealRepo>,
    product: Json<models::Product::ProductDTO>,
) -> Result<RequestResponse, RequestResponse> {
    let query = db.create("product", &product.into_inner()).await;
    return match query {
        Ok(product_output) => {
            if product_output[0].output().is_ok() {
                Ok(RequestResponse::OKRequest(ServerMessage::new(
                    JsonMessage {
                        status: true,
                        message: "Successfully added product to DB".to_string(),
                    },
                )))
            } else {
                Err(RequestResponse::BadRequest(ServerMessage::new(
                    JsonMessage {
                        status: false,
                        message: "Error adding product to DB".to_string(),
                    },
                )))
            }
        }
        Err(_) => Err(RequestResponse::InternalErrorRequest(ServerMessage::new(
            JsonMessage {
                status: false,
                message: "Error adding product".to_string(),
            },
        ))),
    };
}
