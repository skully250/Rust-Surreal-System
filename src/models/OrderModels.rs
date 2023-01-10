use crate::{
    models::ProductModels,
    repository::SurrealRepo::{DBInteractions, SurrealRepo},
};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;

use super::UserModels::DBUser;

/*
 * The Order DTO is created as a way to create a parseable input
 * for Serde and Rocket when creating data
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderDTO {
    customer: String,
    products: Vec<super::ProductModels::ProductDTO>,
    due_date: String,
}

//DB Order will handle data fetched from the Database with an ID, whereas Order will just handle regular data
#[derive(Debug, Serialize, Deserialize)]
pub struct DBOrder {
    id: String,
    customer: String,
    products: OrderProducts,
    removed: bool,
    due_date: Datetime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    customer: String,
    products: OrderProducts,
    removed: bool,
    due_date: Datetime,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum OrderProducts {
    Populated(Vec<ProductModels::DBProduct>),
    Creating(Vec<ProductModels::ProductDTO>),
}

//Not sure if redacting or returning to Trait implementatinos of DB
#[rocket::async_trait]
impl DBInteractions<DBOrder> for DBOrder {
    async fn find(db: &SurrealRepo) -> Result<DBOrder, surrealdb::Error> {
        todo!()
    }

    async fn find_where(db: &SurrealRepo) -> Result<Vec<DBOrder>, surrealdb::Error> {
        todo!()
    }

    async fn find_all(db: &SurrealRepo) -> Result<Vec<DBOrder>, surrealdb::Error> {
        let query = db.find(None, "orders").await;
        return match query {
            Ok(query) => DBOrder::default_find_all(query),
            Err(e) => Err(e),
        };
    }

    async fn create(db: &SurrealRepo) -> Result<bool, surrealdb::Error> {
        todo!()
    }

    async fn update(db: &SurrealRepo) -> Result<bool, surrealdb::Error> {
        todo!()
    }
}

impl DBOrder {
    async fn get_created_by(
        db: &SurrealRepo,
        order_no: u32,
    ) -> Result<Option<DBUser>, surrealdb::Error> {
        let order_id = format!("orders:{order_no}");
        let results = db.find(Some("->created->user.*"), &order_id).await;
        return match results {
            Ok(find_output) => {
                let find_result = find_output[0].output().unwrap();
                let find_string = find_result.to_string();
                //This probably doesnt work from other tests
                let user: Vec<DBUser> =
                    serde_json::from_str(&find_string).expect("Failed to parse into user data");
                Ok(user.into_iter().nth(0))
            }
            Err(_) => panic!("Failed to find user that created order"),
        };
    }
}

impl Order {
    pub fn new(order: OrderDTO) -> Self {
        let due_date: &str = &order.due_date;
        Order {
            customer: order.customer,
            products: OrderProducts::Creating(order.products),
            removed: false,
            due_date: Datetime::from(due_date),
        }
    }
}
