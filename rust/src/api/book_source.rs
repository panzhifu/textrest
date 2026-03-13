use anyhow::Result;
use flutter_rust_bridge::frb;

use crate::api::data_base::get_db;
use crate::models::BookSource;
use crate::utils::BookSourceParser;

#[frb(opaque)]
pub struct BookSourceApi;

impl BookSourceApi {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }
    
    pub fn import_from_text(&self, text: String) -> Result<Vec<BookSource>> {
        let sources = BookSourceParser::from_text(&text)?;
        let db = get_db();
        for source in &sources {
            db.lock().unwrap().book_sources.upsert_book_source(source)?;
        }
        Ok(sources)
    }
    
    pub async fn import_from_url(&self, url: String) -> Result<Vec<BookSource>> {
        let sources = BookSourceParser::from_url(&url).await?;
        let db = get_db();
        for source in &sources {
            db.lock().unwrap().book_sources.upsert_book_source(source)?;
        }
        Ok(sources)
    }
    
    pub fn import_from_file(&self, file_path: String) -> Result<Vec<BookSource>> {
        let sources = BookSourceParser::from_file(file_path)?;
        let db = get_db();
        for source in &sources {
            db.lock().unwrap().book_sources.upsert_book_source(source)?;
        }
        Ok(sources)
    }
    
    pub fn get_source(&self, url: &str) -> Result<Option<BookSource>> {
        let db = get_db();
        let result = db.lock().unwrap().book_sources.get_book_source(url)
            .map_err(|e| anyhow::anyhow!("获取书源失败: {}", e))?;
        Ok(result)
    }
    
    pub fn list_enabled_sources(&self) -> Result<Vec<BookSource>> {
        let db = get_db();
        let result = db.lock().unwrap().book_sources.list_enabled_sources()
            .map_err(|e| anyhow::anyhow!("获取书源列表失败: {}", e))?;
        Ok(result)
    }
    
    pub fn list_all_sources(&self) -> Result<Vec<BookSource>> {
        let db = get_db();
        let result = db.lock().unwrap().book_sources.list_all_sources()
            .map_err(|e| anyhow::anyhow!("获取所有书源列表失败: {}", e))?;
        Ok(result)
    }
    
    pub fn delete_source(&self, url: &str) -> Result<()> {
        let db = get_db();
        db.lock().unwrap().book_sources.delete_book_source(url)?;
        Ok(())
    }
    
    pub fn update_source(&self, source: &BookSource) -> Result<()> {
        let db = get_db();
        db.lock().unwrap().book_sources.upsert_book_source(source)?;
        Ok(())
    }
}
