use std::env;

use axum::{Router, routing::get};
use dotenv::dotenv;

use mongodb::{Client, options::ClientOptions};

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

    let port = env::var("PORT").expect("PORT must be set");

    let bind_address = format!("0.0.0.0:{}", port);


    let client = connect_mongo().await.unwrap();
    for db_name in client.list_database_names(None, None).await.expect("Failed to list databases") {
        println!("{}", db_name);
    }

    let db = client.database("toudou");

// List the names of the collections in that database.
    for collection_name in db.list_collection_names(None).await.expect("Failed to list collections") {
        println!("Collection {}", collection_name);
    }


    println!("Listening on: {}", bind_address);
    let listener = tokio::net::TcpListener::bind(bind_address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn connect_mongo() -> Result<Client, mongodb::error::Error> {
    let mongo_uri = env::var("MONGODB_URI").expect("MONGO uri to be set");
    let mut client_options = ClientOptions::parse(format!("mongodb://{}/", mongo_uri)).await.expect("Failed to create client");

// Manually set an option.
    client_options.app_name = Some("My App".to_string());

// Get a handle to the deployment.
    Client::with_options(client_options)
    // List the names of the databases in that deployment.

}

// basic handler that responds with a static string
async fn healthy() -> &'static str {
    "it works"
}
