use rocket::http::Status;
use serde::Deserialize;
use serde_json::json;
use surrealdb::{engine::remote::ws::Client, method::Query, sql::{self, statements::{BeginStatement, CommitStatement}, Thing, Value}, Surreal};

use crate::{
    models::{
        AuthModels,
        OrderModels::{self, Order},
        ProductModels::ProductDTO
    },
    util::{
        responders::JsonStatus,
        JsonUtil::{self, query_translate},
    },
    SurrealRepo,
};

#[derive(Deserialize)]
struct RelatedOrders {
    orders: Vec<OrderModels::DBOrder>,
}

pub fn get_thing(val: &Value) -> Result<&Thing, Status> {
    match val {
        Value::Thing(t) => Ok(t),
        _ => Err(Status::BadRequest),
    }
}

//Using namespaces to avoid confusiong between model and controller
pub async fn get_orders(db: &SurrealRepo) -> Result<Vec<OrderModels::DBOrder>, Status> {
    let query = db.query("SELECT *, (SELECT * FROM $parent.products[*].model LIMIT 1) as products[*].model FROM orders WHERE removed != true").await;
    //println!("{:?}", query);
    return match query {
        Ok(query) => {
            let order_result = query[0].output().unwrap();
            return query_translate(&order_result);
        }
        Err(_) => Err(Status::InternalServerError),
    };
}

pub async fn get_orders_by_user<'a>(
    db: &SurrealRepo,
    user: &str,
) -> Result<Vec<OrderModels::DBOrder>, Status> {
    //Potential to swap this for a new impl fn for this functionality
    let query_string = format!(
        "SELECT ->created->orders.* AS orders FROM users:{0} FETCH orders.products.model",
        user
    );
    let query = db.query(&query_string).await;
    return match query {
        Ok(query) => {
            let order_result = query[0].output().unwrap();
            if let Value::Array(rows) = order_result {
                println!("{:?}", rows.first());
                //Create Custom struct because of how the data returns from the DB
                let order_data: RelatedOrders = JsonUtil::query_translate(&rows.first().unwrap())
                    .expect("Failed to parse data");
                return Ok(order_data.orders);
            } else {
                Err(Status::BadRequest)
            }
        }
        Err(_) => Err(Status::InternalServerError),
    };
}

//Transaction based function
pub async fn create_order_transaction<'a>(db: &SurrealRepo, content: OrderModels::OrderDTO, user: &AuthModels::AuthUser) -> Result<JsonStatus<&'a str>, Status> {
    let order: (Order, Vec<ProductDTO>)= OrderModels::Order::new(content);
    let order_data = serde_json::json!(order[0]).to_string();
    let product_data = serde_json::json!(order[1]).to_string();

    let ds: Surreal<Client> = db.get_db();

    let rand_id = sql::Id::rand();
    let order_id = format!("orders:{rand_id}");

    //Transaction to catch failed product or order insertion
    //TODO: Work out how to deal with invoice creation mid transaction
    //TODO: Find way to get ID while inside transaction (Alt: Generate ID before creation)
    println!("{:?}", product_data);
    let query = ds.query(BeginStatement)
    .query(format!("CREATE {order_id} CONTENT $order"))
    .query("INSERT INTO products [$products]")
    .query("RELATE $user_id->created->$order_id SET time.created = time::now()")
    .query(CommitStatement)
    .bind(("order", order_data))
    .bind(("products", product_data))
    .bind(("order_id", order_id))
    .bind(("user_id", user.user)).await?;

    return match query {
        Ok(query) => {
            let result_entry = query[0].output.expect("Error in creating entry");
            Ok(JsonStatus::Created("order"))
            /*Ok(JsonStatus {
                status_code: Status::Ok,
                status: true,
                message: "Successfully created order"
            })*/
        },
        Err(_) => {
            Err(Status::InternalServerError)
        }
    }
}

//TODO: Turn into a transaction so that creating order -> relation fail doesnt result in stranded entries
pub async fn create_order<'a>(
    db: &SurrealRepo,
    content: OrderModels::OrderDTO,
    user: &AuthModels::AuthUser,
) -> Result<JsonStatus<&'a str>, Status> {
    let order = OrderModels::Order::new(content);
    let query = db.create("orders", order).await;
    return match query {
        Ok(query) => {
            let result_entry = query[0].output().expect("Error in creating entry");
            if let Value::Object(entry) = result_entry.first() {
                let value = get_thing(entry.get("id").unwrap()).unwrap();
                let username = format!("users:{0}", user.user);
                let related = db
                    .relate(
                        &username,
                        "created",
                        &value.to_string(),
                        "time.created = time::now()",
                    )
                    .await
                    .expect("Failed to relate documents");
                println!("ID: {:?}", related);
                Ok(JsonStatus {
                    status_code: Status::Ok,
                    status: true,
                    message: "Successfully created order",
                })
            } else {
                Err(Status::BadRequest)
            }
        }
        Err(_) => Err(Status::InternalServerError),
    };
}

pub async fn update_order<'a>(
    db: &SurrealRepo,
    order_id: &str,
    order: OrderModels::OrderDTO,
) -> Result<JsonStatus<&'a str>, Status> {
    let cur_order = format!("orders:{order_id}");
    let query = db.update(&cur_order, order).await;
    return match query {
        Ok(query) => {
            let result = query[0].output();
            if result.is_ok() {
                Ok(JsonStatus {
                    status_code: Status::Ok,
                    status: true,
                    message: "Order successfully updated",
                })
            } else {
                Err(Status::BadRequest)
            }
        }
        Err(_) => Err(Status::InternalServerError),
    };
}

pub async fn delete_order<'a>(db: &SurrealRepo, order_id: &str) -> Result<JsonStatus<&'a str>, Status> {
    let query_str = format!("UPDATE {order_id} SET removed = true");
    let query = db.query(&query_str).await;
    return match query {
        Ok(query) => {
            let result = query[0].output();
            if result.is_ok() {
                Ok(JsonStatus {
                    status_code: Status::Ok,
                    status: true,
                    message: "Order successfully deleted",
                })
            } else {
                Err(Status::BadRequest)
            }
        }
        Err(_) => Err(Status::InternalServerError),
    };
}
