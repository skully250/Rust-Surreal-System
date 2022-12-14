use pbkdf2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, SaltString},
    Pbkdf2,
};
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize)]
pub struct DBCustomer {
    id: String,
    name: String,
    email: String,
    address: Address,
    mobile_number: Option<Phone>,
    phone_number: Option<Phone>,
}

//Will be expanded upon later to include additional details
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerDTO {
    name: String,
    email: String,
    address: Address,
    mobile_number: Option<Phone>,
    phone_number: Option<Phone>,
}

//Will add more metadata to users later as expectations expand
#[derive(Serialize, Deserialize)]
pub struct DBUser {
    id: String,
    username: String,
    salt: String,
    hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    salt: String,
    hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserDTO {
    pub username: String,
    pub password: String,
}

impl User {
    pub fn new(user: UserDTO) -> Self {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Pbkdf2
            .hash_password(user.password.as_bytes(), &salt)
            .expect("Failed to hash password")
            .to_string();
        User {
            username: user.username,
            salt: salt.to_string(),
            hash: password_hash,
        }
    }
}
