use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::Duration;

use lazy_static::lazy_static;
use std::sync::Mutex;

use anyhow::{Context, Result};
use reqwest::{Client, ClientBuilder, Response};
use tokio::sync::Semaphore;
use tokio::time::timeout;

use crate::models::book_source::BookSource;
use crate::models::config::HttpClientConfig;

const CLIENT_CACHE_CAPACITY: usize = 16;

#[derive(Default)]
struct ClientCache {
    entries: HashMap<String, Arc<Client>>,
    lru: VecDeque<String>,
}

impl ClientCache {
    fn get(&mut self, key: &str) -> Option<Arc<Client>> {
        if let Some(client) = self.entries.get(key).cloned() {
            self.touch(key);
            return Some(client);
        }
        None
    }

    fn insert(&mut self, key: String, client: Arc<Client>) {
        if !self.entries.contains_key(&key) && self.entries.len() >= CLIENT_CACHE_CAPACITY {
            if let Some(evicted) = self.lru.pop_front() {
                self.entries.remove(&evicted);
            }
        }

        self.entries.insert(key.clone(), client);
        self.touch(&key);
    }

    fn touch(&mut self, key: &str) {
        if let Some(position) = self.lru.iter().position(|k| k == key) {
            self.lru.remove(position);
        }
        self.lru.push_back(key.to_string());
    }

    fn clear(&mut self) {
        self.entries.clear();
        self.lru.clear();
    }
}

lazy_static! {
    static ref CLIENT_CACHE: Mutex<ClientCache> = Mutex::new(ClientCache::default());
}

/// HTTP 客户端封装，支持基本的网络请求功能
pub struct HttpClient {
    client: Arc<Client>,
    config: HttpClientConfig,
    /// 并发控制信号量
    semaphore: Option<Arc<Semaphore>>,
    /// 并发率
    concurrent_rate: Option<String>,
    cache_key: String,
}

impl HttpClient {
    /// 创建新的 HTTP 客户端
    pub fn new(config: HttpClientConfig) -> Result<Self> {
        let cache_key = Self::build_cache_key(&config, None);
        let client = Self::get_or_create_client(&cache_key, &config)?;

        // 初始化并发控制信号量
        let semaphore = if config.max_concurrent > 0 {
            Some(Arc::new(Semaphore::new(config.max_concurrent)))
        } else {
            None
        };

        Ok(Self {
            client,
            config,
            semaphore,
            concurrent_rate: None,
            cache_key,
        })
    }

    /// 从书源创建 HTTP 客户端
    pub fn from_book_source(book_source: &BookSource, base_config: Option<HttpClientConfig>) -> Result<Self> {
        // 使用基础配置或默认配置
        let mut config = base_config.unwrap_or_default();

        // 从书源覆盖配置
        config.enable_cookie_jar = book_source.enabled_cookie_jar;

        // 解析并添加书源的请求头
        if let Some(header_str) = &book_source.header {
            if let Ok(headers) = serde_json::from_str::<HashMap<String, String>>(header_str) {
                config.default_headers.extend(headers);
            }
        }

        // 处理并发率参数
        let concurrent_rate = book_source.concurrent_rate.clone();
        if let Some(rate_str) = &concurrent_rate {
            // 尝试解析并发率为数字
            if let Ok(max_concurrent) = rate_str.parse::<usize>() {
                config.max_concurrent = max_concurrent;
            }
        }

        let cache_key = Self::build_cache_key(&config, Some(book_source.book_source_url.as_str()));
        let max_concurrent = config.max_concurrent;
        let client = Self::get_or_create_client(&cache_key, &config)?;

        let http_client = Self {
            client,
            config,
            semaphore: if max_concurrent > 0 {
                Some(Arc::new(Semaphore::new(max_concurrent)))
            } else {
                None
            },
            concurrent_rate,
            cache_key,
        };

        Ok(http_client)
    }

    /// 发送 GET 请求
    pub async fn get(&self, url: &str) -> Result<Response> {
        let request = self.client.get(url);
        self.send_request(request).await
    }

    /// 发送 POST 请求
    pub async fn post(&self, url: &str, body: &str) -> Result<Response> {
        let request = self.client.post(url).body(body.to_string());
        self.send_request(request).await
    }

