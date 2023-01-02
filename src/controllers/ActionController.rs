use rocket::{http::Status, State};
use surrealdb::sql::Value;

use crate::{
    models::ProductModels::{ActionDTO, ActionList, DBAction},
    util::responders::JsonStatus,
    SurrealRepo,
};

pub async fn action_product<'a>(
    db: &SurrealRepo,
    action_name: String,
    action: ActionDTO<'a>,
) -> Result<(Status, &'a str), Status> {
    let data = serde_json::json!(action.action).to_string();
    let query = format!(
        "UPDATE {0} SET products[WHERE index = {1}].actions.{2} = {3}",
        action.order_id, action.index, action_name, data
    );
    println!("{:?}", query);
    let query_result = db.query(&query).await;
    return match query_result {
        Ok(_query) => Ok({
            println!("{:?}", _query);
            (Status::Ok, "Action successfully run")
        }),
        Err(_e) => Err(Status::BadRequest),
    };
}

pub async fn get_actions(db: &SurrealRepo) -> Result<Vec<DBAction>, Status> {
    let query = db.find(None, "actions").await;
    return match query {
        Ok(query_result) => {
            let result_entry = query_result[0].output().unwrap();
            if let Value::Array(rows) = result_entry {
                println!("{:?}", rows);
                let actions: Vec<DBAction> = serde_json::from_value(serde_json::json!(&rows))
                    .expect("Failed to parse actions");
                Ok(actions)
            } else {
                Err(Status::BadRequest)
            }
        }
        Err(_) => Err(Status::BadRequest),
    };
}

pub async fn create_action<'a>(
    db: &SurrealRepo,
    actions: &State<ActionList>,
    action_name: &str,
) -> Result<JsonStatus<&'a str>, Status> {
    let query = db
        .create("actions", serde_json::json!({ "name": action_name }), None)
        .await;
    let mut actions = actions
        .actions
        .write()
        .expect("Could not open writeable reference");
    actions.push(action_name.to_string());
    return match query {
        Ok(_query_result) => Ok(JsonStatus {
            status_code: Status::Ok,
            status: true,
            message: "Successfully created test action",
        }),
        Err(_) => Err(Status::BadRequest),
    };
}

//Potentially Deprecated - May return in future
// pub async fn create_product(
//     db: &SurrealRepo,
//     content: &models::ProductModels::ProductDTO,
// ) -> Result<Vec<Response>, surrealdb::Error> {
//     let query = db.create("products", content, None).await;
//     return match query {
//         Ok(query) => Ok(query),
//         Err(e) => panic!("DB Could not create product - Error: {:?}", e),
//     };
// }
