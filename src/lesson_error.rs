use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use serde_json::json;

//应用统一错误类型
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("配置错误：{0}")]
    Config(String),

    #[error("请求参数错误,{0}")]
    BadRequest(String),

    #[error("请求体过大")]
    PayloadTooLarge,

    #[error("上游服务连接失败,:{0}")]
    BadeGateway(String),

    #[error("上游服务超时")]
    GatewayTimeout,

    #[error("内部错误,{0}")]
    Internal(String),
}

//
pub type AppResult<T> = Result<T, AppError>;

//
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            AppError::Config(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "config_error",
                "服务配置错误",
            ),

            AppError::BadRequest(message) => {
                (StatusCode::BAD_REQUEST, "bad_request", message.as_str())
            }

            AppError::PayloadTooLarge => (
                StatusCode::PAYLOAD_TOO_LARGE,
                "payload_too_large",
                "请求体过大",
            ),

            AppError::GatewayTimeout => (
                StatusCode::GATEWAY_TIMEOUT,
                "gateway_timeout",
                "上游服务响应超时",
            ),

            AppError::Internal(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "服务暂时不可用",
            ),
        };

        if status.is_server_error() {
            tracing::error!(error =%self,
                "request failed");
        }

        (
            status,
            Json(json!({
                "error":{
                    "code":code,
                    "message":message,
                }
            })),
        )
            .into_response()
    }
}
