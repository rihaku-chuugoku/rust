use axum::{
    Json,
    extract::{Query, rejection::QueryRejection},
};

use crate::HelloQuery;
use serde::Deserialize;
use serde_json::{Value, json};

use crate::lesson_error::{AppError, AppResult};
#[derive(Debug, Deserialize)]
pub struct helloQuery {
    pub name: String,
}

pub async fn hello(query: Result<Query<helloQuery>, QueryRejection>) -> AppResult<Json<Value>> {
    let Query(input) = query.map_err(|_| AppError::BadRequest("查询参数格式不正确".to_string()))?;

    let name = input.name.trim();

    if name.is_empty() {
        return Err(AppError::BadRequest("name不能为空".to_string()));
    }

    Ok(Json(json!({
        "message":format!("你好，{name}!")
    })))
}
