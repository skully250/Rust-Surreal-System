use std::env;

use crate::repository::SurrealRepo::SurrealRepo;
use jsonwebtoken::{decode, DecodingKey, TokenData, Validation};
use rocket::{
    http::Status,
    request::{self, FromRequest, Request},
    State,
};
use serde::{Deserialize, Serialize};

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

async fn get_role(db: &SurrealRepo, token: &TokenData<Claims>) -> String {
    let where_statement = format!("username = '{0}'", token.claims.sub);
    let db_query = db
        .find_where(Some("role"), "users", &where_statement)
        .await
        .unwrap();
    println!("{:?}", db_query);
    let user = serde_json::json!(db_query);

    //Might move this into a struct rather than destructuring the json like this
    return user[0]["result"][0]["role"].as_str().unwrap().to_string();
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
                println!("{:?}", user_role);
                if user_role != "User" {
                    return request::Outcome::Failure((
                        Status::Unauthorized,
                        "Failed to validate user",
                    ));
                }

                return request::Outcome::Success(AuthUser {
                    user: token.claims.sub,
                });
            }
            Err(e) => {
                return request::Outcome::Failure((Status::Unauthorized, e));
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
                if user_role != "Admin" {
                    return request::Outcome::Failure((
                        Status::Unauthorized,
                        "Failed to validate user",
                    ));
                }

                return request::Outcome::Success(AuthAdmin {
                    user: token.claims.sub,
                });
            }
            Err(e) => {
                return request::Outcome::Failure((Status::Unauthorized, e));
            }
        }
    }
}