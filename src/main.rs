use std::env;

use axum::{Router, routing::get};
use dotenv::dotenv;

mod tasks;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/health", get(healthy))
        .route("/tasks", get(tasks::get_tasks));

    // Load the .env file
    dotenv().ok();

/*    let address = env::var("ADDRESS").expect("ADDRESS must be set");*/
    let port = env::var("PORT").expect("PORT must be set");

    let bind_address = format!("0.0.0.0:{}", port);

    println!("Listening on: {}", bind_address);
    let listener = tokio::net::TcpListener::bind(bind_address).await.unwrap(); //net::TcpListener::bind(&bind_address.parse().unwrap()).await.unwrap();
    axum::serve(listener, app).await.unwrap();

}

// basic handler that responds with a static string
async fn healthy() -> &'static str {
    "it works"
}
