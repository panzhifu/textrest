# 数据结构汇总（含书源与规则）

> 说明：以下为当前项目 `rust/src/models/` 中的完整数据结构汇总。书源与规则结构保持不变，仅整理输出。

---

## Book

```rust
pub struct Book {
    pub book_id: String,
    pub name: String,
    pub author: String,
    pub kind: Option<String>,
    pub cover_url: Option<String>,
    pub intro: Option<String>,
    pub origin: Option<String>, // 书源
    pub book_type: String,
    pub word_count: i64,
    pub latest_chapter_title: Option<String>,
    pub toc_url: Option<String>,
    pub book_group: Option<String>,
    /// 添加时间，保持为字符串，具体格式由前后端约定（例如 ISO8601）
    pub add_time: String,
    pub last_read_time: Option<String>,
    pub isbn: Option<String>,
    pub publisher: Option<String>,
    pub publish_date: Option<String>,
    pub status: String,
}
```

---

## Chapter

```rust
pub struct Chapter {
    pub id: i32,
    pub book_id: String,
    pub title: String,
    pub url: String,
    pub chapter_index: i32,
    pub content: Option<String>,
    pub word_count: i32,
    pub is_vip: bool,
}
```

---

## ReadRecord

```rust
pub struct ReadRecord {
    pub book_id: String,
    pub dur_chapter_index: i64,
    pub dur_chapter_pos: i64,
    pub last_chapter_index: i64,
    pub last_chapter_pos: i64,
    pub total_read_time: i64,
    /// 文档在索引优化中引用了 `last_read_time`，这里额外加一个字段以便扩展
    pub last_read_time: Option<String>,
}
```

---

## BookSource（书源结构）

```rust
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
```

---

# 书源规则（Rules）

## BookInfoRule

```rust
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
    pub can_re_name: Option<String>,
    #[serde(alias = "downloadUrls")]
    pub download_urls: Option<String>,
}
```

## ContentRule

```rust
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
```

## SearchRule

```rust
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
```

## TocRule

```rust
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
    pub is_volume: Option<String>,
    #[serde(alias = "isVip", alias = "is_vip")]
    pub is_vip: Option<String>,
    #[serde(alias = "isPay", alias = "is_pay")]
    pub is_pay: Option<String>,
    #[serde(alias = "updateTime", alias = "update_time")]
    pub update_time: Option<String>,
    #[serde(alias = "nextTocUrl", alias = "next_toc_url")]
    pub next_toc_url: Option<String>,
}
```

## ReviewRule

```rust
pub struct ReviewRule {
    pub init: Option<String>,
    pub list: Option<String>,
    pub name: Option<String>,
    pub content: Option<String>,
    pub time: Option<String>,
    pub rating: Option<String>,
    #[serde(alias = "nextPage", alias = "next_page")]
    pub next_page: Option<String>,
}
```

## ExploreRule

```rust
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
    pub update_time: Option<String>,
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
```
