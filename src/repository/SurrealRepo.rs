use serde::Serialize;
use std::fmt::Debug;
use surrealdb::{Datastore, Response, Session};

pub struct DBConfig<'a> {
    pub path: &'a str,
    pub ns: &'a str,
    pub db: &'a str,
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

    fn get_json<T>(content: T) -> serde_json::Value
    where
        T: Serialize + Debug,
    {
        return serde_json::json!(content);
    }

    pub async fn find_where(
        &self,
        selection: Option<&str>,
        collection: &str,
        find_statement: &str,
    ) -> Result<Vec<Response>, surrealdb::Error> {
        let db_query: String = match selection {
            Some(sel_string) => {
                format!("SELECT {sel_string} FROM {collection} WHERE {find_statement}")
            }
            None => format!("SELECT * FROM {collection} WHERE {find_statement}"),
        };
        return self.ds.execute(&db_query, &self.ses, None, false).await;
    }

    pub async fn find(
        &self,
        selection: Option<&str>,
        collection: &str,
    ) -> Result<Vec<Response>, surrealdb::Error> {
        let db_query = match selection {
            Some(query_string) => format!("SELECT {query_string} FROM {collection}"),
            None => format!("SELECT * FROM {collection}"),
        };
        return self.ds.execute(&db_query, &self.ses, None, false).await;
    }

    pub async fn create<T: Serialize + Debug>(
        &self,
        name: &str,
        content: T,
        has_name: Option<String>,
    ) -> Result<Vec<Response>, surrealdb::Error> {
        let db_name;
        match has_name {
            Some(some_name) => db_name = format!("{name}:{some_name}"),
            None => db_name = format!("{name}"),
        }
        let query = format!(
            "CREATE {0} CONTENT {1}",
            db_name,
            self::SurrealRepo::get_json(content)
        );
        return self.ds.execute(&query, &self.ses, None, false).await;
    }

    pub async fn update<T: Serialize + Debug>(
        &self,
        name: &str,
        content: T,
    ) -> Result<Vec<Response>, surrealdb::Error> {
        let query = format!(
            "UPDATE {0} CONTENT {1}",
            name,
            self::SurrealRepo::get_json(content)
        );
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
