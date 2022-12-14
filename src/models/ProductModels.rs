use serde::{Serialize, Deserialize};
use surrealdb::sql::Datetime;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionTime {
    finished: Datetime,
    updated: Datetime
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    name: String,
    finished_by: u16,
    time: ActionTime
}

//Using strings to include measurements and symbols ie
//32x32m || 32cm x 10m || 32x10x30 || 32kg || 320g
#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    id: String,
    name: String,
    price: u32,
    weight: String,
    size: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DBProduct {
    model: Model,
}

//TODO: Re-add Actions into the system using graph edges
#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    model: ProductModel
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum ProductModel {
    Depopulated(String),
    Populated(Model)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelDTO {
    pub name: String,
    price: u32,
    weight: String,
    size: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductDTO {
    model: String
}