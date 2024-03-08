use std::time::Instant;
use my_rest_api_twillo::surrealdb_curd_person;
use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
    routing::{delete, get, post},
    Extension, Json, Router,
};
use serde::Serialize;
use serde_json::json;
use sqlx::{MySqlPool, Row};
use tokio::net::TcpListener;

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}
// we used Serialize to convert the User struct  a JSON response

// Creates a new user and return a response with a status coede of 201 (CREATED) and a message of "User created successfully!"
async fn create_user() -> impl IntoResponse {
    println!("Create user handler");
    Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from("User created successfully!"))
        .unwrap()
}

// return a list of users as a JSON response
async fn list_users() -> Json<Vec<User>> {
    println!("List user handler");
    let users = vec![
        User {
            id: 1,
            name: "Elijah".to_string(),
            email: "elijah@example.com".to_string(),
        },
        User {
            id: 2,
            name: "Jhon".to_string(),
            email: "jhon@doe.com".to_string(),
        },
    ];
    Json(users)
}

// Define the get_users function as before
async fn get_users(Extension(pool): Extension<MySqlPool>) -> impl IntoResponse {
    let start = Instant::now();
    let rows = match sqlx::query("SELECT id, name, email FROM users")
        .fetch_all(&pool)
        .await
    {
        Ok(rows) => rows,
        Err(_) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error",
            )
                .into_response()
        }
    };

    let users: Vec<serde_json::Value> = rows
        .into_iter()
        .map(|row| {
            json!({
                "id": row.try_get::<i32, _>("id").unwrap_or_default(),
                "name": row.try_get::<String, _>("name").unwrap_or_default(),
                "email": row.try_get::<String, _>("email").unwrap_or_default(),
            })
        })
        .collect();
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    (axum::http::StatusCode::OK, Json(users)).into_response()
}

#[tokio::main]
async fn main() {

    let app = Router::new()
        // .route("/users", get(get_users))
        // .route("/create-user", post(create_user))
        .route("/list-users", get(list_users))
        .route("/x", get(people_list))
       
        // .route("/delete-users", delete(list_users).trace(|| async {println!("Deleted users")}))
        ;

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    // listens in for requests comming in at post 3000 for the local machine

    println!("Running on http://localhost:3000");
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
    /*
       starts the server and listens for incoming requests. It uses the listener and the app we created earlier
       kind of like our infinite while loop in previous server interators
    */
}



// use axum::Json;
use serde::{ Deserialize};
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
pub async fn people_list() -> Json<Vec<User>> {
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
    Json(people)
    // Ok(())

}