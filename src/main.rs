use axum::{routing::{get, Route}, Router};
use tokio::net::TcpListener;



#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async {"hello world"})) ;
    println!("Running an localhost:3000");
    axum::serve(TcpListener::bind("127.0.0.1:8080").await.unwrap(), app).await.unwrap();


}   