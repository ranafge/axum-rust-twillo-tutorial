use serde::{Serialize, Deserialize};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

struct Name<'a> {
    first: &'a str,
    last: &'a str,
}