use std::net::SocketAddr;

use serde::Deserialize;

use crate::lesson_error::{AppError, AppResult};

//整个应用配置
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub upstream: UpstreamConfig,
}

//HTTP服务器配置
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServerConfig {
    pub bind: SocketAddr,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpstreamConfig {
    pub base_url: String,
    pub timeout_ms: u64,
    pub max_request_bytes: usize,
}

impl AppConfig {
    pub fn load(path: &str) -> AppResult<Self> {
        //
        let settings: Self = config::Config::builder()
            .add_source(config::File::from(std::path::Path::new(path)))
            .build()
            .map_err(|error| AppError::Config(format!("读取配置失败:{error}")))?
            .try_deserialize()
            .map_err(|error| AppError::Config(format!("解析配置失败: {error}")))?;

        //
        settings.validate()?;

        Ok(settings)
    }

    fn validate(&self) -> AppResult<()> {
        //
        if self.upstream.timeout_ms == 0 {
            return Err(AppError::Config("upstream.timeout_ms不能为0".to_string()));
        }

        if self.upstream.max_request_bytes == 0 {
            return Err(AppError::Config(
                "upstream.max_request_bytes不能为0".to_string(),
            ));
        }

        if self.server.bind.port() == 0 {
            return Err(AppError::Config("server.bind端口不能为0".to_string()));
        }
        if !self.upstream.base_url.starts_with("http://")
            && !self.upstream.base_url.starts_with("https://")
        {
            return Err(AppError::Config(
                "upstream.base_url必须以http://或者https://开头".to_string(),
            ));
        }
        Ok(())
    }
}