    /// 发送带自定义请求头的 GET 请求
    pub async fn get_with_headers(&self, url: &str, headers: &HashMap<String, String>) -> Result<Response> {
        let mut request = self.client.get(url);
        request = self.add_headers(request, headers);
        self.send_request(request).await
    }

    /// 发送带自定义请求头的 POST 请求
    pub async fn post_with_headers(&self, url: &str, body: &str, headers: &HashMap<String, String>) -> Result<Response> {
        let mut request = self.client.post(url).body(body.to_string());
        request = self.add_headers(request, headers);
        self.send_request(request).await
    }

    /// 发送请求并处理超时
    async fn send_request(&self, request: RequestBuilder) -> Result<Response> {
        let request = self.add_default_headers(request);
        let request = request.build()?;
        
        // 处理并发控制
        let permit = if let Some(semaphore) = &self.semaphore {
            Some(semaphore.acquire().await?)
        } else {
            None
        };
        
        let response = timeout(
            Duration::from_secs(self.config.timeout),
            self.client.execute(request)
        )
        .await
        .context("Request timed out")?
        .context("Failed to execute request");

        // 许可会在超出作用域时自动释放
        drop(permit);

        response
    }

    /// 添加默认请求头
    fn add_default_headers(&self, mut request: RequestBuilder) -> RequestBuilder {
        for (key, value) in &self.config.default_headers {
            request = request.header(key, value);
        }
        request
    }

    fn build_cache_key(config: &HttpClientConfig, source_url: Option<&str>) -> String {
        let mut headers: Vec<(&String, &String)> = config.default_headers.iter().collect();
        headers.sort_by(|a, b| a.0.cmp(b.0));

        let header_key = headers
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join(";");

        format!(
            "{}|{}|{}|{}|{}",
            source_url.unwrap_or_default(),
            config.timeout,
            config.enable_cookie_jar,
            config.max_concurrent,
            header_key,
        )
    }

    fn get_or_create_client(cache_key: &str, config: &HttpClientConfig) -> Result<Arc<Client>> {
        let mut cache = CLIENT_CACHE.lock().unwrap();
        if let Some(client) = cache.get(cache_key) {
            return Ok(client);
        }

        let mut builder = ClientBuilder::new()
            .timeout(Duration::from_secs(config.timeout))
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.127 Safari/537.36")
            .danger_accept_invalid_certs(true)
            .pool_idle_timeout(Duration::from_secs(60))
            .pool_max_idle_per_host(8);

        if config.enable_cookie_jar {
            builder = builder.cookie_store(true);
        }

        let client = Arc::new(builder.build()?);
        cache.insert(cache_key.to_string(), client.clone());
        Ok(client)
    }

    /// 添加自定义请求头
    fn add_headers(&self, mut request: RequestBuilder, headers: &HashMap<String, String>) -> RequestBuilder {
        for (key, value) in headers {
            request = request.header(key, value);
        }
        request
    }

    /// 下载文件到指定路径
    pub async fn download_file(&self, url: &str, path: &std::path::Path) -> Result<()> {
        let response = self.get(url).await?;
        let mut file = std::fs::File::create(path)?;
        let content = response.bytes().await?;
        std::io::copy(&mut content.as_ref(), &mut file)?;
        Ok(())
    }

    /// 获取配置
    pub fn config(&self) -> &HttpClientConfig {
        &self.config
    }

    /// 更新配置
    pub fn update_config(&mut self, config: HttpClientConfig) -> Result<()> {
        *self = Self::new(config)?;
        Ok(())
    }

    /// 清理缓存的 HTTP 客户端
    pub fn clear_cached_clients() {
        CLIENT_CACHE.lock().unwrap().clear();
    }
}

/// 请求构建器类型别名
type RequestBuilder = reqwest::RequestBuilder;

/// 构建默认的 HTTP 客户端
pub fn default_http_client() -> Result<HttpClient> {
    HttpClient::new(HttpClientConfig::default())
}

/// 构建启用了 Cookie Jar 的 HTTP 客户端
pub fn cookie_http_client() -> Result<HttpClient> {
    let mut config = HttpClientConfig::default();
    config.enable_cookie_jar = true;
    HttpClient::new(config)
}
