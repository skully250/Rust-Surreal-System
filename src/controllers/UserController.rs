use rocket::{
    http::{Cookie, CookieJar, Status},
    time::{Duration, OffsetDateTime},
};

use crate::{
    models::UserModels::{self, DBUser, User, UserDTO},
    util::{responders::JsonStatus, AuthUtil},
    SurrealRepo,
};

pub async fn get_users(db: &SurrealRepo) -> Result<Vec<DBUser>, Status> {
    let query: Result<Vec<DBUser>, surrealdb::Error> = db.find_all("users").await;
    return match query {
        Ok(query) => Ok(query),
        Err(_) => Err(Status::InternalServerError),
    };
}

pub async fn add_user(db: &SurrealRepo, user: UserDTO) -> Result<JsonStatus<&str>, Status> {
    let new_user = UserModels::User::new(user);
    let username = new_user.username.to_owned();
    let query: Result<User, surrealdb::Error> = db.create_named("users", &username, new_user).await;
    return match query {
        Ok(_) => Ok(JsonStatus {
            status_code: Status::Ok,
            status: true,
            message: "Successfully created user",
        }),
        Err(_) => Err(Status::InternalServerError),
    };
}

pub async fn edit_user(
    db: &SurrealRepo,
    user: UserDTO,
    user_id: String,
) -> Result<JsonStatus<&str>, Status> {
    let new_user = UserModels::User::new(user);
    let query = db.update("users", &user_id, new_user).await;
    return match query {
        Ok(_) => Ok(JsonStatus {
            status_code: Status::Ok,
            status: true,
            message: "Successfully edited user",
        }),
        Err(_) => Err(Status::InternalServerError),
    };
}

pub async fn delete_user(db: &SurrealRepo, user_id: String) -> Result<JsonStatus<&str>, Status> {
    let query: Result<DBUser, surrealdb::Error> = db.delete("users", &user_id).await;
    return match query {
        Ok(_) => Ok(JsonStatus {
            status_code: Status::Ok,
            status: true,
            message: "Successfully removed user",
        }),
        Err(_) => Err(Status::InternalServerError),
    };
}

//Login Functions

pub async fn login_user<'a>(
    db: &SurrealRepo,
    cookies: &CookieJar<'_>,
    user: UserDTO,
) -> Result<JsonStatus<&'a str>, Status> {
    let mut db_query: Result<DBUser, surrealdb::Error> = db.find("users", &user.username).await;
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
            Ok(JsonStatus {
                status_code: Status::Ok,
                status: true,
                message: "Successfully logged in",
            })
        }
        Err(err) => Ok(JsonStatus {
            status_code: Status::InternalServerError,
            status: false,
            message: "Could not Fetch User",
        }),
    }
}
