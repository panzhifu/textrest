use serde::{Deserialize, Serialize};

/// 对应 `chapters` 表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    pub id: u32,
    pub book_id: String,
    pub title: String,
    pub url: String,
    pub chapter_index: u32,
    pub word_count: u32,
    pub is_vip: bool,
}

impl Default for Chapter {
    fn default() -> Self {
        Self {
            id: 0,
            book_id: String::new(),
            title: String::new(),
            url: String::new(),
            chapter_index: 0,
            word_count: 0,
            is_vip: false,
        }
    }
}
