use axum::{
    extract::{Extension, Path, State},
    routing::{get, post},
    Json, Router,
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
    headers: HeaderMap,
    body: Bytes,
) -> Json<serde_json::Value> {
    println!("GET body: {:?}", body);
    println!("GET headers: {:?}", headers);
    let k="hello";
    let v="universe";
    let map = data.lock().await;
    println!("value of 'hello': {:?}", map.get("hello"));
    println!("entire map: {:?}", *map);

    Json(json!({
        // "key": key,
        "key": "key",
        // "value": value,
         "value": "value",
        "status": "ok"
    }))
}

async fn changevalue(
    State(data): State<Arc<Mutex<HashMap<String, String>>>>,
    headers: HeaderMap,
    body: Bytes,
) -> Json<serde_json::Value> {
    println!("POST body: {:?}", body);
    println!("POST headers: {:?}", headers);
    let a = str::from_utf8(&body).unwrap();
    println!("a: {:?}", a);
    let json: Value = serde_json::from_str(a).unwrap();
    println!("json is  {:?}", json);


    let k="hello";
    let v="universe";
    let mut map = data.lock().await;
    map.insert(k.to_string(), v.to_string());
    println!("value of 'hello': {:?}", map.get("hello"));
    println!("entire map: {:?}", *map);
    //  if let Some(obj) = json.as_object() {
    //     for key in obj.keys() {
    //         println!("kv: {}, {:?}", key, json[key]);
            //  let mut map = data.lock().await;  // mutable access
            //  let value = json[key];
            // map.insert(key.clone(), value.clone());
        //}
    //}

    return Json(json!({
        // "key": key,
        "key": "key",
        // "new_value": new_value,
        "status": "updated"
    }));
    
    }


    async fn insertkeyvalue(){}
    async fn getallkeyvalue(){}