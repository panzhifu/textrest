// 基础解析器
use crate::models::{Book, Chapter, ReadRecord};
use anyhow::{Context, Result};
use chrono::Local;
use std::fs::File;
use std::hash::Hasher;
use std::io::Read;
use std::path::Path;
use twox_hash::XxHash64;

use crate::parser::EpubParser;

/// 解析器统一接口
pub trait BookParser {
    /// 支持的文件类型（小写扩展名）
    fn file_type(&self) -> &'static str;

    /// 解析书籍、章节列表与阅读进度
    fn parse(&self, file_path: String) -> Result<(Book, Vec<Chapter>, ReadRecord)>;

    /// 获取指定章节内容
    fn get_content(
        &self,
        file_path: String,
        chapter_index: usize,
        chapter_url: Option<&str>,
    ) -> Result<String>;
}

/// 解析器注册表，负责根据格式分发调用
pub struct ParserRegistry {
    parsers: Vec<Box<dyn BookParser + Send + Sync>>,
}

impl ParserRegistry {
    /// 创建默认注册表（内置 epub）
    pub fn default_registry() -> Self {
        Self {
            parsers: vec![Box::new(EpubParser)],
        }
    }

    /// 通过扩展名解析书籍
    pub fn parse_book(&self, file_path: String) -> Result<(Book, Vec<Chapter>, ReadRecord)> {
        let path = Path::new(&file_path);
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .context("无法获取文件扩展名")?
            .to_lowercase();

        let parser = self
            .parsers
            .iter()
            .find(|parser| parser.file_type() == extension)
            .context("不支持的文件格式")?;

        parser.parse(file_path)
    }

    /// 按格式统一提取章节内容
    pub fn get_chapter_content(
        &self,
        book_type: &str,
        file_path: &str,
        chapter_index: usize,
        chapter_url: Option<&str>,
    ) -> Result<String> {
        let parser = self
            .parsers
            .iter()
            .find(|parser| parser.file_type() == book_type)
            .context("不支持的文件格式")?;

        parser.get_content(file_path.to_string(), chapter_index, chapter_url)
    }
}

/// 通用解析工具集
pub struct BaseParser;

impl BaseParser {
    /// 创建书籍基础信息（导入时调用）
    pub fn create_book(book_id: &str, path: &Path, book_type: &str) -> Book {
        let mut book = Book::default();
        book.book_id = book_id.to_string();
        book.book_type = book_type.to_string();
        book.toc_url = Some(path.to_string_lossy().to_string());
        book.origin = Some("本地文件".to_string());
        book.add_time = Local::now().timestamp();
        book.status = "已完成".to_string();
        book
    }

    /// 创建默认阅读进度
    pub fn create_read_record(book_id: &str, chapter_count: usize) -> ReadRecord {
        let mut record = ReadRecord::default();
        record.book_id = book_id.to_string();
        record.last_chapter_index = (chapter_count as u64).saturating_sub(1);
        record
    }

    /// 基于文件内容生成稳定的书籍 ID
    pub fn generate_id(path: &Path, prefix: &str) -> Result<String> {
        let mut file = File::open(path).context("无法打开文件")?;
        let mut hasher = XxHash64::default();
        let mut buffer = [0; 8192];

        loop {
            let n = file.read(&mut buffer).context("读取文件失败")?;
            if n == 0 {
                break;
            }
            hasher.write(&buffer[..n]);
        }

        Ok(format!("{}:{:x}", prefix, hasher.finish()))
    }

    /// 估算字数（中英文混排）
    pub fn calculate_word_count(text: &str) -> u32 {
        let mut word_count: u32 = 0;
        let mut in_word = false;

        for c in text.chars() {
            if c.is_whitespace() || c.is_ascii_punctuation() {
                in_word = false;
                continue;
            }

            if Self::is_chinese(c) {
                word_count = word_count.saturating_add(1);
                in_word = false;
            } else if !in_word {
                word_count = word_count.saturating_add(1);
                in_word = true;
            }
        }

        word_count
    }

    /// 按格式分发解析
    pub fn parse_book(file_path: String) -> Result<(Book, Vec<Chapter>, ReadRecord)> {
        ParserRegistry::default_registry().parse_book(file_path)
    }

    /// 按格式统一提取章节内容
    pub fn get_chapter_content(
        book_type: &str,
        file_path: &str,
        chapter_index: usize,
        chapter_url: Option<&str>,
    ) -> Result<String> {
        ParserRegistry::default_registry().get_chapter_content(
            book_type,
            file_path,
            chapter_index,
            chapter_url,
        )
    }
}


impl BaseParser {
    fn is_chinese(c: char) -> bool {
        matches!(c as u32,
            0x4E00..=0x9FFF |
            0x3400..=0x4DBF |
            0x20000..=0x2A6DF |
            0x2A700..=0x2B73F |
            0x2B740..=0x2B81F |
            0x2B820..=0x2CEAF |
            0xF900..=0xFAFF |
            0x2F800..=0x2FA1F
        )
    }
}

