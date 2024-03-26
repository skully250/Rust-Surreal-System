use rocket::http::Status;

use crate::{models::ProductModels, repository::SurrealRepo, util::responders::JsonStatus};

pub async fn get_models(
    fetch_all: Option<bool>,
) -> Result<Vec<ProductModels::ProductModel>, Status> {
    let query: Result<Vec<ProductModels::ProductModel>, surrealdb::Error>;
    if fetch_all.is_some_and(|fetch| fetch == true) {
        query = SurrealRepo::find_all("models").await;
    } else {
        query = SurrealRepo::find_all_where("models", "active=true").await;
    }
    return match query {
        Ok(query) => {
            println!("Query Return: {:?}", &query);
            Ok(query)
        }
        Err(_) => Err(Status::InternalServerError),
    };
}

pub async fn add_model<'a>(
    content: ProductModels::ProductModel,
) -> Result<JsonStatus<String>, Status> {
    //Take ownership of DTO Name as it is required for creation of the model in the DB
    let name = content.name.to_owned();
    let query = SurrealRepo::create_named("models", &name, content).await;
    return match query {
        Ok(_) => Ok(JsonStatus::success("Successfully created new model")),
        Err(_) => Err(Status::InternalServerError),
    };
}

pub async fn edit_model(
    content: ProductModels::ProductModel,
    product_id: &str,
) -> Result<JsonStatus<String>, Status> {
    let query = SurrealRepo::update("models", &product_id, content).await;
    return match query {
        Ok(_) => Ok(JsonStatus::success("Successfully updated models")),
        Err(_) => Err(Status::InternalServerError),
    };
}

//TODO: ISSUES WITH QUERY ELSEWHERE, TEST IF PERSISTS IN THESE FUNCTIONS
pub async fn restore_model(
    product_id: String,
) -> Result<JsonStatus<String>, Status> {
    let query_string = format!("UPDATE {0} SET active = true", product_id);
    let query = SurrealRepo::query(&query_string).await;
    return match query {
        Ok(_) => Ok(JsonStatus::success("Successfully restored model")),
        Err(_) => Err(Status::InternalServerError),
    };
}

pub async fn delete_model(
    product_id: String,
) -> Result<JsonStatus<String>, Status> {
    let query_string = format!("UPDATE {0} SET active = false", product_id);
    let query = SurrealRepo::query(&query_string).await;
    println!("{:?}", query);
    return match query {
        Ok(_) => Ok(JsonStatus::success("Successfully removed model")),
        Err(_) => Err(Status::InternalServerError),
    };
}