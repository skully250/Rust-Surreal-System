use serde::Serialize;
use surrealdb::{Datastore, Response, Session};

pub struct DBConfig<'a> {
    pub path: &'a str,
    pub ns: &'a str,
    pub db: &'a str
}

pub struct SurrealRepo {
    ds: Datastore,
    ses: Session,
}

//Look into potentialy using generics in future
impl SurrealRepo {
    pub async fn init(config: DBConfig<'_>) -> Self {
        let ds = Datastore::new(config.path)
            .await
            .expect("Error occured connecting to surreal");
        let ses = Session::for_db(config.ns, config.db);
        return SurrealRepo { ds, ses };
    }

    fn get_json<T: Serialize>(content: T) -> serde_json::Value {
        return serde_json::json!(content)
    }

    pub async fn find(
        &self,
        query: &str,
        collection: &str
    ) -> Result<Vec<Response>, surrealdb::Error> {
        let query = format!("SELECT {query} FROM {collection}");
        return self.ds.execute(&query, &self.ses, None, false).await;
    }

    pub async fn create<T: Serialize>(
        &self,
        name: &str,
        content: T,
    ) -> Result<Vec<Response>, surrealdb::Error> {
        let query = format!("CREATE {0} CONTENT {1}", name, self::SurrealRepo::get_json(content));
        return self.ds.execute(&query, &self.ses, None, false).await;
    }

    pub async fn update<T: Serialize>(
        &self,
        name: &str,
        content: T,
    ) -> Result<Vec<Response>, surrealdb::Error> {
        let query = format!("UPDATE {0} CONTENT {1}", name, self::SurrealRepo::get_json(content));
        return self.ds.execute(&query, &self.ses, None, false).await;
    }

    pub async fn relate(
        &self,
        from: &str,
        action: &str,
        to: &str,
        content: &str,
    ) -> Result<Vec<Response>, surrealdb::Error> {
        let query = format!("RELATE {from}->{action}->{to} SET {content}");
        return self.ds.execute(&query, &self.ses, None, false).await;
    }

    pub async fn query(&self, query: &str) -> Result<Vec<Response>, surrealdb::Error> {
        return self.ds.execute(query, &self.ses, None, false).await;
    }
}
