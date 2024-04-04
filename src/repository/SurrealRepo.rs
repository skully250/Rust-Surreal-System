use once_cell::sync::Lazy;
use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize};
use std::fmt::Debug;
use surrealdb::{engine::remote::ws::{Client, Ws}, opt::PatchOp, sql::Thing, Response, Surreal};

use crate::util::JsonUtil::MyThing;

pub static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum PopulatedValue<T> {
    Populated(T),
    //#[serde(with = "MyThing")]
    //#[serde(deserialize_with = "value_from_str")]
    Unpopulated(Thing),
    Inputting(String)
}

fn value_from_str<'de, D>(deserializer: D) -> Result<MyThing, D::Error> where D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    Ok(MyThing::from(s))
}

pub struct DBConfig<'a> {
    pub path: &'a str,
    pub ns: &'a str,
    pub db: &'a str,
}

pub async fn connect(config: DBConfig<'_>) -> () {
    DB.connect::<Ws>(config.path).await.unwrap();
    DB.use_ns(config.ns).use_db(config.db).await.unwrap();
    return ();
}

//Look into potentialy using generics in future
fn get_json<T>(content: T) -> serde_json::Value
where
    T: Serialize + Debug,
{
    return serde_json::json!(content);
}

//TODO: Change this and find_where to be impl of models to specific variables, so dynamic queries cant pose an Injection Risk
pub async fn find_all_where<T: DeserializeOwned>(
    collection: &str,
    find_statement: &str,
) -> Result<Vec<T>, surrealdb::Error> {
    let query_string = format!("SELECT * FROM type::table($collection) WHERE {find_statement}");
    let mut query = DB
        .query(query_string)
        .bind(("collection", collection))
        .await
        .expect("Something went wrong");

    return query.take(0);
}

pub async fn find_where<T: DeserializeOwned>(
    collection: &str,
    selection: &str,
    find_statement: &str,
) -> Result<Vec<T>, surrealdb::Error> {
    let mut query = DB
        .query("SELECT $sel_string FROM type::table($collection) WHERE $find_statement")
        .bind(("sel_string", selection))
        .bind(("collection", collection))
        .bind(("find_statement", find_statement))
        .await
        .expect("Something went wrong");

    println!("Query: {:?}", query);

    return query.take(0);
}

pub async fn find_all<T: DeserializeOwned>(
    collection: &str,
) -> Result<Vec<T>, surrealdb::Error> {
    let response: Vec<T> = DB.select(collection).await?;
    return Ok(response);
}

pub async fn find<T: DeserializeOwned>(
    collection: &str,
    selection: &str,
) -> Result<T, surrealdb::Error> {
    let response: Option<T> = DB.select((collection, selection)).await?;
    return Ok(response.unwrap());
}

pub async fn create_named<T>(
    collection: &str,
    id: &str,
    content: T,
) -> Result<T, surrealdb::Error>
where
    T: Serialize + DeserializeOwned + Debug,
{
    let response: Option<T> = DB.create((collection, id)).content(content).await?;
    return Ok(response.unwrap());
}

pub async fn create<T> (name: &str, content: T) -> Result<Vec<T>, surrealdb::Error>
where
    T: Serialize + DeserializeOwned + Debug,
{
    let response: Vec<T> = DB.create(name).content(content).await?;
    return Ok(response);
}

pub async fn update<T>(
    collection: &str,
    selection: &str,
    content: T,
) -> Result<Option<T>, surrealdb::Error>
where
    T: Serialize + DeserializeOwned + Debug,
{
    let response: Option<T> = DB
        .update((collection, selection))
        .merge(content)
        .await?;
    return Ok(response);
}

pub async fn update_patch<T: Serialize + Debug>(
    collection: &str,
    selection: &str,
    patch: PatchOp,
) -> Result<Option<T>, surrealdb::Error>
where
    T: Serialize + DeserializeOwned + Debug,
{
    let response = DB
        .update((collection, selection))
        .patch(patch)
        .await
        .unwrap();
    return Ok(response);
}

pub async fn update_where<T: Serialize + Debug>(
    name: &str,
    content: T,
    find_statement: &str,
) -> Result<Response, surrealdb::Error> {
    return DB
        .query("UPDATE $name MERGE $content WHERE $find_statement")
        .bind([
            ("name", name),
            ("content", &get_json(content).to_string()),
            ("find_statement", find_statement),
        ])
        .await;
}

pub async fn relate(
    from: &str,
    action: &str,
    to: &str,
    content: &str,
) -> Result<Response, surrealdb::Error> {
    return DB
        .query("RELATE $from->type::table($action)->$to SET $content")
        .bind([
            ("from", from),
            ("action", action),
            ("to", to),
            ("content", content),
        ])
        .await;
}

pub async fn delete<T> (collection: &str, item: &str) -> Result<T, surrealdb::Error>
where
    T: Serialize + DeserializeOwned + Debug,
{
    let response: Option<T> = DB.delete((collection, item)).await?;
    return Ok(response.unwrap());
}

//TODO: Find out why query is broken - it simply returns whatever text is sent into it currently
pub async fn query (query: &str) -> Result<Response, surrealdb::Error> {
    let query = DB.query(query).await?;
    return Ok(query);
}