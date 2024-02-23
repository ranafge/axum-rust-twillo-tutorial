use axum::{routing::get, Router};
use tokio::net::TcpListener;



#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async {"Hello, Rust!"}))
        
        // .route("/delete-users", delete(list_users).trace(|| async {println!("Deleted users")}))
        
        ;

        let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
        // listens in for requests comming in at post 3000 for the local machine

        println!("Running on http://localhost:3000");
        axum::serve(listener, app.into_make_service()).await.unwrap();
        /*
            starts the server and listens for incoming requests. It uses the listener and the app we created earlier
            kind of like our infinite while loop in previous server interators
         */
}