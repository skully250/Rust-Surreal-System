use std::str::FromStr;

use rocket::http::Status;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use surrealdb::sql::Thing;

use crate::repository::SurrealRepo::{PopulatedValue, DB};

use super::ActionModels::Action;

//Using strings to include measurements and symbols ie
//32x32m || 32cm x 10m || 32x10x30 || 32kg || 320g
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<Thing>,
    pub name: String,
    price: u32,
    weight: String,
    size: String,
}

impl ProductModel {
    pub fn new(name: String, price: u32, weight: String, size: String) -> Self {
        return ProductModel {
            id: None,
            name: name,
            price: price,
            weight: weight,
            size: size
        }
    }

    pub async fn find_active() -> Vec<Self> {
        let mut query = DB.query("SELECT * FROM type::table(models) WHERE active = true").await.unwrap();
        return query.take(0).expect("No models found");
    }

    pub async fn set_active(product_id: &str, active: bool) -> Result<&str, Status> {
        let query_string = format!("UPDATE models:{0} SET active = {1}", product_id, active);
        let query = DB.query(query_string).await;
        return match query {
            Ok(_) => Ok("Successfully modified model"),
            Err(_) => Err(Status::InternalServerError),
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum ProductQuantity {
    Single(Action),
    Multiple(Vec<Action>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<Thing>,
    orderNo: u32,
    model: PopulatedValue<ProductModel>,
    //TODO: Update this to conform with new graph edges
    actions: Option<ProductQuantity>,
    //JSON Data that can be read by a customized frontend for product differences
    customizations: Option<Value>,
}

impl Product {
    pub fn new(orderNo: u32, model: &str, customizations: Option<Value>) -> Result<Self, ()> {
        let model_thing = Thing::from_str(model)?;
        return Ok(Product {
            id: None,
            orderNo: orderNo,
            model: PopulatedValue::Unpopulated(model_thing),
            actions: None,
            customizations: customizations
        });
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductDTO {
    orderNo: u32,
    model: String,
    customizations: Option<Value>,
}
