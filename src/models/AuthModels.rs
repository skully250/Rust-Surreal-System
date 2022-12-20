use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize
}