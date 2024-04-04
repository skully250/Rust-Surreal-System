use std::str::FromStr;

use crate::{
    repository::SurrealRepo::{self, PopulatedValue},
    util::JsonUtil::MyThing,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};

use super::{
    ProductModels::Product,
    UserModels::{Customer, User},
};

/*
 * The Order DTO is created as a way to create a parseable input
 * for Serde and Rocket when creating data
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderDTO {
    pub customer: String,
    //???
    pub products: Vec<Product>,
    due_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<MyThing>,
    orderNo: Option<u32>,
    customer: PopulatedValue<Customer>,
    pub products: Option<Vec<PopulatedValue<Product>>>,
    removed: bool,
    created_date: Datetime,
    due_date: Datetime,
}

impl Order {
    pub fn new(order: &OrderDTO) -> Self {
        let due_date: DateTime<Utc> = DateTime::from_str(order.due_date.as_str()).unwrap();
        let created_time: DateTime<Utc> = Utc::now();
        return Order {
            id: None,
            orderNo: None,
            customer: PopulatedValue::Unpopulated(Thing::from((
                "customers",
                order.customer.as_str(),
            ))),
            //Products are created during the process and added to the order
            products: None,
            removed: false,
            created_date: Datetime::from(created_time),
            due_date: Datetime::from(due_date),
        };
    }

    async fn get_created_by(order_no: u32) -> Result<User, surrealdb::Error> {
        let order_id = format!("orders:{order_no}");
        let results: Result<User, surrealdb::Error> =
            SurrealRepo::find("->created->user.*", &order_id).await;
        return match results {
            Ok(find_output) => Ok(find_output),
            Err(_) => panic!("Failed to find user that created order"),
        };
    }
}
