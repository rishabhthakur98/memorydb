use axum::{
    Json, Router, body::Body, extract::{Extension, Path, State}, response:: Response, routing::{get, post}
};
use serde_json::{json, Value};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use axum::body::Bytes;
use axum::http::HeaderMap;



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

async fn getvalue(
    State(data): State<Arc<Mutex<HashMap<String, String>>>>,
    Path(key): Path<String>,
    headers: HeaderMap,
    body: Bytes
) -> Response<(Body)> {
    let map = data.lock().await;
    println!("entire map: {:?}", *map);
    let value = map.get(&key).unwrap();
    let res_body:String = format!(r#"{{"{}":"{:?}"}}"#, key, value);
    let body = Body::from(res_body);
    let response = Response::builder()
    .status(200)
    .header("Content-Type", "application/json")
    .body(body)
    .unwrap();
    return response
}



    async fn insertkeyvalue(
    State(data): State<Arc<Mutex<HashMap<String, String>>>>,
    headers: HeaderMap,
    body: Bytes)
    -> Response<(Body)> {
    let a = str::from_utf8(&body).unwrap();
    println!("a: {:?}", a);
// Step 1: parse outer string -> gives inner JSON string
let inner = serde_json::from_str::<String>(a).unwrap();
// Step 2: parse inner JSON to real object
let json_data: Value = serde_json::from_str(&inner).unwrap();

let mut map = data.lock().await;

for (key, value) in json_data.as_object().unwrap() {
    map.insert(key.clone(), value.as_str().unwrap_or(&value.to_string()).to_string());
}
    
   
    println!("entire map: {:?}", *map);
    let res_body:String = format!(r#"{{"status":"done"}}"#);
    let body = Body::from(res_body);
    let response = Response::builder()
    .status(200)
    .header("Content-Type", "application/json")
    .body(body)
    .unwrap();
    return response
    }



    async fn getallkeyvalue(){}