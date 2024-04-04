use rocket::{http::Status, serde::DeserializeOwned};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use surrealdb::sql::{Thing, Value};

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

//Custom type used for serializing to JSON Format used by frontend
//Code credit: barelm from the SurrealDB Discord
//Modifications: deserialize as Thing then convert to string to fix errors
#[derive(Debug, Clone)]
pub struct MyThing(Thing);

impl Serialize for MyThing {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}:{}", self.0.id, self.0.tb);
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for MyThing {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let thing: Thing = Deserialize::deserialize(deserializer)?;
        let s: String = thing.to_string();
        let parts: Vec<&str> = s.split(':').collect();
        println!("{:?}", parts);
        if parts.len() == 2 {
            Ok(MyThing(Thing {
                id: surrealdb::sql::Id::String(parts[0].to_string()),
                tb: parts[1].to_string(),
            }))
        } else {
            Err(serde::de::Error::custom("Invalid format"))
        }
    }
}

impl From<Thing> for MyThing {
    fn from(thing: Thing) -> Self {
        MyThing(thing)
    }
}

impl From<String> for MyThing {
    fn from(thing: String) -> Self {
        MyThing::from(thing.split_once(":").unwrap())
    }
}

impl From<&str> for MyThing {
    fn from(string: &str) -> Self {
        MyThing::from(string.split_once(":").unwrap())
    }
}

impl From<(&str, &str)> for MyThing {
    fn from(id_string: (&str, &str)) -> Self {
        MyThing(Thing::from((id_string.0, id_string.1)))
    }
}
