use rocket::{http::Status, serde::DeserializeOwned};
use surrealdb::sql::Value;

//Possibly a poor way to translate from DB Return data to Struct data to ensure correctness

/// Translates a SurrealDB Query into JSON and then into a Deserializable Data Type to ensure Correctness
///
/// # Panics
/// This function panics if it fails trying to convert to T
///
pub fn query_translate<T>(value: &Value) -> Result<T, Status>
where
    T: DeserializeOwned,
{
    let json = serde_json::json!(value);
    let typed_data = serde_json::from_value(json);
    return match typed_data {
        Ok(data) => Ok(data),
        Err(_) => Err(Status::BadRequest),
    };
}