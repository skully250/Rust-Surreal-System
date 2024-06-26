use argon2::Config;
use rocket::http::Status;
use serde::{Deserialize, Serialize};

use crate::{
    repository::SurrealRepo::DB,
    util::{responders::ApiResult, JsonUtil::MyThing},
};

//Putting both customer and user in the same file since they both correlate to a person in the DB

#[derive(Debug, Serialize, Deserialize)]
enum NumberTypes {
    MOBILE,
    DEFAULT,
}

#[derive(Debug, Serialize, Deserialize)]
struct Phone {
    phone_type: NumberTypes,
    number: String,
    area_code: String,
    country_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
enum AddressType {
    POBOX,
    STREET,
    DELIVERY,
}

#[derive(Debug, Serialize, Deserialize)]
struct Address {
    address_type: AddressType,
    address_line: String,
    city: String,
    region: String,
    postal_code: String,
    country: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Customer {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<MyThing>,
    name: String,
    email: String,
    address: Address,
    mobile_number: Option<Phone>,
    phone_number: Option<Phone>,
}

impl Customer {
    pub async fn find_removed() -> Result<Vec<Self>, surrealdb::Error> {
        let mut query = DB
            .query("SELECT * FROM customers WHERE removed != false")
            .await
            .unwrap();
        return query.take(0);
    }

    pub async fn remove_customer(customer_id: &str) -> ApiResult<&str> {
        let query_string = format!("UPDATE customers:{customer_id} SET removed = true");
        let query = DB.query(query_string).await;
        return match query {
            Ok(_) => Ok("Successfully removed customer"),
            Err(_) => Err(Status::InternalServerError),
        };
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum UserRole {
    User,
    Admin,
}

impl From<&str> for UserRole {
    fn from(role: &str) -> Self {
        match role {
            "Admin" => UserRole::Admin,
            _ => UserRole::User,
        }
    }
}

impl From<&UserRole> for String {
    fn from(role: &UserRole) -> String {
        match role {
            UserRole::Admin => "Admin".to_string(),
            _ => "User".to_string(),
        }
    }
}

//Will add more metadata to users later as expectations expand
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<MyThing>,
    pub username: String,
    pub role: UserRole,
    salt: String,
    pub hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserDTO {
    pub username: String,
    pub password: String,
    pub role: Option<String>,
}

impl User {
    pub fn new(user: UserDTO) -> Self {
        let salt = b"saltstring";
        let config = Config::default();
        let hash = argon2::hash_encoded(user.password.as_bytes(), salt, &config).unwrap();
        let role: UserRole;
        if user.role.is_none() {
            role = UserRole::User;
        } else {
            role = UserRole::from(user.role.unwrap().as_str());
        }
        User {
            id: None,
            username: user.username,
            role: role,
            salt: String::from_utf8(salt.to_vec()).unwrap(),
            hash: hash,
        }
    }
}
