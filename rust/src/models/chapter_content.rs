use serde::{Deserialize, Serialize};
use flutter_rust_bridge::frb;

#[frb]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterContent {
    pub book_id: String,
    pub content: String,
}
