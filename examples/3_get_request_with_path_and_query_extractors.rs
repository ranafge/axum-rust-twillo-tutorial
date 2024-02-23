
use axum::{extract::{Path, Query}, routing::get, Router};
use serde::Deserialize;
use tokio::net::TcpListener;




#[derive(Deserialize)]
struct Page {
    number: u32
}



/* show item method contain path and query parameter for path parameter rquest url will be item/:item and Query will be ?number=number type
    here page struct need to deserialize 
 */
async fn show_item(Path(id):Path<String>, Query(page): Query<Page>) -> String {
    println!("Show item handler");
    format!("Item {} on page {} ", id, page.number)
}


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/file3", get(|| async {"file 3"}))
        .route("/item/:id", get(show_item))
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