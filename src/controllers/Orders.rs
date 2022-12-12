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
    content: &models::Order::OrderDTO,
) -> Result<Vec<Response>, surrealdb::Error> {
    let query = db.create("order", content).await;
    return match query {
        Ok(query) => Ok(query),
        Err(e) => panic!("DB Could not add order - Error: {:?}", e),
    };
}

pub async fn update_order(
    db: &SurrealRepo,
    order_no: u32,
    order: &models::Order::OrderDTO,
) -> Result<Vec<Response>, surrealdb::Error> {
    let cur_order = format!("order:{order_no}");
    let query = db.update(&cur_order, order).await;
    return match query {
        Ok(query) => Ok(query),
        Err(e) => panic!("DB Could not update Order - Error: {:?}", e),
    }
}