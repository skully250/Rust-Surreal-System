#[get("/orders")]
pub fn get_orders(db: &State<SurrealRepo>) -> Result<Json<Vec<Order>>, Status> {}

#[post("/orders")]
pub fn add_orders() -> Result<Json<Vec<Order>>, Status> {}