use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;

//Actions

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    pub finished_by: u16,
    pub time: Datetime,
}

#[derive(Serialize, Deserialize)]
pub struct ActionDTO<'a> {
    pub order_id: &'a str,
    pub index: u8,
    pub action: Action,
}

//Models

//Using strings to include measurements and symbols ie
//32x32m || 32cm x 10m || 32x10x30 || 32kg || 320g
#[derive(Debug, Serialize, Deserialize)]
pub struct DBModel {
    id: String,
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
pub struct DBProduct {
    index: u8,
    model: DBModel,
    actions: Option<HashMap<String, Action>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductDTO {
    //More than 255 products seems a bit excessive
    index: u8,
    model: String,
}
