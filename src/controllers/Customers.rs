use crate::{SurrealRepo, util::responders::RequestResponse};

pub async fn get_customers(db: &SurrealRepo) -> Result<serde_json::Value, RequestResponse> {
    Ok(serde_json::json!("Not yet implemented"))
}