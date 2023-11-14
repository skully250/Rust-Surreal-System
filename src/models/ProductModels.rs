use std::{collections::HashMap, fmt::Display};

use rocket::tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;

//Actions

#[derive(Debug, Serialize, Deserialize)]
pub struct DBAction {
    pub id: String,
    pub name: String,
    pub active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    pub finished_by: u16,
    pub time: Datetime,
}

#[derive(Serialize, Deserialize)]
pub struct ActionDTO {
    pub action_name: String,
    pub index: u8,
    pub action: Action,
}

pub struct ActionList {
    pub actions: RwLock<Vec<String>>,
}

impl Display for ActionList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let actions = self.actions.try_read();

        match actions {
            Ok(actions) => {
                let mut comma_string = String::new();
                for entry in actions.iter() {
                    comma_string.push_str(entry);
                    comma_string.push_str(", ");
                }
                return write!(f, "{}", comma_string);
            }
            Err(err) => {
                return write!(f, "Error occurred trying to read RWLock");
            }
        }
    }
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
#[serde(untagged)]
enum ProductQuantity {
    Single(Action),
    Multiple(Vec<Action>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DBProduct {
    index: u8,
    model: DBModel,
    actions: Option<HashMap<String, ProductQuantity>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductDTO {
    //More than 255 products seems a bit excessive
    index: u8,
    model: String,
}
