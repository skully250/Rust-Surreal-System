use rocket::http::Status;
use surrealdb::sql::Value;

use crate::{models::ProductModels, util::responders::JsonStatus, SurrealRepo};

pub async fn get_models(db: &SurrealRepo) -> Result<Vec<ProductModels::Model>, Status> {
    let query = db.find(None, "models").await;
    return match query {
        Ok(query) => {
            let model_result = query[0].output().unwrap();
            if let Value::Array(rows) = model_result {
                let models: Vec<ProductModels::Model> =
                    serde_json::from_value(serde_json::json!(&rows))
                        .expect("Failed to parse model data");
                Ok(models)
            } else {
                Err(Status::BadRequest)
            }
        }
        Err(e) => Err(Status::InternalServerError),
    };
}

pub async fn add_model(
    db: &SurrealRepo,
    content: ProductModels::ModelDTO,
) -> Result<JsonStatus, Status> {
    let name = content.name.to_owned();
    let query = db.create("models", content, Some(name)).await;
    return match query {
        Ok(query) => {
            let result_entry = query[0].output();
            if result_entry.is_ok() {
                Ok(JsonStatus {
                    status_code: Status::Ok,
                    status: true,
                    message: "Succesfully created new model",
                })
            } else {
                Err(Status::BadRequest)
            }
        }
        Err(e) => Err(Status::InternalServerError),
    };
}
