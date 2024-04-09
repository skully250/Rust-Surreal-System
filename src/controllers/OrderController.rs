use rocket::http::Status;
use serde::Deserialize;
use surrealdb::sql::statements::{BeginStatement, CommitStatement};

use crate::{
    models::{
        AuthModels,
        OrderModels::{self, Order},
    },
    repository::SurrealRepo::{self, DB},
    util::responders::JsonStatus,
};

#[derive(Deserialize)]
struct RelatedOrders {
    orders: Vec<OrderModels::Order>,
}

//Using namespaces to avoid confusiong between model and controller
pub async fn get_orders() -> Result<Vec<Order>, Status> {
    let query: Result<surrealdb::Response, surrealdb::Error> =
        SurrealRepo::query("SELECT *, (SELECT *, array::first((SELECT VALUE array::first(<-actioned.actions) FROM $parent.id)) as actions FROM $parent.products) as products FROM orders FETCH customer;").await;
    //println!("{:?}", query);
    return match query {
        Ok(mut query_return) => {
            let order_result: Vec<Order> = query_return.take(0).unwrap();
            Ok(order_result)
        }
        Err(_) => Err(Status::InternalServerError),
    };
}

pub async fn get_orders_by_user(user: &str) -> Result<Vec<OrderModels::Order>, Status> {
    //Potential: make this an extension of user
    let query = DB
        .query("select ->created->orders.* AS orders FROM $user FETCH orders.products.model")
        .bind(("user", format!("users:{user}")))
        .await;
    return match query {
        Ok(mut query_response) => {
            let order_result = query_response.take(0).unwrap();
            Ok(order_result)
        }
        Err(_) => Err(Status::InternalServerError),
    };
}

//Transaction based function
pub async fn create_order(
    content: OrderModels::OrderDTO,
    user: &AuthModels::AuthUser,
) -> Result<JsonStatus<String>, Status> {
    let order: Order = OrderModels::Order::new(&content);
    //Could not find reliable way to do this without preformatting string
    let relate_query = format!(
        "RELATE users:{}->created->$order_id SET time.created = time::now();",
        &user.user
    );
    //Transaction to catch failed product or order insertion
    //TODO: Work out how to deal with invoice creation mid transaction
    //Without value it returns array objects, i want singular values
    let query = DB
        .query(BeginStatement)
        .query("let $order_no = UPDATE counter:orders SET orders += 1 RETURN VALUE orders;")
        .query("let $product_ids = INSERT INTO products $products RETURN VALUE id;")
        .query("UPDATE $product_ids SET orderNo = array::first($order_no);")
        .query("let $order_id = CREATE orders CONTENT $order RETURN VALUE id;")
        .query("let $customer_id = SELECT VALUE id FROM customers WHERE name = $customer_name;")
        .query("UPDATE $order_id SET products = $product_ids, customer = array::first($customer_id), orderNo = array::first($order_no) RETURN AFTER;")
        .query(relate_query)
        .query(CommitStatement)
        .bind(("order", order))
        .bind(("products", content.products))
        .bind(("customer_name", content.customer))
        .await;

    //println!("{:?}", query);

    return match query {
        Ok(mut query_return) => {
            let result_entry: Result<Vec<Order>, surrealdb::Error> = query_return.take(5);
            //Input data is fine, return data is erroring
            if result_entry.is_ok() {
                Ok(JsonStatus::created("order"))
            } else {
                println!("{:?}", result_entry);
                Err(Status::InternalServerError)
            }
        }
        Err(_) => Err(Status::InternalServerError),
    };
}

pub async fn update_order(
    order_id: &str,
    order: OrderModels::Order,
) -> Result<JsonStatus<&str>, Status> {
    let cur_order = format!("orders:{order_id}");
    //Todo: Check for non-product updates such as customer changes
    let query = DB
        .query(BeginStatement)
        .query("let $product_ids = (INSERT INTO products $products RETURN id)")
        .query("INSERT INTO $order_id $order")
        .query("UPDATE $order_id SET products = $product_ids")
        .query(CommitStatement)
        .bind(("order_id", cur_order))
        .bind(("products", order.products))
        .await;
    return match query {
        Ok(_) => Ok(JsonStatus::success("Order successfully updated")),
        Err(_) => Err(Status::InternalServerError),
    };
}

pub async fn delete_order(order_id: &str) -> Result<JsonStatus<&str>, Status> {
    let query = DB
        .query("UPDATE $order_id SET removed = true")
        .bind(("order_id", order_id))
        .await;
    return match query {
        Ok(_) => Ok(JsonStatus::success("Order successfully deleted")),
        Err(_) => Err(Status::InternalServerError),
    };
}
