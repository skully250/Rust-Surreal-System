use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Umbrella {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub model: Option<ObjectId>,
    pub base: Option<ObjectId>,
    pub valance: Option<ObjectId>,
    pub frameColour: Option<ObjectId>,
    pub canopyColour: Option<ObjectId>,
    pub canopyColourTwo: Option<ObjectId>,
    pub assembled: i16,
    pub packaged: i16,
    pub sewed: i16,
    pub cut: i16,
    pub assembledWhen: f64,
    pub packagedWhen: f64,
    pub sewedWhen: f64,
    pub cutWhen: f64,
    pub __v: i32
}