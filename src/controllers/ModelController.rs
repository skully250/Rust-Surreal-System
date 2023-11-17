use rocket::http::Status;
use surrealdb::sql::Value;

use crate::{models::ProductModels, util::responders::JsonStatus, SurrealRepo};

pub async fn get_models(
    db: &SurrealRepo,
    fetch_all: Option<bool>,
) -> Result<Vec<ProductModels::DBModel>, Status> {
    let query: Result<Vec<ProductModels::DBModel>, surrealdb::Error>;
    if fetch_all.is_some() {
        query = db.find_all("models").await;
    } else {
        let query_string = format!("active != false");
        query = db.find_where("models", None, &query_string).await;
    }
    return match query {
        Ok(query) => {
            println!("{:?}", &query);
            Ok(query)
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
    let query = db.create_named("models", &name, content).await;
    return match query {
        Ok(_) => Ok(JsonStatus {
            status_code: Status::Ok,
            status: true,
            message: "Succesfully created new model",
        }),
        Err(_) => Err(Status::InternalServerError),
    };
}

pub async fn edit_model(
    db: &SurrealRepo,
    content: ProductModels::ModelDTO,
    product_id: String,
) -> Result<JsonStatus<&str>, Status> {
    let query = db.update("models", &product_id, content).await;
    return match query {
        Ok(_) => Ok(JsonStatus {
            status_code: Status::Ok,
            status: true,
            message: "Successfully updated models",
        }),
        Err(_) => Err(Status::InternalServerError),
    };
}

//TODO: ISSUES WITH QUERY ELSEWHERE, TEST IF PERSISTS IN THESE FUNCTIONS
pub async fn restore_model(
    db: &SurrealRepo,
    product_id: String,
) -> Result<JsonStatus<&str>, Status> {
    let query_string = format!("UPDATE {0} SET active = true", product_id);
    let query = db.query(&query_string).await;
    return match query {
        Ok(_) => Ok(JsonStatus {
            status_code: Status::Ok,
            status: true,
            message: "Successfully restored model",
        }),
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
        Ok(query) => Ok(JsonStatus {
            status_code: Status::Ok,
            status: true,
            message: "Successfully removed model",
        }),
        Err(_) => Err(Status::InternalServerError),
    };
}
