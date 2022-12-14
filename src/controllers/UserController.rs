use surrealdb::sql::Value;

use crate::{
    models::UserModels::{self, UserDTO},
    util::responders::{JsonMessage, RequestResponse, ServerMessage},
    SurrealRepo,
};

pub async fn get_users(db: &SurrealRepo) -> Result<Vec<UserModels::DBUser>, RequestResponse> {
    let query = db.find(None, "users").await;
    return match query {
        Ok(query) => {
            let query_result = query[0].output().unwrap();
            if let Value::Array(rows) = query_result {
                let users: Vec<UserModels::DBUser> =
                    serde_json::from_value(serde_json::json!(&rows))
                        .expect("Failed to parse user data");
                Ok(users)
            } else {
                Err(RequestResponse::BadRequest(ServerMessage::new(
                    JsonMessage {
                        status: false,
                        message: "Error fetching users from DB".to_string(),
                    },
                )))
            }
        }
        Err(e) => Err(RequestResponse::InternalErrorRequest(ServerMessage::new(
            JsonMessage {
                status: false,
                message: e.to_string(),
            },
        ))),
    };
}

pub async fn add_user(db: &SurrealRepo, user: UserDTO) -> Result<RequestResponse, RequestResponse> {
    let new_user = UserModels::User::new(user);
    let username = new_user.username.to_owned();
    let query = db.create("users", new_user, Some(username)).await;
    return match query {
        Ok(query) => {
            let query_entry = query[0].output();
            if query_entry.is_ok() {
                Ok(RequestResponse::OKRequest(ServerMessage::new(
                    JsonMessage {
                        status: false,
                        message: "Successfully created user".to_string(),
                    },
                )))
            } else {
                Err(RequestResponse::BadRequest(ServerMessage::new(
                    JsonMessage {
                        status: false,
                        message: "Error creating user in DB".to_string(),
                    },
                )))
            }
        }
        Err(e) => Err(RequestResponse::InternalErrorRequest(ServerMessage::new(
            JsonMessage {
                status: false,
                message: e.to_string(),
            },
        ))),
    };
}
