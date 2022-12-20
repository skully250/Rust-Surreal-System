use std::env;

use super::responders::{JsonMessage, RequestResponse, ServerMessage};
use crate::models::{AuthModels::Claims, UserModels::UserRole};
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

pub fn create_jwt(uid: &str, role: &UserRole) -> Result<String, RequestResponse> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::weeks(2))
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
    .map_err(|_| {
        RequestResponse::BadRequest(
            (ServerMessage::new(JsonMessage {
                status: false,
                message: "Failed to create JWT".to_string(),
            })),
        )
    });
}