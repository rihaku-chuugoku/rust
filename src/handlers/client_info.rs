use axum::{Json, extract::State};

use serde_json::{Value, json};

use crate::app_state::AppState;

pub async fn clinet_info(State(state): State<AppState>) -> Json<Value> {
    let count = std::sync::Arc::strong_count(&state.http_client);

    Json(json!({
     "service": state.service_name,
     "http_client_shared":true,
     "arc_strong_count":count
    }))
}
