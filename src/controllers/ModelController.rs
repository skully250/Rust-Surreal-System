use surrealdb::sql::Value;

use crate::{
    models::ProductModels,
    util::responders::{JsonMessage, RequestResponse, ServerMessage},
    SurrealRepo,
};

pub async fn get_models(db: &SurrealRepo) -> Result<Vec<ProductModels::Model>, RequestResponse> {
    let query = db.find(None, "models").await;
    return match query {
        Ok(query) => {
            let model_result = query[0].output().unwrap();
            if let Value::Array(rows) = model_result {
                let models: Vec<ProductModels::Model> = serde_json::from_value(serde_json::json!(&rows))
                    .expect("Failed to parse model data");
                Ok(models)
            } else {
                Err(RequestResponse::BadRequest(ServerMessage::new(
                    JsonMessage {
                        status: false,
                        message: "Error while processing models from DB".to_string(),
                    },
                )))
            }
        }
        Err(e) => Err(RequestResponse::InternalErrorRequest(ServerMessage::new(
            JsonMessage {
                status: false,
                message: e.to_string(),
            },
        ))),
    };
}

pub async fn add_model(
    db: &SurrealRepo,
    content: ProductModels::ModelDTO,
) -> Result<RequestResponse, RequestResponse> {
    let name = content.name.to_owned();
    let query = db.create("models", content, Some(name)).await;
    return match query {
        Ok(query) => {
            let result_entry = query[0].output();
            if result_entry.is_ok() {
                Ok(RequestResponse::OKRequest(ServerMessage::new(
                    JsonMessage {
                        status: true,
                        message: "Succesfully created new model".to_string(),
                    },
                )))
            } else {
                Err(RequestResponse::BadRequest(ServerMessage::new(
                    JsonMessage {
                        status: false,
                        message: "Error while creating model in DB".to_string(),
                    },
                )))
            }
        }
        Err(e) => Err(RequestResponse::InternalErrorRequest(ServerMessage::new(
            JsonMessage {
                status: false,
                message: e.to_string(),
            },
        ))),
    };
}