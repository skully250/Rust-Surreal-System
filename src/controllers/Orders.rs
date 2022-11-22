use surrealdb::Response;

use crate::{SurrealRepo, models};

//Using namespaces to avoid confusiong between model and controller
pub async fn get_orders(db: &SurrealRepo) -> Result<Vec<Response>, surrealdb::Error> {
    let query = db.query("SELECT * FROM orders").await;
    return match query {
        Ok(query) => {
            Ok(query)
        }
        Err(E) => panic!("DB Could not fetch data - Error: {:?}", E)
    }
}

pub async fn add_order(db: &SurrealRepo, content: models::Order::Order) -> Result<Vec<Response>, surrealdb::Error> {
    let json_content = serde_json::json!(content);
    let query = db.create("order", json_content).await;
    return match query {
        Ok(query) => {
            Ok(query)
        }
        Err(E) => panic!("DB Could not add orer - Error: {:?}", E)
    }
}