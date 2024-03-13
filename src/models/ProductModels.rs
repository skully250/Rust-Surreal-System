use serde::{Deserialize, Serialize};
use surreal_simple_querybuilder::prelude::*;
use surreal_simple_querybuilder::model;
use surrealdb::sql::{Datetime, Id, Thing};
use rocket::serde::json::Json;

use crate::repository::SurrealRepo::DB;
use crate::util::responders::ApiResult;

//Using strings to include measurements and symbols ie
//32x32m || 32cm x 10m || 32x10x30 || 32kg || 320g
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    pub name: String,
    pub price: u32,
    pub weight: String,
    pub size: String
}

//PModel - Product Model
#[allow(non_upper_case_globals)]
model!(PModel {
    id,
    pub name,
    pub price,
    pub weight,
    pub size
});

impl PModel {
    pub async fn find_all() -> ApiResult<Vec<Self>> {
        use schema::model as pmodel;
        let (query, _params) = select("*" , &pmodel, ())?;
        let items: Option<Vec<PModel>> = DB.query(query).await?.take(0)?;

        return Ok(items.unwrap());
    }

    pub async fn find_name(name: &str) -> ApiResult<Self> {
        use schema::model as pmodel;
        let (query, params) = select("*", &pmodel, (Where((pmodel.name, name))))?;
        let item: Option<PModel> = DB.query(query).bind(params).await?.take(0)?;

        return Ok(item.unwrap());
    }
}

/*#[derive(Debug, Serialize, Deserialize)]
pub struct DBModel {
    id: Thing,
    name: String,
    price: u32,
    weight: String,
    size: String,
}*/

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
    customizations: Option<Json>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductDTO {
    orderNo: u32,
    model: String,
    customizations: Option<Json>
}
