use surrealdb::{Session, Datastore, Response};

pub struct SurrealRepo {
    ds: Datastore,
    ses: Session
}

impl SurrealRepo {
    pub async fn init(ns: &str, db: &str) -> Self {
        let ds = Datastore::new("memory").await.expect("Error occured connecting to surreal");
        let ses = Session::for_db(ns, db);
        return SurrealRepo { ds, ses }
    }

    pub async fn query(&self, query: &str) -> Result<Vec<Response>, surrealdb::Error> {
        let res = self.ds.execute(query, &self.ses, None, false).await;
        return res;
    }
}