use rocket::http::Status;
use rocket::{serde::json::Json, Route, State};

use crate::controllers::ActionController;
use crate::models::ActionModels::{ActionDTO, ActionList};
use crate::models::AuthModels::AuthUser;
use crate::models::OrderModels::Order;
use crate::repository::SurrealRepo::DB;
use crate::util::responders::{JsonStatus, Jsonstr};
use crate::{controllers, models};

pub fn order_routes() -> Vec<Route> {
    let routes = routes![
        get_orders,
        add_order,
        update_order,
        delete_order,
        get_orders_by_user,
        get_orders_by_customer,
        action_product,
        get_unpopulated_orders
    ];
    return routes;
}

async fn action_exists(action_name: &String, action_list: &State<ActionList>) -> bool {
    let action_list = action_list.actions.read().await;
    action_list.contains(action_name)
}

#[get("/unpopulated")]
async fn get_unpopulated_orders() -> Result<Json<Vec<Order>>, Status> {
    let query = DB.query("SELECT * FROM orders").await;
    return match query {
        Ok(mut response) => {
            let orders: Vec<Order> = response.take(0).unwrap();
            Ok(Json(orders))
        }
        Err(err) => Err(Status::InternalServerError),
    };
}

#[get("/")]
async fn get_orders() -> Result<Json<Vec<Order>>, Status> {
    let controller_orders = controllers::OrderController::get_orders().await;
    return match controller_orders {
        Ok(orders) => Ok(Json(orders)),
        Err(err) => Err(err),
    };
}

#[get("/customer?<customer>")]
async fn get_orders_by_customer(customer: &str) -> Result<Json<Vec<Order>>, Status> {
    let orders = Order::orders_by_customer_name(customer).await;
    return match orders {
        Ok(found) => Ok(Json(found)),
        Err(err) => Err(err),
    };
}

#[get("/user?<user>")]
async fn get_orders_by_user(user: &str) -> Result<Json<Vec<Order>>, Status> {
    let related_orders = controllers::OrderController::get_orders_by_user(user).await;
    return match related_orders {
        Ok(orders) => Ok(Json(orders)),
        Err(err) => Err(err),
    };
}

#[post("/", format = "json", data = "<order>")]
async fn add_order(
    user: AuthUser,
    order: Json<models::OrderModels::OrderDTO>,
) -> Result<JsonStatus<String>, Status> {
    return controllers::OrderController::create_order(order.into_inner(), &user).await;
}

#[put("/<order_id>", format = "json", data = "<order>")]
async fn update_order(order_id: &str, order: Json<models::OrderModels::Order>) -> Jsonstr {
    return controllers::OrderController::update_order(&order_id, order.into_inner()).await;
}

#[delete("/<order_id>")]
async fn delete_order(order_id: &str) -> Jsonstr {
    return controllers::OrderController::delete_order(order_id).await;
}

//Product Actions

#[post("/action/<action>", format = "json", data = "<action_info>")]
async fn action_product<'a>(
    action_list: &State<ActionList>,
    action: &'a str,
    action_info: Json<ActionDTO>,
) -> Result<JsonStatus<&'a str>, JsonStatus<&'a str>> {
    let contains = action_exists(&action.to_string(), action_list).await;
    match contains {
        true => {
            let action_result =
                ActionController::action_product(action, action_info.into_inner()).await;
            if action_result.is_some() {
                Ok(action_result.unwrap())
            } else {
                Err(JsonStatus::failure("Failed to action product"))
            }
        }
        false => Err(JsonStatus::failure("Unable to find action active")),
    }
}
