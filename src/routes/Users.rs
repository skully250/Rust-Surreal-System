#[get("/users")]
pub fn get_users(db: &State<SurrealRepo>) -> Result<Json<Vec<User>>, Status> {}

#[post("/users")]
pub fn add_users() -> Result<Json<Vec<User>>, Status> {}