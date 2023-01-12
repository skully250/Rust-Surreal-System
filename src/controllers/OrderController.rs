use rocket::http::Status;
use serde::Deserialize;
use surrealdb::sql::{Thing, Value};

use crate::{
    models::{
        AuthModels,
        OrderModels::{self},
    },
    util::responders::JsonStatus,
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
    let query = db.query("SELECT *, (SELECT * FROM $parent.products[*].model LIMIT 1) as products[*].model FROM orders").await;
    //println!("{:?}", query);
    return match query {
        Ok(query) => {
            let order_result = query[0].output().unwrap();
            if let Value::Array(rows) = order_result {
                //println!("{0}", rows);
                let orders: Vec<OrderModels::DBOrder> =
                    serde_json::from_value(serde_json::json!(&rows))
                        .expect("Failed to parse order data");
                //println!("{:?}", orders);
                Ok(orders)
            } else {
                Err(Status::BadRequest)
            }
        }
        Err(_) => Err(Status::InternalServerError),
    };
}

pub async fn get_orders_by_user<'a>(
    db: &SurrealRepo,
    user: &str,
) -> Result<Vec<OrderModels::DBOrder>, (Status, &'a str)> {
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
                let query_return = serde_json::json!(&rows.first());
                println!("{:?}", query_return);
                let order_data: Result<RelatedOrders, serde_json::Error> =
                    serde_json::from_value(query_return);
                return match order_data {
                    Ok(order_data) => Ok(order_data.orders),
                    Err(_) => Err((Status::BadRequest, "No orders exist for that user")),
                };
            } else {
                Err((Status::BadRequest, "Failed to parse data"))
            }
        }
        Err(_) => Err((Status::InternalServerError, "Failed to execute query")),
    };
}

//TODO: Turn into a transaction so that creating order -> relation fail doesnt result in stranded entries
pub async fn create_order<'a>(
    db: &SurrealRepo,
    content: OrderModels::OrderDTO,
    user: &AuthModels::AuthUser,
) -> Result<JsonStatus<&'a str>, Status> {
    let order = OrderModels::Order::new(content);
    let query = db.create("orders", order, None).await;
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
