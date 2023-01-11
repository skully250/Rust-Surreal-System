use rocket::{
    http::{Cookie, CookieJar, Status},
    time::{Duration, OffsetDateTime},
};
use surrealdb::sql::Value;

use crate::{
    models::UserModels::{self, DBUser, UserDTO},
    util::{responders::JsonStatus, AuthUtil},
    SurrealRepo,
};

pub async fn get_users(db: &SurrealRepo) -> Result<Vec<UserModels::DBUser>, Status> {
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
                Err(Status::BadRequest)
            }
        }
        Err(_) => Err(Status::InternalServerError),
    };
}

pub async fn add_user(db: &SurrealRepo, user: UserDTO) -> Result<JsonStatus<&str>, Status> {
    let new_user = UserModels::User::new(user);
    let username = new_user.username.to_owned();
    let query = db.create("users", new_user, Some(username)).await;
    return match query {
        Ok(query) => {
            let query_entry = query[0].output();
            if query_entry.is_ok() {
                Ok(JsonStatus {
                    status_code: Status::Ok,
                    status: true,
                    message: "Successfully created user",
                })
            } else {
                Err(Status::BadRequest)
            }
        }
        Err(_) => Err(Status::InternalServerError),
    };
}

pub async fn edit_user(
    db: &SurrealRepo,
    user: UserDTO,
    user_id: String,
) -> Result<JsonStatus<&str>, Status> {
    let query = db.update(&user_id, user).await;
    return match query {
        Ok(query) => {
            let empty_query = query[0].output().unwrap().first().is_none();
            if !empty_query {
                Ok(JsonStatus {
                    status_code: Status::Ok,
                    status: true,
                    message: "Successfully edited user",
                })
            } else {
                Ok(JsonStatus {
                    status_code: Status::NotFound,
                    status: false,
                    message: "User doesnt exist",
                })
            }
        }
        Err(_) => Err(Status::InternalServerError),
    };
}

pub async fn delete_user(db: &SurrealRepo, user_id: String) {
    
}

//Login Functions

pub async fn login_user<'a>(
    db: &SurrealRepo,
    cookies: &CookieJar<'_>,
    user: UserDTO,
) -> Result<JsonStatus<&'a str>, Status> {
    let user_query = format!("users:{0}", user.username);
    let db_query = db
        .find(None, &user_query)
        .await
        .expect("Failed to query Database");
    let user_select = serde_json::json!(db_query[0].output().unwrap().first());
    println!("{:?}", user_select);
    let found_user: DBUser =
        serde_json::from_value(user_select).expect("Failed to parse user from DB");
    let password_compare = match argon2::verify_encoded(&found_user.hash, user.password.as_bytes())
    {
        Ok(result) => result,
        Err(_) => {
            return Err(Status::InternalServerError);
        }
    };

    if !password_compare {
        return Err(Status::InternalServerError);
    }

    let new_jwt =
        AuthUtil::create_jwt(&found_user.username, &found_user.role).expect("Failed to create JWT");
    let expires = OffsetDateTime::now_utc() + Duration::weeks(2);
    let cookie = Cookie::build("token", new_jwt).expires(expires).finish();
    cookies.add(cookie);
    Ok(JsonStatus {
        status_code: Status::Ok,
        status: true,
        message: "Successfully logged in",
    })
}