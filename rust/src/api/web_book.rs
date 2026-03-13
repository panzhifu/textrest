use anyhow::Result;
use flutter_rust_bridge::frb;

use crate::models::{Book, Chapter, ReadRecord, BookSource, ChapterContent};
use crate::api::data_base::get_db;
use crate::search::web_book::WebBookParser;

#[frb(opaque)]
pub struct WebBookApi;

impl WebBookApi {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }
    
    pub async fn search(&self, keyword: String) -> Result<Vec<Book>> {
        WebBookParser::search(&keyword).await
    }

    pub async fn get_book_info(&self, book_source: BookSource, book_url: String) -> Result<Book> {
        let (book, _) = WebBookParser::get_book_info(&book_source, &book_url).await?;
        Ok(book)
    }

    pub async fn get_book_info_from_search(
        &self,
        book_source: BookSource,
        book_url: String,
        base_book: Book,
    ) -> Result<Book> {
        let (book, _) =
            WebBookParser::get_book_info_from_search(&book_source, &book_url, &base_book).await?;
        Ok(book)
    }

    pub async fn get_book_toc_urls(&self, book_source: BookSource, book_url: String) -> Result<Vec<String>> {
        let (_, toc_urls) = WebBookParser::get_book_info(&book_source, &book_url).await?;
        Ok(toc_urls)
    }

    pub async fn get_book_toc_urls_from_search(
        &self,
        book_source: BookSource,
        book_url: String,
        base_book: Book,
    ) -> Result<Vec<String>> {
        let (_, toc_urls) =
            WebBookParser::get_book_info_from_search(&book_source, &book_url, &base_book).await?;
        Ok(toc_urls)
    }
    
    pub async fn get_book_toc(&self, book_source: BookSource, toc_url: String) -> Result<Vec<Chapter>> {
        WebBookParser::get_book_toc(&book_source, &toc_url).await
    }
    
    pub async fn get_chapter_content(
        &self,
        book_id: String,
        book_source: BookSource,
        chapter_url: String,
    ) -> Result<ChapterContent> {
        WebBookParser::get_chapter_content(&book_id, &book_source, &chapter_url).await
    }

    /// 添加网络书籍到书架（写入 books/chapters/read_records）
    ///
    /// - `book_url`: 书籍详情页 URL（也将作为网络书籍的 `book_id`）
    pub async fn add_network_book(&self, book_source: BookSource, book_url: String) -> Result<Book> {
        // 1) 拉取书籍信息
        let (mut book, toc_urls) = WebBookParser::get_book_info(&book_source, &book_url).await?;

        // 2) 规范化关键字段（网络书籍 book_id 使用 url）
        if book.book_id.trim().is_empty() {
            book.book_id = book_url.clone();
        }
        if book.origin.as_ref().map(|s| s.trim().is_empty()).unwrap_or(true) {
            // 保留更稳定的标识：优先书源 url，其次书源名
            if !book_source.book_source_url.trim().is_empty() {
                book.origin = Some(book_source.book_source_url.clone());
            } else {
                book.origin = Some(book_source.book_source_name.clone());
            }
        }
        if book.toc_url.as_ref().map(|s| s.trim().is_empty()).unwrap_or(true) {
            book.toc_url = toc_urls.first().cloned().or_else(|| Some(book_url.clone()));
        }

        // 3) 拉取目录并补齐 book_id（支持多 toc_url）
        let toc_urls = if toc_urls.is_empty() {
            vec![book_url.clone()]
        } else {
            toc_urls
        };
        let mut chapters = Vec::new();
        for toc_url in toc_urls {
            let mut part = WebBookParser::get_book_toc(&book_source, &toc_url).await?;
            chapters.append(&mut part);
        }
        for ch in &mut chapters {
            ch.book_id = book.book_id.clone();
        }

        // 4) 创建默认阅读记录（从第 0 章开始）
        let read_record = WebBookParser::create_read_record(&book.book_id, 0, 0);

        // 5) 写入数据库
        let db = get_db();
        let db = db.lock().unwrap();

        db.books.upsert_book(&book)?;
        for chapter in &chapters {
            db.chapters.upsert_chapter(chapter)?;
        }
        db.read_records.upsert_record(&read_record)?;

        Ok(book)
    }
    
    pub fn create_read_record(&self, book_id: String, chapter_index: i64, chapter_pos: i64) -> ReadRecord {
        let chapter_index = u64::try_from(chapter_index).unwrap_or(0);
        let chapter_pos = u64::try_from(chapter_pos).unwrap_or(0);
        WebBookParser::create_read_record(&book_id, chapter_index, chapter_pos)
    }

    pub fn load_book_sources(&self) -> Result<Vec<BookSource>> {
        let db = get_db();
        let db = db.lock().unwrap();
        let result = db.book_sources.list_enabled_sources()
            .map_err(|e| anyhow::anyhow!("加载书源失败: {}", e))?;
        Ok(result)
    }
}
