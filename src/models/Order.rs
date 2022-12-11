use serde::{Serialize, Deserialize};
use surrealdb::sql::{Id, Datetime};
use crate::{models::Product::Product, repository::SurrealRepo::SurrealRepo};

use super::User::User;


//TODO: Discover best practices for keeping typing but using Surreal ID's
//TOOD: Turn Created - Finished - MadeBy into Graph edges in Surreal
#[derive(Serialize, Deserialize)]
pub struct Order {
    id: Id,
    customer: Id,
    order_number: u32,
    products: Vec<Product>,
    removed: bool,
    due_date: Datetime
}

impl Order {
    async fn get_created_by(db: &SurrealRepo, orderNo: u32) -> Result<Option<User>, surrealdb::Error> {
        let order_id = format!("orders:{orderNo}");
        let results = db.find("->created->user.*", &order_id).await;
        return match results {
            Ok(find_output) => {
                let find_result = find_output[0].output().unwrap();
                let find_string = find_result.to_string();
                let user: Vec<User> = serde_json::from_str(&find_string).expect("Failed to parse into user data");
                Ok(user.into_iter().nth(0))
            },
            Err(_) => panic!("Failed to find user that created order")
        }
    }
}

/*
 * The Order DTO is created as a way to create a parseable object for 
 * Serde and Rocket when creating objects
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderDTO {
    customer: String,
    products: Vec<String>
}