use rocket::http::{Cookie, CookieJar};
use surrealdb::sql::Value;

use crate::{
    models::UserModels::{self, DBUser, UserDTO},
    util::{
        responders::{JsonMessage, RequestResponse, ServerMessage},
        AuthUtil,
    },
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

pub async fn login_user(
    db: &SurrealRepo,
    cookies: &CookieJar<'_>,
    user: UserDTO,
) -> Result<RequestResponse, RequestResponse> {
    let user_query = format!("users:{0}", user.username);
    let DBQuery = db
        .find(None, &user_query)
        .await
        .expect("Failed to query Database");
    let user_select = serde_json::json!(DBQuery[0].output().unwrap().first());
    println!("{:?}", user_select);
    let found_user: DBUser =
        serde_json::from_value(user_select).expect("Failed to parse user from DB");
    let password_compare = match argon2::verify_encoded(&found_user.hash, user.password.as_bytes())
    {
        Ok(result) => result,
        Err(_) => {
            return Err(RequestResponse::BadRequest(ServerMessage::new(
                JsonMessage {
                    status: false,
                    message: "Error running verify function".to_string(),
                },
            )));
        }
    };

    if !password_compare {
        return Err(RequestResponse::BadRequest(ServerMessage::new(
            JsonMessage {
                status: false,
                message: "Failed to verify PW".to_string(),
            },
        )));
    }

    let new_jwt =
        AuthUtil::create_jwt(&found_user.username, &found_user.role).expect("Failed to create JWT");
    cookies.add(Cookie::new("token", new_jwt));
    Ok(RequestResponse::OKRequest(ServerMessage::new(
        JsonMessage {
            status: true,
            message: "Successfully logged in".to_string(),
        },
    )))
}
