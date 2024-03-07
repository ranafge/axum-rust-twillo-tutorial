use serde::{Serialize, Deserialize};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[derive(Debug, Serialize)]
struct Name<'a> {
    first: &'a str,
    last: &'a str,
}

#[derive(Debug, Serialize)]
struct Person<'a > {
    title: &'a str,
    name : Name<'a>,
    marketing: bool
}

#[derive(Debug, Serialize)]
struct Responsibility{
    marketing: bool
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(unused)]
    id: Thing
}

#[tokio::main]
async fn main() -> surrealdb::Result<()>{
    // connect to the server 
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

    // Signin as a namespace, database or root user
    db.signin(Root{
        username: "root",
        password: "root",
    }).await?;
    // Select a specific namespace and database
    db.use_ns("test").use_db("test").await?;
    // Create a new person with a random id 
 
    // db.create(resource).content(data) resource is required for database table
    // here `person` is a table name
    // data it is option to record data to insert Person is data here or record
    // the below code is run as in database like CREATE $resource CONTENT $data;
    let created: Vec<Record> = db.create("person")
        .content(Person {
            title: "Founder and CEO",
            name: Name {
                first: "Tobie",
                last: "Morgan Hitchcock",
            },
            marketing: true
        }).await?;
        println!("Created person with id: {:?}", created);

    // update a person with a specific id 

    let updated: Option<Record> = db.update(("person", "jaime"))
        .merge(Responsibility {marketing:true}).await?;

    dbg!(updated);

    // Select all perople records
    let people = db.
        query("SELECT marketing, count() FROM type::table($table) GROUP BY marketing")
        .bind(("table", "person"))
        .await?;
    dbg!(people);

    

    Ok(())
}