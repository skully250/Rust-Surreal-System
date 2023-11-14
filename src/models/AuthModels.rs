use std::env;

use crate::repository::SurrealRepo::SurrealRepo;
use jsonwebtoken::{decode, DecodingKey, TokenData, Validation};
use rocket::{
    http::Status,
    request::{self, FromRequest, Request},
    State,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthUser {
    pub user: String,
}

pub struct AuthAdmin {
    pub user: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}

fn grab_token<'a>(req: &Request) -> Result<TokenData<Claims>, &'a str> {
    let cookies = req.cookies();
    let user_cookie = match cookies.get("token") {
        Some(result) => result,
        None => {
            return Err("Failed to validate user");
        }
    };
    let user_token = user_cookie.value();
    let decoded = decode::<Claims>(
        user_token,
        &DecodingKey::from_secret(env::var("JWTSECRET").unwrap().as_bytes()),
        &Validation::new(jsonwebtoken::Algorithm::HS512),
    )
    .expect("Failed to verify token");
    return Ok(decoded);
}

async fn get_role<'a>(db: &SurrealRepo, token: &TokenData<Claims>) -> Result<String, &'a str> {
    let where_statement = format!("username = '{0}'", token.claims.sub);
    let mut db_query = db
        .find_where("users", Some("role"), &where_statement)
        .await
        .unwrap();
    println!("{:?}", db_query);

    let user: Option<Value> = db_query.take("result").unwrap();

    //Might move this into a struct rather than destructuring the json like this
    match user {
        Some(user) => {
            return Ok(user[0]["role"].as_str().unwrap().to_string());
        }
        None => {
            return Err("Error fetching user");
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthUser {
    type Error = &'r str;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token = grab_token(req);
        let db = req
            .guard::<&State<SurrealRepo>>()
            .await
            .expect("DB not found");

        match token {
            Ok(token) => {
                let user_role = get_role(db, &token).await;
                match user_role {
                    Ok(user_role) => {
                        println!("{:?}", user_role);
                        if (user_role != "User") && (user_role != "Admin") {
                            return request::Outcome::Error((
                                Status::Unauthorized,
                                "Failed to validate user",
                            ));
                        }
                    }
                    Err(error) => {
                        return request::Outcome::Error((Status::Unauthorized, error));
                    }
                }

                return request::Outcome::Success(AuthUser {
                    user: token.claims.sub,
                });
            }
            Err(e) => {
                return request::Outcome::Error((Status::Unauthorized, e));
            }
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthAdmin {
    type Error = &'r str;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token = grab_token(req);
        let db = req
            .guard::<&State<SurrealRepo>>()
            .await
            .expect("DB not found");

        match token {
            Ok(token) => {
                let user_role = get_role(db, &token).await;
                match user_role {
                    Ok(user_role) => {
                        if user_role != "Admin" {
                            return request::Outcome::Error((
                                Status::Unauthorized,
                                "Failed to validate user",
                            ));
                        }
                    }
                    Err(error) => {
                        return request::Outcome::Error((Status::Unauthorized, error));
                    }
                }

                return request::Outcome::Success(AuthAdmin {
                    user: token.claims.sub,
                });
            }
            Err(e) => {
                return request::Outcome::Error((Status::Unauthorized, e));
            }
        }
    }
}
