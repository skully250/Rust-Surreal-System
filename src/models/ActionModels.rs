use std::fmt::Display;

use rocket::tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};

use crate::repository::SurrealRepo;

#[derive(Debug, Serialize, Deserialize)]
pub struct DBAction {
    pub id: Thing,
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
    pub action: Action,
}

/*
 * From: User performing the Action
 * To: Order the action is being performed on
 * Action: Action being performed either populated or ID
 */
 #[derive(Debug, Serialize, Deserialize)]
 pub struct Actioned {
     pub from: Thing,
     pub to: Thing,
     pub action: SurrealRepo::PopulatedValue<Action>
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