use serde::{Deserialize, Serialize};

/// 对应 `read_records` 表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadRecord {
    pub book_id: String,
    pub dur_chapter_index: u64,
    pub dur_chapter_pos: u64,
    pub last_chapter_index: u64,
    pub last_chapter_pos: u64,
    pub total_read_time: u64,
    /// 文档在索引优化中引用了 `last_read_time`，这里额外加一个字段以便扩展
    pub last_read_time: Option<i64>,
}

impl Default for ReadRecord {
    fn default() -> Self {
        Self {
            book_id: String::new(),
            dur_chapter_index: 0,
            dur_chapter_pos: 0,
            last_chapter_index: 0,
            last_chapter_pos: 0,
            total_read_time: 0,
            last_read_time: None,
        }
    }
}

