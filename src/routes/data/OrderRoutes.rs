use rocket::http::Status;
use rocket::{serde::json::Json, Route, State};

use crate::models::AuthModels::AuthUser;
use crate::models::OrderModels::DBOrder;
use crate::models::ProductModels::{self, ActionList};
use crate::util::responders::JsonStatus;
use crate::{controllers, models, SurrealRepo};

pub fn order_routes() -> Vec<Route> {
    let routes = routes![
        get_orders,
        add_order,
        update_order,
        delete_order,
        get_orders_by_user,
        action_product
    ];
    return routes;
}

fn action_exists(action_name: &String, action_list: &State<ActionList>) -> bool {
    let action_list = action_list.actions.read().unwrap();
    action_list.contains(action_name)
}

#[get("/")]
async fn get_orders(db: &State<SurrealRepo>) -> Result<Json<Vec<DBOrder>>, Status> {
    let controller_orders = controllers::OrderController::get_orders(db).await;
    return match controller_orders {
        Ok(orders) => Ok(Json(orders)),
        Err(err) => Err(err),
    };
}

#[get("/?<customer>")]
async fn get_orders_by_customer(db: &State<SurrealRepo>, customer: &str) -> Status {
    return Status::NotImplemented;
}

#[get("/?<user>")]
async fn get_orders_by_user<'a>(
    db: &State<SurrealRepo>,
    user: &str,
) -> Result<Json<Vec<DBOrder>>, JsonStatus<&'a str>> {
    let related_orders = controllers::OrderController::get_orders_by_user(db, user).await;
    return match related_orders {
        Ok(orders) => Ok(Json(orders)),
        Err(err) => Err(JsonStatus {
            status_code: err.0,
            status: false,
            message: err.1,
        }),
    };
}

#[post("/", format = "json", data = "<order>")]
async fn add_order(
    db: &State<SurrealRepo>,
    user: AuthUser,
    order: Json<models::OrderModels::OrderDTO>,
) -> Result<JsonStatus<&str>, Status> {
    return controllers::OrderController::create_order(db, order.into_inner(), &user).await;
}

#[put("/<order_id>", format = "json", data = "<order>")]
async fn update_order(
    db: &State<SurrealRepo>,
    order_id: String,
    order: Json<models::OrderModels::OrderDTO>,
) -> Result<JsonStatus<&str>, Status> {
    return controllers::OrderController::update_order(db, &order_id, order.into_inner()).await;
}

#[delete("/<order_id>")]
async fn delete_order(
    db: &State<SurrealRepo>,
    order_id: String,
) -> Result<JsonStatus<&str>, Status> {
    let db_name = format!("orders:{order_id}");
    return Err(Status::NotImplemented);
}

//Products

//Potential: Add index as query instead of in struct
#[post("/action/<order_id>", format = "json", data = "<action>")]
async fn action_product<'a>(
    db: &State<SurrealRepo>,
    action_list: &State<ActionList>,
    order_id: String,
    action: Json<ProductModels::ActionDTO>,
) -> Result<JsonStatus<&'a str>, Status> {
    let action_name = action.action_name.to_owned();
    let contains = action_exists(&action_name, action_list);
    match contains {
        true => {
            let query =
                controllers::ActionController::action_product(db, order_id, action.into_inner())
                    .await;
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
}
