use std::env;

use jsonwebtoken::{decode, DecodingKey, Validation};
use rocket::request::{self, FromRequest, Request};

use serde::{Deserialize, Serialize};

use crate::util::responders::JsonStatus;

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthUser {
    pub user: String,
    role: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthUser {
    type Error = JsonStatus<'r>;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let cookies = req.cookies();
        let user_cookie = match cookies.get("token") {
            Some(result) => result,
            None => return request::Outcome::Forward(()),
        };
        let user_token = user_cookie.value();
        let decoded = decode::<Claims>(
            user_token,
            &DecodingKey::from_secret(env::var("JWTSECRET").unwrap().as_bytes()),
            &Validation::new(jsonwebtoken::Algorithm::HS512),
        )
        .expect("Failed to verify token");
        return request::Outcome::Success(AuthUser {
            user: decoded.claims.sub,
            role: decoded.claims.role,
        });
    }
}
