//!二进制入口，配置校验，运维命令，初始化依赖，双监听器和退出。
use axum::{
    Json, Router,
    extract::{Query, rejection::QueryRejection},
    routing::get,
};

mod app_state;
mod handlers;
mod lesson_config;
mod lesson_error;

use app_state::AppState;
use lesson_config::AppConfig;
use lesson_error::{AppError, AppResult};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;

//接收URL查询参数
#[derive(Debug, Deserialize)]
struct HelloQuery {
    name: String,
}

//程序入口
#[tokio::main]
async fn main() -> AppResult<()> {
    //1.加载配置
    let settings = AppConfig::load("config/lesson.toml")?;

    let http_client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .map_err(|error| AppError::Internal(format!("创建HTTP客户端失败:{error}")))?;

    let state = AppState {
        service_name: "rate-limiter-gateway".to_string(),

        http_client: Arc::new(http_client),

        upstream_base_url: settings.upstream.base_url.clone(),

        upstream_timeout_ms: settings.upstream.timeout_ms,

        max_request_bytes: settings.upstream.max_request_bytes,
    };
    //2.创建HTTP路由
    let app = Router::new()
        .route("/debug", get(|| async { "OK" }))
        .route("/api/client-info", get(handlers::clinet_info))
        .route("/health", get(handlers::health))
        .route("/api/hello", get(handlers::hello))
        .route("/proxy/{*path}", axum::routing::any(handlers::proxy))
        .with_state(state);

    //3.绑定监听地址
    let listener = TcpListener::bind(settings.server.bind)
        .await
        .map_err(|error| AppError::Internal(format!("绑定监听地址失败:{error}")))?;

    println!(
        "Geteway listening on {}",
        listener
            .local_addr()
            .map_err(|error| AppError::Internal(format!("获取监听地址失败:{error}")))?
    );
    //4.启动HTTP服务
    axum::serve(listener, app)
        .await
        .map_err(|error| AppError::Internal(format!("HTTP服务运行失败:{error}")))?;

    Ok(())
}
