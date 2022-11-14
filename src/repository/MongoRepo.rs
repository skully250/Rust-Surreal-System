use mongodb::{sync::{Client, Collection}, options::ClientOptions};
use std::env;
extern crate dotenv;
use dotenv::dotenv;

use crate::models::umbrella_model::Umbrella;

pub struct MongoRepo {
    col: Collection<Umbrella>
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let options = ClientOptions::parse(uri).expect("Could not parse URI from .env");
        let client = Client::with_options(options).unwrap();
        let db = client.database("testingSystem");
        let col: Collection<Umbrella> = db.collection("umbrellas");
        return MongoRepo { col };
    }

    pub fn get_umbrellas(&self) -> Result<Vec<Umbrella>, mongodb::error::Error> {
        let cursors = self.col.find(None, None).ok().expect("Error getting list of users");
        let umbrellas = cursors.map(|doc| doc.unwrap()).collect();
        return Ok(umbrellas)
    }
}