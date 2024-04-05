use rocket::http::Status;
use rocket::{serde::json::Json, Route, State};

use crate::models::ActionModels::ActionList;
use crate::models::AuthModels::AuthUser;
use crate::models::OrderModels::Order;
use crate::util::responders::JsonStatus;
use crate::{controllers, models};

pub fn order_routes() -> Vec<Route> {
    let routes = routes![
        get_orders,
        add_order,
        update_order,
        delete_order,
        get_orders_by_user,
        get_orders_by_customer
        //action_product
    ];
    return routes;
}

async fn action_exists(action_name: &String, action_list: &State<ActionList>) -> bool {
    let action_list = action_list.actions.read().await;
    action_list.contains(action_name)
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
        Err(err) => Err(err)
    }
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
async fn update_order(
    order_id: &str,
    order: Json<models::OrderModels::Order>,
) -> Result<JsonStatus<&str>, Status> {
    return controllers::OrderController::update_order(&order_id, order.into_inner()).await;
}

#[delete("/<order_id>")]
async fn delete_order(order_id: &str) -> Result<JsonStatus<&str>, Status> {
    return controllers::OrderController::delete_order(order_id).await;
}

//Products

//Potential: Add index as query instead of in struct
/*#[post("/action/<order_id>", format = "json", data = "<action>")]
async fn action_product<'a>(
    action_list: &State<ActionList>,
    order_id: String,
    action: Json<ActionModels::ActionDTO>,
) -> Result<JsonStatus<&'a str>, Status> {
    let action_name = action.action_name.to_owned();
    let contains = action_exists(&action_name, action_list).await;
    match contains {
        true => {
            let query =
                controllers::ActionController::action_product(order_id, action.into_inner()).await;
            match query {
                Ok(status) => Ok(JsonStatus {
                    status_code: status.0,
                    status: true,
                    message: status.1,
                }),
                Err(e) => Err(e),
            }
        }
        false => Err(Status::NotFound),
    }
}*/
