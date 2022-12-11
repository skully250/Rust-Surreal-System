use surrealdb::Response;

use crate::{models, SurrealRepo};

//Using namespaces to avoid confusiong between model and controller
pub async fn get_orders(db: &SurrealRepo) -> Result<Vec<Response>, surrealdb::Error> {
    let query = db.query("SELECT * FROM orders").await;
    return match query {
        Ok(query) => Ok(query),
        Err(e) => panic!("DB Could not fetch data - Error: {:?}", e),
    };
}

pub async fn create_order(
    db: &SurrealRepo,
    content: models::Order::Order,
) -> Result<Vec<Response>, surrealdb::Error> {
    let json_content = serde_json::json!(content);
    let query = db.create("order", json_content).await;
    return match query {
        Ok(query) => Ok(query),
        Err(e) => panic!("DB Could not add orer - Error: {:?}", e),
    };
}

pub async fn update_order(
    db: &SurrealRepo,
    order_no: i32,
    order: models::Order::OrderDTO,
) -> Result<Vec<Response>, surrealdb::Error> {
    let json_content = serde_json::json!(order);
    let cur_order = format!("order:{order_no}");
    let query = db.create(&cur_order, json_content).await;
    return match query {
        Ok(query) => Ok(query),
        Err(e) => panic!("DB Could not update Order - Error: {:?}", e),
    }
}