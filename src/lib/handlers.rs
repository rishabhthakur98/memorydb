
use axum::{
    Json, body::Body, extract::{ Path, State}, response:: Response
};
use serde_json::{json, Value};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;



pub async fn getvalue(
    State(data): State<Arc<Mutex<HashMap<String, String>>>>,
    Path(key): Path<String>
) -> Response<Body> {
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



   pub async fn insertkeyvalue(
    State(data): State<Arc<Mutex<HashMap<String, String>>>>,
    body: String
)-> Response<Body> {

let json_data: Value = serde_json::from_str(&body).unwrap();

let mut hash_map = data.lock().await;

for (key, value) in json_data.as_object().unwrap() {
    hash_map.insert(key.clone(), value.as_str().unwrap_or(&value.to_string()).to_string());
    }
    let res_body:String = format!(r#"{{"status":"done"}}"#);
    let res_body = Body::from(res_body);

    Response::builder()
    .status(200)
    .header("Content-Type", "application/json")
    .body(res_body)
    .unwrap()
    }



   pub async fn getallkeyvalue(State(data): State<Arc<Mutex<HashMap<String, String>>>>,
)    -> Json<serde_json::Value> {
        println!("This is first");
        let hash_map = data.lock().await;

    Json(json!({

         "data": *hash_map
    }))
    }