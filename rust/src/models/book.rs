use serde::{Deserialize, Serialize};

/// 对应 `books` 表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Book {
    pub book_id: String,
    pub name: String,
    pub author: String,
    pub kind: Option<String>,
    pub cover_url: Option<String>,
    pub intro: Option<String>,
    pub origin: Option<String>, // 书源
    pub book_type: String,
    pub word_count: u32,
    pub latest_chapter_title: Option<String>,
    pub toc_url: Option<String>,
    pub book_group: Option<String>,
    /// 添加时间，使用时间戳（秒）
    pub add_time: i64,
    pub last_read_time: Option<i64>,
    pub isbn: Option<String>,
    pub publisher: Option<String>,
    pub publish_date: Option<i64>,
    pub status: String,
}

impl Default for Book {
    fn default() -> Self {
        Self {
            book_id: String::new(),
            name: String::new(),
            author: String::new(),
            kind: None,
            cover_url: None,
            intro: None,
            origin: None,
            book_type: String::new(),
            word_count: 0,
            latest_chapter_title: None,
            toc_url: None,
            book_group: None,
            add_time: 0,
            last_read_time: None,
            isbn: None,
            publisher: None,
            publish_date: None,
            status: String::new(),
        }
    }
}