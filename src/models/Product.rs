use serde::{Serialize, Deserialize};
use surrealdb::sql::Id;

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    name: String,
    finished_by: u16
}

//Using strings to include measurements and symbols ie
//32x32m || 32cm x 10m || 32x10x30 || 32kg || 320g
#[derive(Serialize, Deserialize)]
pub struct Model {
    id: Id,
    name: String,
    price: u32,
    weight: String,
    size: String
}

#[derive(Serialize, Deserialize)]
pub struct Product {
    model: Model,
    actions: Vec<Action>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelDTO {
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductDTO {
    model: ModelDTO
}