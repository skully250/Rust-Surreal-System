use crate::models::ActionModels;

pub async fn action_product<'a>(
    db: &SurrealRepo,
    product_id: String,
    action: ActionDTO,
) -> Result<(Status, &'a str), Status> {
    let data = serde_json::json!(action.action).to_string();
    let query = format!(
        "UPDATE {0} SET products[WHERE index = {1}].actions.{2} = {3}",
        product_id, action.index, action.action_name, data
    );
    println!("{:?}", query);
    let query_result = db.relate()
    let query_result = db.query(&query).await;
    return match query_result {
        Ok(_query) => Ok({
            println!("{:?}", _query);
            (Status::Ok, "Action successfully run")
        }),
        Err(_e) => Err(Status::BadRequest),
    };
}

pub async fn modify_product(product: ProductModels::ProductDTO) -> ProductModels::DBProduct {

}