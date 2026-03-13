use std::collections::HashMap;

/// HTTP 客户端配置
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HttpClientConfig {
    /// 超时时间（秒）
    pub timeout: u64,
    /// 并发限制
    pub max_concurrent: usize,
    /// 是否启用 Cookie Jar
    pub enable_cookie_jar: bool,
    /// 默认请求头
    pub default_headers: HashMap<String, String>,
}

impl Default for HttpClientConfig {
    fn default() -> Self {
        Self {
            timeout: 30,
            max_concurrent: 10,
            enable_cookie_jar: false,
            default_headers: HashMap::new(),
        }
    }
}

/// 存储配置
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StorageConfig {
    /// 书籍存储位置
    pub book_storage_path: String,
    /// 日志存储位置
    pub log_storage_path: String,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            book_storage_path: String::new(),
            log_storage_path: String::new(),
        }
    }
}

/// 应用配置
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub http_client_config: HttpClientConfig,
    pub storage_config: StorageConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            http_client_config: HttpClientConfig::default(),
            storage_config: StorageConfig::default(),
        }
    }
}