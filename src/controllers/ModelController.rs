use rocket::http::Status;
use surrealdb::sql::Value;

use crate::{models::ProductModels, util::responders::JsonStatus, SurrealRepo};

pub async fn get_models(
    db: &SurrealRepo,
    fetch_all: Option<bool>,
) -> Result<Vec<ProductModels::DBModel>, Status> {
    let query: Result<Vec<surrealdb::Response>, surrealdb::Error>;
    if fetch_all.is_some() {
        query = db.find(None, "models").await;
    } else {
        let query_string = format!("active != false");
        query = db.find_where(None, "models", &query_string).await;
    }
    return match query {
        Ok(query) => {
            let model_result = query[0].output().unwrap();
            if let Value::Array(rows) = model_result {
                let models: Vec<ProductModels::DBModel> =
                    serde_json::from_value(serde_json::json!(&rows))
                        .expect("Failed to parse model data");
                Ok(models)
            } else {
                Err(Status::BadRequest)
            }
        }
        Err(_) => Err(Status::InternalServerError),
    };
}

pub async fn add_model<'a>(
    db: &SurrealRepo,
    content: ProductModels::ModelDTO,
) -> Result<JsonStatus<&'a str>, Status> {
    //Take ownership of DTO Name as it is required for creation of the model in the DB
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
        Err(_) => Err(Status::InternalServerError),
    };
}

pub async fn edit_model(
    db: &SurrealRepo,
    content: ProductModels::ModelDTO,
    product_id: String,
) -> Result<JsonStatus<&str>, Status> {
    let query = db.update(&product_id, content).await;
    return match query {
        Ok(query) => {
            let is_empty = query[0].output().unwrap().is_none();
            if !is_empty {
                Ok(JsonStatus {
                    status_code: Status::Ok,
                    status: true,
                    message: "Successfully updated models",
                })
            } else {
                Ok(JsonStatus {
                    status_code: Status::NotFound,
                    status: false,
                    message: "Model does not exist",
                })
            }
        }
        Err(_) => Err(Status::InternalServerError),
    };
}

pub async fn delete_model(
    db: &SurrealRepo,
    product_id: String,
) -> Result<JsonStatus<&str>, Status> {
    let query_string = format!("UPDATE {0} SET active = false", product_id);
    let query = db.query(&query_string).await;
    return match query {
        Ok(query) => {
            let is_empty = query[0].output().unwrap().is_none();
            if !is_empty {
                Ok(JsonStatus {
                    status_code: Status::Ok,
                    status: true,
                    message: "Successfully removed model",
                })
            } else {
                Ok(JsonStatus {
                    status_code: Status::NotFound,
                    status: false,
                    message: "Model doesnt exist",
                })
            }
        }
        Err(_) => Err(Status::InternalServerError),
    };
}
