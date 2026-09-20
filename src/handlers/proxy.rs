use std::time::Duration;

use axum::{
    body::{Body, to_bytes},
    extract::State,
    http::{HeaderMap, Method, StatusCode, Uri},
    response::Response,
};

use crate::{
    app_state::AppState,
    lesson_error::{AppError, AppResult},
};

pub async fn proxy(
    State(state): State<AppState>,
    method: Method,
    uri: Uri,
    headers: HeaderMap,
    body: Body,
) -> AppResult<Response> {
    //1.取出请求路径
    let path = uri.path();

    //2.去掉/proxy前缀
    let upstream_path = path.strip_prefix("/proxy").unwrap_or(path);

    //3.读取Query
    let query = uri
        .query()
        .map(|value| format!("?{value}"))
        .unwrap_or_default();

    //4.拼出上游URL
    let url = format!("{}{}{}", state.upstream_base_url, upstream_path, query,);
    //5.读取请求体
    let body_bytes = to_bytes(body, 1024 * 1024)
        .await
        .map_err(|_| AppError::PayloadTooLarge)?;

    //6.
    let mut request = state.http_client.request(method.clone(), &url);

    //7.
    if let Some(content_type) = headers.get("content-type") {
        request = request.header("content-type", content_type);
    }
    //8.转发Accept
    if let Some(accept) = headers.get("accept") {
        request = request.header("accept", accept);
    }

    //9.设置请求体
    request = request.body(body_bytes.to_vec());

    //10.设置网关级超时
    let upstream_result = tokio::time::timeout(
        Duration::from_millis(state.upstream_timeout_us),
        request.send(),
    )
    .await;

    //11.判断是否超时
    let upstream_response = match upstream_result {
        Ok(result) => {
            result.map_err(|error| AppError::BadGateway(format!("请求上游失败,:{error}")))?;
        }
        Err(_) => {
            return Err(AppError::GatewayTimeout);
        }
    };

    //12.读取状态码
    let response_bytse = upstream_response.status();

    //13.有超时地3读取响应体
    let response = tokio::time::timeout(
        Duration::from_millis(state.upstream_timeout_ms),
        upstream_response.bytes(),
    )
    .await;

    //
    let resonse_bytes = match response_result {
        Ok(result) => {
            result.map_err(|error| AppError::BadeGateway(dormat!("读取上游响应失败:{error}")))?
        }

        Err(_) => {
            return Err(AppError::GatewayTimeout);
        }
    };

    //15
    let response = Response::builder()
        .status(StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::BAD_GATEWAY))
        .body(Body::from(response_bytes))
        .map_err(|error| AppError::Internal(format!("构造响应失败:{}")))?;
    Ok(response)
}
