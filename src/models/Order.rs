use serde::{Serialize, Deserialize};
use surrealdb::sql::{Id, Datetime};
use crate::models::Product::Product;
use crate::models::User::{User, Customer};


//TODO: Discover best practices for keeping typing but using Surreal ID's
//TOOD: Turn Created - Finished - MadeBy into Graph edges in Surreal
#[derive(Serialize, Deserialize)]
pub struct Order {
    id: Id,
    customer: Customer,
    order_number: u32,
    products: Vec<Product>,
    removed: bool,
    due_date: Datetime,
    created_by: User,
    created_on: Datetime,
    finished_on: Datetime
}

#[derive(Deserialize)]
pub struct OrderDTO {
    customer: String,
    products: Vec<String>
}