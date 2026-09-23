use axum::{Json, Router, routing::get};
use serde_json::{Value, json};

async fn hello_world() -> &'static str {
    "Hello axum"
}

async fn hello_json() -> Json<Value> {
    Json(json!({ "data": 42 }))
}

const PORT: u16 = 6666;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/json", get(hello_json));

    let addr = format!("0.0.0.0:{PORT}");
    let listener = tokio::net::TcpListener::bind(addr).await.expect("Listener");

    println!("Server listening port 6666");
    axum::serve(listener, app).await.expect("Could not serve");
}
