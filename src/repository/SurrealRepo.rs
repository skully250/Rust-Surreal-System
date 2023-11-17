use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt::Debug;
use surrealdb::{
    dbs::Session,
    engine::local::{Db, RocksDb},
    iam::Level,
    iam::Role,
    Response, Surreal,
};

//Trait for use as implementation on data types that interact with the DB
#[rocket::async_trait]
pub trait DBInteractions<T>
where
    T: for<'a> Deserialize<'a>,
{
    async fn find(db: &SurrealRepo) -> Result<T, surrealdb::Error>;
    async fn find_where(db: &SurrealRepo) -> Result<Vec<T>, surrealdb::Error>;
    async fn find_all(db: &SurrealRepo) -> Result<Vec<T>, surrealdb::Error>;

    async fn create(db: &SurrealRepo) -> Result<bool, surrealdb::Error>;
    async fn update(db: &SurrealRepo) -> Result<bool, surrealdb::Error>;
}

pub struct DBConfig<'a> {
    pub path: &'a str,
    pub ns: &'a str,
    pub db: &'a str,
}

pub struct SurrealRepo {
    ds: Surreal<Db>,
    ses: Session,
}

//Look into potentialy using generics in future
impl SurrealRepo {
    pub async fn init(config: DBConfig<'_>) -> Self {
        let ds = Surreal::new::<RocksDb>(config.path)
            .await
            .expect("Error occured connecting to surreal");
        let ses = Session::for_level(
            Level::Database(String::from(config.ns), String::from(config.db)),
            Role::Owner,
        );
        ds.use_ns(config.ns).use_db(config.db).await.unwrap();
        return SurrealRepo { ds, ses };
    }

    fn get_json<T>(content: T) -> serde_json::Value
    where
        T: Serialize + Debug,
    {
        return serde_json::json!(content);
    }

    pub async fn find_where<T: DeserializeOwned>(
        &self,
        collection: &str,
        selection: Option<&str>,
        find_statement: &str,
    ) -> Result<Vec<T>, surrealdb::Error> {
        //let array = HashMap::from()
        let sel_string = if Option::is_some(&selection) {
            selection.unwrap()
        } else {
            "*"
        };

        let mut query = self
            .ds
            .query("SELECT $sel_string FROM type::table($collection) WHERE $find_statement")
            .bind(("sel_string", sel_string))
            .bind(("collection", collection))
            .bind(("find_statement", find_statement))
            .await?;

        println!("Query: {:?}", query);

        return query.take(0);
    }

    pub async fn find_all<T: DeserializeOwned>(
        &self,
        collection: &str,
    ) -> Result<Vec<T>, surrealdb::Error> {
        let response: Vec<T> = self.ds.select(collection).await?;
        return Ok(response);
    }

    pub async fn find<T: DeserializeOwned>(
        &self,
        collection: &str,
        selection: &str,
    ) -> Result<T, surrealdb::Error> {
        let response: Option<T> = self.ds.select((collection, selection)).await?;
        return Ok(response.unwrap());
    }

    pub async fn create_named<T>(
        &self,
        collection: &str,
        id: &str,
        content: T,
    ) -> Result<T, surrealdb::Error>
    where
        T: Serialize + DeserializeOwned + Debug,
    {
        let response: Option<T> = self.ds.create((collection, id)).content(content).await?;
        return Ok(response.unwrap());
    }

    pub async fn create<T>(&self, name: &str, content: T) -> Result<Vec<T>, surrealdb::Error>
    where
        T: Serialize + DeserializeOwned + Debug,
    {
        let response: Vec<T> = self.ds.create(name).content(content).await?;
        return Ok(response);
    }

    pub async fn update<T>(
        &self,
        collection: &str,
        selection: &str,
        content: T,
    ) -> Result<Option<T>, surrealdb::Error>
    where
        T: Serialize + DeserializeOwned + Debug,
    {
        let response: Option<T> = self
            .ds
            .update((collection, selection))
            .merge(content)
            .await?;
        return Ok(response);
    }

    pub async fn update_where<T: Serialize + Debug>(
        &self,
        name: &str,
        content: T,
        find_statement: &str,
    ) -> Result<Response, surrealdb::Error> {
        return self
            .ds
            .query("UPDATE $name MERGE $content WHERE $find_statement")
            .bind([
                ("name", name),
                ("content", &self::SurrealRepo::get_json(content).to_string()),
                ("find_statement", find_statement),
            ])
            .await;
    }

    pub async fn relate(
        &self,
        from: &str,
        action: &str,
        to: &str,
        content: &str,
    ) -> Result<Response, surrealdb::Error> {
        return self
            .ds
            .query("RELATE $from->type::table($action)->$to SET $content")
            .bind([
                ("from", from),
                ("action", action),
                ("to", to),
                ("content", content),
            ])
            .await;
    }

    pub async fn delete<T>(&self, collection: &str, item: &str) -> Result<T, surrealdb::Error>
    where
        T: Serialize + DeserializeOwned + Debug,
    {
        let response: Option<T> = self.ds.delete((collection, item)).await?;
        return Ok(response.unwrap());
    }

    //TODO: Find out why query is broken - it simply returns whatever text is sent into it currently
    pub async fn query(&self, query: &str) -> Result<Response, surrealdb::Error> {
        let mut query = self.ds.query(query).await?;
        return Ok(query);
    }
}
