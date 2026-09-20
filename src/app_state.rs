use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub service_name: String,              //应用信息
    pub http_client: Arc<reqwest::Client>, //共享基础设置
    pub upstream_base_url: String,
    pub upstream_timeout_ms: u64,
    pub max_request_bytes: usize, //运行配置
}
