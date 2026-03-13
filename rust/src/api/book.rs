use anyhow::{Context, Result};
use flutter_rust_bridge::frb;

use crate::api::data_base::get_db;
use crate::models::{Book, Chapter, ReadRecord};
use crate::parser::base_parser::BaseParser;

#[frb(opaque)]
pub struct BookApi;

impl BookApi {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }
    
    pub fn import_book(&self, file_path: String) -> Result<Book> {
        let (book, chapters, read_record) = BaseParser::parse_book(file_path)?;
        
        let db = get_db();
        let db = db.lock().unwrap();
        db.books.upsert_book(&book)?;
        
        for chapter in &chapters {
            db.chapters.upsert_chapter(chapter)?;
        }
        
        db.read_records.upsert_record(&read_record)?;
        
        Ok(book)
    }
    
    pub fn delete_book(&self, book_id: &str) -> Result<()> {
        let db = get_db();
        let db = db.lock().unwrap();
        db.books.delete_book(book_id)?;
        db.chapters.delete_chapters_by_book(book_id)?;
        db.read_records.delete_record(book_id)?;
        Ok(())
    }
    
    pub fn load_book_toc(&self, book_id: &str) -> Result<Vec<Chapter>> {
        let db = get_db();
        let result = db.lock().unwrap().chapters.list_chapters_by_book(book_id)
            .map_err(|e| anyhow::anyhow!("加载目录失败: {}", e))?;
        Ok(result)
    }
    
    pub fn load_read_progress(&self, book_id: &str) -> Result<Option<ReadRecord>> {
        let db = get_db();
        let result = db.lock().unwrap().read_records.get_record(book_id)
            .map_err(|e| anyhow::anyhow!("加载阅读进度失败: {}", e))?;
        Ok(result)
    }
    
    pub fn update_read_progress(&self, record: &ReadRecord) -> Result<()> {
        let db = get_db();
        db.lock().unwrap().read_records.upsert_record(record)
            .map_err(|e| anyhow::anyhow!("更新阅读进度失败: {}", e))?;
        Ok(())
    }
    
    pub fn get_book(&self, book_id: &str) -> Result<Option<Book>> {
        let db = get_db();
        let result = db.lock().unwrap().books.get_book(book_id)
            .map_err(|e| anyhow::anyhow!("获取书籍失败: {}", e))?;
        Ok(result)
    }
    
    pub fn list_books(&self) -> Result<Vec<Book>> {
        let db = get_db();
        let result = db.lock().unwrap().books.list_books()
            .map_err(|e| anyhow::anyhow!("获取书籍列表失败: {}", e))?;
        Ok(result)
    }

    /// 获取指定章节内容
    pub fn load_chapter_content(&self, book_id: &str, chapter_index: usize) -> Result<String> {
        let db = get_db();
        let db = db.lock().unwrap();

        let book = db.books.get_book(book_id)
            .map_err(|e| anyhow::anyhow!("获取书籍失败: {}", e))?
            .context("书籍不存在")?;

        let chapters = db.chapters.list_chapters_by_book(book_id)
            .map_err(|e| anyhow::anyhow!("加载目录失败: {}", e))?;

        let chapter = chapters.get(chapter_index)
            .context("章节索引超出范围")?;

        let file_path = book.toc_url.as_deref().context("书籍文件路径缺失")?;

        BaseParser::get_chapter_content(
            &book.book_type,
            file_path,
            chapter_index,
            Some(chapter.url.as_str()),
        )
    }
}
