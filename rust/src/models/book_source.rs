use serde::{Deserialize, Serialize};
use crate::models::rules::{BookInfoRule, ContentRule, SearchRule, TocRule, ReviewRule, ExploreRule};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookSource {
    pub book_source_url: String,
    pub book_source_name: String,
    pub book_source_group: Option<String>,
    pub book_source_type: i32,
    pub book_url_pattern: Option<String>,
    pub custom_order: i32,
    pub enabled: bool,
    pub enabled_explore: bool,
    pub js_lib: Option<String>,
    pub enabled_cookie_jar: bool,
    pub concurrent_rate: Option<String>,
    pub header: Option<String>,
    pub login_url: Option<String>,
    pub login_ui: Option<String>,
    pub login_check_js: Option<String>,
    pub cover_decode_js: Option<String>,
    pub book_source_comment: Option<String>,
    pub variable_comment: Option<String>,
    pub last_update_time: String,
    pub respond_time: i32,
    pub weight: i32,
    pub explore_url: Option<String>,
    pub explore_screen: Option<String>,
    pub rule_explore: Option<ExploreRule>,
    pub search_url: Option<String>,
    pub search_rule: Option<SearchRule>,
    pub book_info_rule: Option<BookInfoRule>,
    pub toc_rule: Option<TocRule>,
    pub content_rule: Option<ContentRule>,
    pub review_rule: Option<ReviewRule>,
}

impl Default for BookSource {
    fn default() -> Self {
        Self {
            book_source_url: String::new(),
            book_source_name: String::new(),
            book_source_group: None,
            book_source_type: 0,
            book_url_pattern: None,
            custom_order: 0,
            enabled: true,
            enabled_explore: false,
            js_lib: None,
            enabled_cookie_jar: false,
            concurrent_rate: None,
            header: None,
            login_url: None,
            login_ui: None,
            login_check_js: None,
            cover_decode_js: None,
            book_source_comment: None,
            variable_comment: None,
            last_update_time: chrono::Local::now().to_rfc3339(),
            respond_time: 0,
            weight: 0,
            explore_url: None,
            explore_screen: None,
            rule_explore: None,
            search_url: None,
            search_rule: None,
            book_info_rule: None,
            toc_rule: None,
            content_rule: None,
            review_rule: None,
        }
    }
}

