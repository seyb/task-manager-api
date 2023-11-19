use std::time::SystemTime;

use axum::{Json, Router, routing::get};
use axum::http::StatusCode;
use serde::Serialize;

use std::env;
use dotenv::dotenv;
#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/health", get(healthy))
        .route("/tasks", get(get_tasks));

    // Load the .env file
    dotenv().ok();

    let address = env::var("ADDRESS").expect("ADDRESS must be set");
    let port = env::var("PORT").expect("PORT must be set");

    let bind_address = format!("{}:{}", address, port);

    axum::Server::bind(&bind_address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn healthy() -> &'static str {
    "it works"
}

#[derive(Serialize)]
struct TasksResponse {
    id: String,
    description: String,
    complete: Option<SystemTime>
}

async fn get_tasks() -> (StatusCode, Json<Vec<TasksResponse>>) {
    let collections = vec![TasksResponse{ id: "test".to_string(), description:"description".to_string(), complete: None }];
    (StatusCode::OK, Json(collections))
}
