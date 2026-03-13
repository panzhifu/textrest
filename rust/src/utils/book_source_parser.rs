use std::fs::File;
use std::io::Read;

use anyhow::{Context, Result};
use serde_json::Value;

use crate::models::book_source::BookSource;

pub struct BookSourceParser;

impl BookSourceParser {
    pub fn from_text(text: &str) -> Result<Vec<BookSource>> {
        let value = serde_json::from_str::<Value>(text)
            .context("无效的书源JSON格式")?;
        
        // 检查是否是数组
        if let Value::Array(array) = value {
            if array.is_empty() {
                return Err(anyhow::anyhow!("书源数组为空"));
            }
            
            // 解析所有书源
            let mut sources = Vec::new();
            for item in array {
                let source = Self::parse_from_json(item)?;
                sources.push(source);
            }
            return Ok(sources);
        }
        
        // 否则解析为单个书源
        let source = Self::parse_from_json(value)?;
        Ok(vec![source])
    }
    
    pub async fn from_url(url: &str) -> Result<Vec<BookSource>> {
        // 创建一个具有更长超时时间的HTTP客户端
        let config = crate::models::config::HttpClientConfig::default();
        let http_client = crate::network::HttpClient::new(config)?;
        
        let response = http_client.get(url).await
            .context(format!("获取书源失败: {}", url))?;
        let text = response.text().await
            .context("读取响应内容失败")?;
        
        Self::from_text(&text)
    }
    
    pub fn from_file(file_path: String) -> Result<Vec<BookSource>> {
        let mut file = File::open(file_path).context("打开书源文件失败")?;
        let mut text = String::new();
        file.read_to_string(&mut text).context("读取书源文件失败")?;
        Self::from_text(&text)
    }
    
    fn parse_from_json(value: Value) -> Result<BookSource> {
        let mut source = BookSource::default();
        
        source.book_source_url = value.get("bookSourceUrl")
            .and_then(|v| v.as_str())
            .context("缺少 bookSourceUrl 字段")?
            .to_string();
        
        source.book_source_name = value.get("bookSourceName")
            .and_then(|v| v.as_str())
            .context("缺少 bookSourceName 字段")?
            .to_string();
        
        if let Some(v) = value.get("bookSourceGroup").and_then(|v| v.as_str()) {
            source.book_source_group = Some(v.to_string());
        }
        if let Some(v) = value.get("bookSourceType").and_then(|v| v.as_i64()) {
            source.book_source_type = v as i32;
        }
        if let Some(v) = value.get("bookUrlPattern").and_then(|v| v.as_str()) {
            source.book_url_pattern = Some(v.to_string());
        }
        if let Some(v) = value.get("customOrder").and_then(|v| v.as_i64()) {
            source.custom_order = v as i32;
        }
        if let Some(v) = value.get("enabled").and_then(|v| v.as_bool()) {
            source.enabled = v;
        }
        if let Some(v) = value.get("enabledExplore").and_then(|v| v.as_bool()) {
            source.enabled_explore = v;
        }
        if let Some(v) = value.get("jsLib").and_then(|v| v.as_str()) {
            source.js_lib = Some(v.to_string());
        }
        if let Some(v) = value.get("enabledCookieJar").and_then(|v| v.as_bool()) {
            source.enabled_cookie_jar = v;
        }
        if let Some(v) = value.get("concurrentRate").and_then(|v| v.as_str()) {
            source.concurrent_rate = Some(v.to_string());
        }
        if let Some(v) = value.get("header").and_then(|v| v.as_str()) {
            source.header = Some(v.to_string());
        }
        if let Some(v) = value.get("loginUrl").and_then(|v| v.as_str()) {
            source.login_url = Some(v.to_string());
        }
        if let Some(v) = value.get("loginUi").and_then(|v| v.as_str()) {
            source.login_ui = Some(v.to_string());
        }
        if let Some(v) = value.get("loginCheckJs").and_then(|v| v.as_str()) {
            source.login_check_js = Some(v.to_string());
        }
        if let Some(v) = value.get("coverDecodeJs").and_then(|v| v.as_str()) {
            source.cover_decode_js = Some(v.to_string());
        }
        if let Some(v) = value.get("bookSourceComment").and_then(|v| v.as_str()) {
            source.book_source_comment = Some(v.to_string());
        }
        if let Some(v) = value.get("variableComment").and_then(|v| v.as_str()) {
            source.variable_comment = Some(v.to_string());
        }
        if let Some(v) = value.get("lastUpdateTime").and_then(|v| v.as_str()) {
            source.last_update_time = v.to_string();
        }
        if let Some(v) = value.get("respondTime").and_then(|v| v.as_i64()) {
            source.respond_time = v as i32;
        }
        if let Some(v) = value.get("weight").and_then(|v| v.as_i64()) {
            source.weight = v as i32;
        }
        if let Some(v) = value.get("exploreUrl").and_then(|v| v.as_str()) {
            source.explore_url = Some(v.to_string());
        }
        if let Some(v) = value.get("exploreScreen").and_then(|v| v.as_str()) {
            source.explore_screen = Some(v.to_string());
        }
        if let Some(v) = value.get("searchUrl").and_then(|v| v.as_str()) {
            source.search_url = Some(v.to_string());
        }
        
        // 尝试从不同格式的字段名中获取规则
        if let Some(v) = value.get("ruleExplore").or_else(|| value.get("exploreRule")) {
            source.rule_explore = serde_json::from_value(v.clone()).ok();
        }
        if let Some(v) = value.get("searchRule").or_else(|| value.get("ruleSearch")) {
            source.search_rule = serde_json::from_value(v.clone()).ok();
        }
        if let Some(v) = value.get("bookInfoRule").or_else(|| value.get("ruleBookInfo")) {
            source.book_info_rule = serde_json::from_value(v.clone()).ok();
        }
        if let Some(v) = value.get("tocRule").or_else(|| value.get("ruleToc")) {
            source.toc_rule = serde_json::from_value(v.clone()).ok();
        }
        if let Some(v) = value.get("contentRule").or_else(|| value.get("ruleContent")) {
            source.content_rule = serde_json::from_value(v.clone()).ok();
        }
        if let Some(v) = value.get("reviewRule").or_else(|| value.get("ruleReview")) {
            source.review_rule = serde_json::from_value(v.clone()).ok();
        }
        
        Ok(source)
    }

}
