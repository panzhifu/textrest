use serde::{Deserialize, Serialize};

/// 4.4.1 BookInfoRule (书籍信息规则)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookInfoRule {
    pub init: Option<String>,
    pub name: Option<String>,
    pub author: Option<String>,
    pub intro: Option<String>,
    pub kind: Option<String>,
    #[serde(alias = "lastChapter")]
    pub last_chapter: Option<String>,
    #[serde(alias = "updateTime")]
    pub update_time: Option<String>,
    #[serde(alias = "coverUrl")]
    pub cover_url: Option<String>,
    #[serde(alias = "tocUrl")]
    pub toc_url: Option<String>,
    #[serde(alias = "wordCount")]
    pub word_count: Option<String>,
    #[serde(alias = "canReName")]
    pub can_re_name: Option<bool>,
    #[serde(alias = "downloadUrls")]
    pub download_urls: Option<String>,
}

/// 4.4.2 ContentRule (正文页规则)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentRule {
    pub init: Option<String>,
    pub content: Option<String>,
    #[serde(alias = "nextContent", alias = "next_content")]
    pub next_content: Option<String>,
    #[serde(alias = "prevContent", alias = "prev_content")]
    pub prev_content: Option<String>,
    #[serde(alias = "refreshContent", alias = "refresh_content")]
    pub refresh_content: Option<String>,
    #[serde(alias = "replaceRegex", alias = "replace_regex")]
    pub replace_regex: Option<String>,
    #[serde(alias = "removeRegex", alias = "remove_regex")]
    pub remove_regex: Option<String>,
    #[serde(alias = "cssSelector", alias = "css_selector")]
    pub css_selector: Option<String>,
    #[serde(alias = "isWebView", alias = "is_web_view")]
    pub is_web_view: Option<bool>,
    pub js: Option<String>,
    pub encode: Option<String>,
}

/// 4.4.3 SearchRule (搜索规则)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchRule {
    pub init: Option<String>,
    #[serde(alias = "bookList", alias = "book_list")]
    pub list: Option<String>,
    pub name: Option<String>,
    #[serde(alias = "bookUrl", alias = "book_url")]
    pub url: Option<String>,
    #[serde(alias = "coverUrl", alias = "cover_url")]
    pub cover: Option<String>,
    pub author: Option<String>,
    pub intro: Option<String>,
    pub kind: Option<String>,
    #[serde(alias = "lastChapter", alias = "last_chapter")]
    pub last_chapter: Option<String>,
    #[serde(alias = "updateTime", alias = "update_time")]
    pub update_time: Option<String>,
    #[serde(alias = "wordCount", alias = "word_count")]
    pub word_count: Option<String>,
    #[serde(alias = "nextPage", alias = "next_page")]
    pub next_page: Option<String>,
}

/// 4.4.4 TocRule (目录规则)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TocRule {
    #[serde(alias = "preUpdateJs", alias = "pre_update_js")]
    pub pre_update_js: Option<String>,
    #[serde(alias = "chapterList", alias = "chapter_list")]
    pub chapter_list: Option<String>,
    #[serde(alias = "chapterName", alias = "chapter_name")]
    pub chapter_name: Option<String>,
    #[serde(alias = "chapterUrl", alias = "chapter_url")]
    pub chapter_url: Option<String>,
    #[serde(alias = "formatJs", alias = "format_js")]
    pub format_js: Option<String>,
    #[serde(alias = "isVolume", alias = "is_volume")]
    pub is_volume: Option<bool>,
    #[serde(alias = "isVip", alias = "is_vip")]
    pub is_vip: Option<bool>,
    #[serde(alias = "isPay", alias = "is_pay")]
    pub is_pay: Option<bool>,
    #[serde(alias = "updateTime", alias = "update_time")]
    pub update_time: Option<String>,
    #[serde(alias = "nextTocUrl", alias = "next_toc_url")]
    pub next_toc_url: Option<String>,
}

/// 4.4.5 ReviewRule (评论规则)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewRule {
    pub init: Option<String>,
    pub list: Option<String>,
    pub name: Option<String>,
    pub content: Option<String>,
    pub time: Option<i64>,
    pub rating: Option<String>,
    #[serde(alias = "nextPage", alias = "next_page")]
    pub next_page: Option<String>,
}

/// 4.4.6 ExploreRule (发现规则)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExploreRule {
    pub init: Option<String>,
    pub list: Option<String>,
    pub name: Option<String>,
    #[serde(alias = "bookUrl", alias = "book_url")]
    pub url: Option<String>,
    #[serde(alias = "coverUrl", alias = "cover_url")]
    pub cover: Option<String>,
    pub author: Option<String>,
    pub intro: Option<String>,
    pub kind: Option<String>,
    #[serde(alias = "lastChapter", alias = "last_chapter")]
    pub last_chapter: Option<String>,
    #[serde(alias = "updateTime", alias = "update_time")]
    pub update_time: Option<i64>,
    #[serde(alias = "wordCount", alias = "word_count")]
    pub word_count: Option<String>,
    #[serde(alias = "nextPage", alias = "next_page")]
    pub next_page: Option<String>,
    pub sort: Option<String>,
    pub filter: Option<String>,
    pub search: Option<String>,
    pub comment: Option<String>,
    pub chapter: Option<String>,
    pub content: Option<String>,
}

