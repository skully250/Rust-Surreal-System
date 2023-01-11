use rocket::{http::Status, State};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Value;

use crate::{
    models::ProductModels::{ActionDTO, ActionList, DBAction},
    util::responders::JsonStatus,
    SurrealRepo,
};

#[derive(Serialize, Deserialize)]
pub struct ActionDetails {
    name: String,
    active: bool,
}

pub async fn action_product<'a>(
    db: &SurrealRepo,
    action_name: String,
    action: ActionDTO<'a>,
) -> Result<(Status, &'a str), Status> {
    let data = serde_json::json!(action.action).to_string();
    let query = format!(
        "UPDATE {0} SET products[WHERE index = {1}].actions.{2} = {3}",
        action.order_id, action.index, action_name, data
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
    let query = db.find(None, "actions").await;
    return match query {
        Ok(query_result) => {
            let result_entry = query_result[0].output().unwrap();
            if let Value::Array(rows) = result_entry {
                println!("{:?}", rows);
                let actions: Vec<DBAction> = serde_json::from_value(serde_json::json!(&rows))
                    .expect("Failed to parse actions");
                Ok(actions)
            } else {
                Err(Status::BadRequest)
            }
        }
        Err(_) => Err(Status::BadRequest),
    };
}

pub async fn create_action<'a>(
    db: &SurrealRepo,
    actions: &State<ActionList>,
    action_name: String,
) -> Result<JsonStatus<&'a str>, Status> {
    let action_details = ActionDetails {
        name: action_name,
        active: true,
    };
    let query = db
        .create("actions", serde_json::json!(action_details), None)
        .await;
    let mut actions = actions
        .actions
        .write()
        .expect("Could not open writeable reference");
    actions.push(action_details.name);
    return match query {
        Ok(_query_result) => Ok(JsonStatus {
            status_code: Status::Ok,
            status: true,
            message: "Successfully created test action",
        }),
        Err(_) => Err(Status::BadRequest),
    };
}

pub async fn update_action<'a>(
    db: &SurrealRepo,
    action_list: &State<ActionList>,
    action_name: String,
    active: bool,
) -> JsonStatus<&'a str> {
    let action_details = ActionDetails {
        name: action_name,
        active: active,
    };
    let where_statement = format!("name = '{0}'", action_details.name);
    let query = db
        .update_where(
            "actions",
            serde_json::json!(action_details),
            &where_statement,
        )
        .await
        .unwrap();

    let empty_query = query[0].output().unwrap().first().is_none();

    println!("{:?}", empty_query);

    let mut actions = action_list
        .actions
        .write()
        .expect("Could not open writeable reference");

    let index = actions
        .iter()
        .position(|action| action.eq(&action_details.name));

    match active {
        true => {
            if empty_query {
                return JsonStatus {
                    status_code: Status::NotFound,
                    status: false,
                    message: "Action doesnt exist",
                };
            }
            if index.is_some() {
                return JsonStatus {
                    status_code: Status::NotModified,
                    status: true,
                    message: "Action already active",
                };
            } else {
                actions.push(action_details.name);
                return JsonStatus {
                    status_code: Status::Ok,
                    status: true,
                    message: "Successfully activated action",
                };
            }
        }
        false => {
            if empty_query {
                return JsonStatus {
                    status_code: Status::NotFound,
                    status: false,
                    message: "Action doesnt exist",
                };
            }
            if index.is_some() {
                actions.remove(index.unwrap());
                return JsonStatus {
                    status_code: Status::Ok,
                    status: true,
                    message: "Action archived",
                };
            } else {
                return JsonStatus {
                    status_code: Status::NotFound,
                    status: false,
                    message: "Action doesnt exist",
                };
            }
        }
    }
}
