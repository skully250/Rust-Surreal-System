use serde::{Deserialize, Serialize};
use serde_json::Value;
use surrealdb::sql::Thing;

use super::ActionModels::Action;

//Using strings to include measurements and symbols ie
//32x32m || 32cm x 10m || 32x10x30 || 32kg || 320g
#[derive(Debug, Serialize, Deserialize)]
pub struct DBModel {
    id: Thing,
    name: String,
    price: u32,
    weight: String,
    size: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelDTO {
    pub name: String,
    price: u32,
    weight: String,
    size: String,
}

//Products
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum ProductModel {
    Depopulated(String),
    Populated(DBModel),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum ProductQuantity {
    Single(Action),
    Multiple(Vec<Action>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DBProduct {
    id: Thing,
    orderNo: u32,
    model: DBModel,
    //TODO: Update this to conform with new graph edges
    actions: Option<ProductQuantity>,
    //JSON Data that can be read by a customized frontend for product differences
    customizations: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductDTO {
    orderNo: u32,
    model: String,
    customizations: Option<Value>,
}
