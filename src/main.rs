use std::time::Instant;

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
    let database_url = "mysql://root:Rust_12345@0.tcp.ngrok.io:11872/mysql";
    let pool = MySqlPool::connect(&database_url).await.unwrap();

    let app = Router::new()
        .route("/users", get(get_users))
        .layer(Extension(pool))
        // .route("/create-user", post(create_user))
        // .route("/list-users", get(list_users))
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
