use axum::{Router, routing::{get, post}};

use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

mod lib;
use lib::handlers::{getvalue, insertkeyvalue, getallkeyvalue};


#[tokio::main]
async fn main() {
    let hashmap_data: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::<String, String>::new()));
    // let refer_data: &Arc<Mutex<HashMap<String, String>>> = &data;

    let app = Router::new()
        .route("/getvalue/{key}", get(getvalue))
        .route("/insertkeyvalue", post(insertkeyvalue))
        .route("/getallkeyvalue", get(getallkeyvalue))
        .with_state(hashmap_data);

    // let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    // axum::serve(listener, app).await.unwrap();


    let listener = match tokio::net::TcpListener::bind("0.0.0.0:8000").await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to bind: {}", e);
            return;
        }
    };

    if let Err(e) = axum::serve(listener, app).await {
        eprintln!("Server error: {}", e);
    }

}