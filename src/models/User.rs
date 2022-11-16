use surrealdb::sql::Id;

//Putting both customer and user in the same file since they both correlate to a person in the DB

//Will be expanded upon later to include additional details
pub struct Customer {
    id: Id,
    name: String
}

//Will add more metadata to users later as expectations expand
pub struct User {
    id: Id,
    username: String,
    hash: String,
    password: String,
}