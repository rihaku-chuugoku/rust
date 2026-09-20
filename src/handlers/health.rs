use crate::app_state::AppState;
use axum::{Json, extract::State};
use serde_json::{Value, json};

pub async fn health(State(state): State<AppState>) -> Json<Value> {
    Json(json!({
        "status":"ok",
        "service":"tate-limiter-gateway",
        "version":"0.1.0",
        "environment":"development"
    }))
}
