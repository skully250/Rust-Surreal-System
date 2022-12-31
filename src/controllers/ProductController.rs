use rocket::http::Status;

use crate::{models::ProductModels::ActionDTO, SurrealRepo};

pub async fn action_product<'a>(
    db: &SurrealRepo,
    action: ActionDTO<'a>,
) -> Result<(Status, &'a str), Status> {
    let data = serde_json::json!(action.action).to_string();
    let query = format!(
        "UPDATE {0} SET actions.{1} = {2}",
        action.order_id, action.name, data
    );
    let query_result = db.query(&query).await;
    return match query_result {
        Ok(_query) => Ok((Status::Ok, "Action successfully run")),
        Err(_e) => Err(Status::BadRequest),
    };
}

//Potentially Deprecated - May return in future
// pub async fn create_product(
//     db: &SurrealRepo,
//     content: &models::ProductModels::ProductDTO,
// ) -> Result<Vec<Response>, surrealdb::Error> {
//     let query = db.create("products", content, None).await;
//     return match query {
//         Ok(query) => Ok(query),
//         Err(e) => panic!("DB Could not create product - Error: {:?}", e),
//     };
// }
