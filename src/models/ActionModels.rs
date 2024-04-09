use std::fmt::Display;

use rocket::tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};

use crate::{repository::SurrealRepo, util::JsonUtil::MyThing};

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<MyThing>,
    pub name: String,
    pub active: bool,
}

impl Action {
    pub fn new(name: &str, active: bool) -> Self {
        return Action {
            id: None,
            name: name.to_string(),
            active: active
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionDetails {
    pub finished_by: u16,
    pub time: Datetime,
}

#[derive(Serialize, Deserialize)]
pub struct ActionDTO {
    pub employee_id: String,
    pub product_id: String
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ActionEnum {
    emp_id(u32),
    datetime(String)
}

/*
 * From: User performing the Action
 * To: Order the action is being performed on
 * Action: Action being performed either populated or ID
 */
 #[derive(Debug, Serialize, Deserialize)]
 pub struct Actioned {
     pub from: MyThing,
     pub to: MyThing,
     pub action: SurrealRepo::PopulatedValue<ActionDetails>
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