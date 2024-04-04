use std::env;

use crate::models::{AuthModels::Claims, UserModels::UserRole};
use chrono::Utc;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use rocket::http::Status;

pub fn create_jwt(uid: &str, role: &UserRole) -> Result<String, Status> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::try_weeks(2).unwrap())
        .expect("Valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: uid.to_owned(),
        role: role.into(),
        exp: expiration as usize,
    };

    let header = Header::new(Algorithm::HS512);

    return encode(
        &header,
        &claims,
        &EncodingKey::from_secret(env::var("JWTSECRET").unwrap().as_bytes()),
    )
    .map_err(|_| Status::BadRequest);
}
