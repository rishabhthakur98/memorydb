use axum::{Router, routing::{get, post}};
use tower_http::cors::CorsLayer;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

mod lib;
use lib::handlers::{getvalue, insertkeyvalue, getallkeyvalue};


#[tokio::main]
async fn main() {
    let hashmap_data: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::<String, String>::new()));

    let cors = CorsLayer::permissive();  
    let app = Router::new()
        .route("/getvalue/{key}", get(getvalue))
        .route("/insertkeyvalue", post(insertkeyvalue))
        .route("/getallkeyvalue", get(getallkeyvalue))
        .with_state(hashmap_data)
        .layer(cors);

    match tokio::net::TcpListener::bind("0.0.0.0:8000").await{
        Ok(listener) => {
            match axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal())
            .await
            
            {
                Ok(_server) => {

                },
                Err(e) => {
                        eprintln!("Server error: {}", e);
                }
            }
        },
        Err(e) => {
                eprintln!("Failed to bind: {}", e);
        }
    }

}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install Ctrl+C handler");

    println!("Signal received. Shutting down gracefully...");
}
