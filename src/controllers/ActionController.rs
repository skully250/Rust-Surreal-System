use rocket::{http::Status, State};
use serde::{Deserialize, Serialize};

use crate::{
    models::ProductModels::{ActionDTO, ActionList, DBAction},
    util::responders::JsonStatus,
    SurrealRepo,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct ActionDetails {
    name: String,
    active: bool,
}

pub async fn action_product<'a>(
    db: &SurrealRepo,
    order_id: String,
    action: ActionDTO,
) -> Result<(Status, &'a str), Status> {
    let data = serde_json::json!(action.action).to_string();
    let query = format!(
        "UPDATE {0} SET products[WHERE index = {1}].actions.{2} = {3}",
        order_id, action.index, action.action_name, data
    );
    println!("{:?}", query);
    let query_result = db.query(&query).await;
    return match query_result {
        Ok(_query) => Ok({
            println!("{:?}", _query);
            (Status::Ok, "Action successfully run")
        }),
        Err(_e) => Err(Status::BadRequest),
    };
}

pub async fn get_actions(db: &SurrealRepo) -> Result<Vec<DBAction>, Status> {
    let query: Result<Vec<DBAction>, surrealdb::Error> = db.find_all("actions").await;
    return match query {
        Ok(query_result) => Ok(query_result),
        Err(_) => Err(Status::BadRequest),
    };
}

pub async fn create_action<'a>(
    db: &SurrealRepo,
    actions: &State<ActionList>,
    action_name: &str,
) -> Result<JsonStatus<&'a str>, Status> {
    let query = db
        .create_named(
            "actions",
            &action_name,
            ActionDetails {
                name: action_name.to_owned(),
                active: true,
            },
        )
        .await;
    return match query {
        Ok(_query_result) => {
            let mut actions = actions.actions.write().await;
            actions.push(action_name.to_string());
            Ok(JsonStatus {
                status_code: Status::Ok,
                status: true,
                message: "Successfully created test action",
            })
        }
        Err(_) => Err(Status::BadRequest),
    };
}

pub async fn update_action<'a>(
    db: &SurrealRepo,
    action_list: &State<ActionList>,
    action_name: String,
    active: bool,
) -> Result<JsonStatus<&'a str>, Status> {
    let action_details = ActionDetails {
        name: action_name,
        active: active,
    };
    let act_name = action_details.name.clone();
    let query = db.update("actions", &act_name, action_details).await;
    return match query {
        Ok(_) => {
            let mut actions = action_list.actions.write().await;

            let index = actions.iter().position(|action| action.eq(&act_name));

            match active {
                true => {
                    if index.is_some() {
                        return Ok(JsonStatus {
                            status_code: Status::NotModified,
                            status: true,
                            message: "Action already active",
                        });
                    } else {
                        actions.push(act_name);
                        return Ok(JsonStatus {
                            status_code: Status::Ok,
                            status: true,
                            message: "Successfully activated action",
                        });
                    }
                }
                false => {
                    if index.is_some() {
                        actions.remove(index.unwrap());
                        return Ok(JsonStatus {
                            status_code: Status::Ok,
                            status: true,
                            message: "Action archived",
                        });
                    } else {
                        return Ok(JsonStatus {
                            status_code: Status::NotFound,
                            status: false,
                            message: "Action doesnt exist",
                        });
                    }
                }
            }
        }
        Err(_) => Err(Status::BadRequest),
    };
}
