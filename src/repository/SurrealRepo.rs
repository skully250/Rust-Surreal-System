use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt::Debug;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::PatchOp,
    Response, Surreal,
};

pub static DB: Surreal<Client> = Surreal::init();

pub struct DBConfig<'a> {
    pub path: &'a str,
    pub ns: &'a str,
    pub db: &'a str,
}

pub async fn connect(config: DBConfig<'_>) -> ApiResult<()> {
    DB.connect(config.path).await?;
    DB.use_ns(config.ns).use_db(config.db).await.unwrap();
    return Ok(());
}

//Look into potentialy using generics in future
impl SurrealRepo {
    pub async fn init(config: DBConfig<'_>) -> Self {
        let ds = Surreal::new::<Ws>(config.path)
            .await
            .expect("Error occured connecting to surreal");
        ds.use_ns(config.ns).use_db(config.db).await.unwrap();
        return SurrealRepo { ds };
    }

    //Potentially dangerous
    //Alt: Some other way to do pre-made queries and bind outside of sharing the Datastore
    pub fn get_db() -> Surreal<Client> {
        return Self.ds;
    }

    fn get_json<T>(content: T) -> serde_json::Value
    where
        T: Serialize + Debug,
    {
        return serde_json::json!(content);
    }

    pub async fn find_all_where<T: DeserializeOwned>(
        &self,
        collection: &str,
        find_statement: &str,
    ) -> Result<Vec<T>, surrealdb::Error> {
        let mut query = self
            .ds
            .query("SELECT * FROM type::table($collection) WHERE $find")
            .bind(("collection", collection))
            .bind(("find", find_statement))
            .await
            .expect("Something went wrong");

        return query.take(0);
    }

    pub async fn find_where<T: DeserializeOwned>(
        &self,
        collection: &str,
        selection: &str,
        find_statement: &str,
    ) -> Result<Vec<T>, surrealdb::Error> {
        let mut query = self
            .ds
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

    pub async fn create<T>(&self, collection: &str, content: T) -> Result<Vec<T>, surrealdb::Error>
    where
        T: Serialize + DeserializeOwned + Debug,
    {
        let response: Vec<T> = self.ds.create(collection).content(content).await?;
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

    pub async fn update_patch<T: Serialize + Debug>(
        &self,
        collection: &str,
        selection: &str,
        patch: PatchOp,
    ) -> Result<Option<T>, surrealdb::Error>
    where
        T: Serialize + DeserializeOwned + Debug,
    {
        let response = self
            .ds
            .update((collection, selection))
            .patch(patch)
            .await
            .unwrap();
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
        let query = self.ds.query(query).await?.unwrap();
        return Ok(query);
    }
}
