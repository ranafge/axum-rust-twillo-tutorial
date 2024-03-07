use axum::Json;
use serde::{Serialize, Deserialize};
use sqlx::{query, Value};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

// #[derive(Debug, Serialize)]
// struct Name<'a> {
//     first: &'a str,
//     last: &'a str,
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct Person<'a > {
//     title: &'a str,
//     // name : Name<'a>,
//     marketing: bool
// }

// #[derive(Debug, Serialize)]
// struct Responsibility{
//     marketing: bool
// }

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(unused)]
    id: Thing
}

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    // id: i32, // Adjust field types according to your database schema
    name: String,
    age: i32,
}

// axum::Json<serde_json::Value>
pub async fn people_list() -> Result<(), sqlx::Error> {
    println!("people list handkeler sdjfssd");
    // connect to the server 
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await.unwrap();

    // Signin as a namespace, database or root user
    db.signin(Root{
        username: "root",
        password: "root",
    }).await.expect("failed to sign ");
    // Select a specific namespace and database
    db.use_ns("test").use_db("test").await.expect("failed to selece database ");
    // Create a new person with a random id 
 
    // db.create(resource).content(data) resource is required for database table
    // here `person` is a table name
    // data it is option to record data to insert Person is data here or record
    // the below code is run as in database like CREATE $resource CONTENT $data;
    let created: Vec<Record> = db.create("person")
        .content(Person {
           name: "x".to_string(),
           age: 10,
        }).await.unwrap();
        println!("Created person with id: {:?}", created);

    // update a person with a specific id 


    // Select all perople records
    let people = db.
        // query("SELECT marketing, count() FROM type::table($table) GROUP BY marketing")
        query("SELECT * FROM type::table")
        .bind(("table", "person"))
        .await.unwrap();
    dbg!(people);
    let people = db.query("SELECT * FROM person").await.unwrap();
    // let titles = people.iter().map(|x| x.name).collect::<Vec<String>>().join(",");
    // people.iter().map(|x| x.name).collect::<Value<String>>().join(",");
    // people.check().into_iter().map(|x| x.name).collect::<Value<String>>;
    dbg!(people);
    // Ok(people)
    Ok(())

}