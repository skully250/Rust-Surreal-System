use rocket::{
    http::{Cookie, CookieJar, Status},
    time::{Duration, OffsetDateTime},
};

use crate::{
    models::UserModels::{self, User, UserDTO},
    repository::SurrealRepo,
    util::{responders::{ApiResult, JsonStatus, Jsonstr}, AuthUtil},
};

pub async fn get_users() -> ApiResult<Vec<User>> {
    let query: Result<Vec<User>, surrealdb::Error> = SurrealRepo::find_all("users").await;
    return match query {
        Ok(query) => Ok(query),
        Err(_) => Err(Status::InternalServerError),
    };
}

pub async fn add_user<'a>(user: UserDTO) -> Jsonstr<'a> {
    let new_user = UserModels::User::new(user);
    let username = new_user.username.to_owned();
    let query: Result<User, surrealdb::Error> =
        SurrealRepo::create_named("users", &username, new_user).await;
    return match query {
        Ok(_) => Ok(JsonStatus::success("Successfully created user")),
        Err(_) => Err(Status::InternalServerError),
    };
}

pub async fn edit_user(user: UserDTO, user_id: &str) -> Jsonstr {
    let new_user = UserModels::User::new(user);
    let query = SurrealRepo::update("users", &user_id, new_user).await;
    return match query {
        Ok(_) => Ok(JsonStatus::success("Successfully edited user")),
        Err(_) => Err(Status::InternalServerError),
    };
}

pub async fn delete_user(user_id: &str) -> Jsonstr {
    //Potential: Set to inactive, or delete all information but retain name for graph edge purposes
    let query: Result<User, surrealdb::Error> = SurrealRepo::delete("users", &user_id).await;
    return match query {
        Ok(_) => Ok(JsonStatus::success("Successfully removed user")),
        Err(_) => Err(Status::InternalServerError),
    };
}

//Login Functions

pub async fn login_user<'a>(
    cookies: &CookieJar<'_>,
    user: UserDTO,
) -> Jsonstr<'a> {
    let db_query: Result<User, surrealdb::Error> =
        SurrealRepo::find("users", &user.username).await;
    //println!("{:?}", &found_user.into());
    match db_query {
        Ok(found_user) => {
            let password_compare =
                match argon2::verify_encoded(&found_user.hash, user.password.as_bytes()) {
                    Ok(result) => result,
                    Err(_) => {
                        return Err(Status::InternalServerError);
                    }
                };

            if !password_compare {
                return Err(Status::InternalServerError);
            }

            let new_jwt = AuthUtil::create_jwt(&found_user.username, &found_user.role)
                .expect("Failed to create JWT");
            let expires = OffsetDateTime::now_utc() + Duration::weeks(2);
            let cookie = Cookie::build(("token", new_jwt)).expires(expires);
            cookies.add(cookie);
            Ok(JsonStatus::success("Successfully logged in"))
        }
        Err(_) => Ok(JsonStatus::failure("Could not fetch user")),
    }
}
