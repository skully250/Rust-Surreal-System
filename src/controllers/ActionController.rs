use rocket::{http::Status, State};

use crate::{
    models::ActionModels::{Action, ActionList}, util::responders::{ApiResult, JsonStatus, Jsonstr}, SurrealRepo
};

pub async fn get_actions() -> ApiResult<Vec<Action>> {
    let query: Result<Vec<Action>, surrealdb::Error> = SurrealRepo::find_all("actions").await;
    return match query {
        Ok(query_result) => Ok(query_result),
        Err(_) => Err(Status::BadRequest),
    };
}

pub async fn create_action<'a>(
    actions: &State<ActionList>,
    action_name: &str,
) -> Jsonstr<'a> {
    let query = SurrealRepo::create_named(
        "actions",
        &action_name,
        Action::new(action_name, true),
    )
    .await;
    println!("${:?}", query);
    return match query {
        Ok(_query_result) => {
            let mut actions = actions.actions.write().await;
            actions.push(action_name.to_string());
            Ok(JsonStatus::success("Successfully created action"))
        }
        Err(_) => Err(Status::BadRequest),
    };
}

pub async fn update_action<'a>(
    action_list: &State<ActionList>,
    action_name: &str,
    active: bool,
) -> Jsonstr<'a> {
    let action_details = Action::new(action_name, active);
    let act_name = action_details.name.clone();
    let query = SurrealRepo::update("actions", &act_name, action_details).await;
    return match query {
        Ok(_) => {
            let mut actions = action_list.actions.write().await;

            let index = actions.iter().position(|action| action.eq(&act_name));

            match active {
                true => {
                    if index.is_some() {
                        return Ok(JsonStatus::custom(Status::BadRequest, false, "Action already Active"));
                    } else {
                        actions.push(act_name);
                        return Ok(JsonStatus::success("Successfully activated action"));
                    }
                }
                false => {
                    if index.is_some() {
                        actions.remove(index.unwrap());
                        return Ok(JsonStatus::success("Action Archived"));
                    } else {
                        return Ok(JsonStatus::custom(Status::NotFound, false, "Action Doesnt exist"));
                    }
                }
            }
        }
        Err(_) => Err(Status::BadRequest),
    };
}

//Action events and graph edges down here.