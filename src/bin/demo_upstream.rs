use std::time::Duration;

use axum::{
    Json, Router,
    body::Bytes,
    extract::Query,
    http::Method,
    routing::{any, get},
};

use serde::Deserialize;
use serde_json::json;
use tokio::net::TcpListener;

#[derive(Debug, Deserialize)]
struct EchoQuery {
    name: Option<String>,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/hello", get(hello))
        .route("/slow", get(slow))
        .route("/echo", any(echo));

    let listener = TcpListener::bind("127.0.0.1:4000")
        .await
        .expect("绑定4000端口失败");
    println!("Demo upstream listing on http://127.0.0.1:4000");

    axum::serve(listener, app).await.expect("上游服务启动失败");
}

async fn hello() -> Json<serde_json::Value> {
    Json(json!({
        "message":"hello from upsteram!",
        "service":"demo-upstream"
    }))
}

    method: Method,
async fn echo(
    Query(query): Query<EchoQuery>,
    body: Bytes,
) -> Json<serde_json::Value> {
    let body_text = String::from_utf8_lossy(&body).to_string();

    Json(json!({
        "method":method.to_string(),
        "name":query.name,
        "body":body_text,
        "service":"demo-upstream"
    }))
}

async fn slow() -> Json<serde_json::Value> {
    tokio::time::sleep(Duration::from_secs(5)).await;

    Json(json!({
        "message":"slow response",
        "service":"demo-upstream"
    }))
}
