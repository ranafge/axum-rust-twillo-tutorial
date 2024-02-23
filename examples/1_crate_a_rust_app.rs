use axum::{routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, Rust!" }));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Running on http://localhost:3000");

    axum::serve(listener, app.into_make_service()).await.unwrap();
}

/*
    Fundamental axum router server
*/