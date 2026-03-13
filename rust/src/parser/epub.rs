// EPUB解析器
use crate::models::{Book, Chapter, ReadRecord};
use crate::parser::base_parser::BaseParser;
use anyhow::{Context, Result};
use epub::doc::EpubDoc;
use std::env;
use std::fs;
use std::path::Path;

pub struct EpubParser;

impl EpubParser {
    pub fn parse(file_path: String) -> Result<(Book, Vec<Chapter>, ReadRecord)> {
        let path = Path::new(&file_path);
        let mut doc = EpubDoc::new(path).context("解析EPUB文件失败")?;
        let book_id = BaseParser::generate_id(path, "epub")?;
        
        let mut book = BaseParser::create_book(&book_id, path, "epub");
        Self::parse_metadata(&mut book, &doc);
        Self::extract_cover(&mut book, &mut doc);
        Self::fill_defaults(&mut book, path, &doc);
        
        let chapters = Self::build_chapters(&mut doc, &book_id);
        book.latest_chapter_title = chapters.last().map(|c| c.title.clone());
        
        let read_record = BaseParser::create_read_record(&book_id, chapters.len());
        
        Ok((book, chapters, read_record))
    }
    
    pub fn get_content(file_path: String, content_ref: String) -> Result<String> {
        let mut doc = EpubDoc::new(Path::new(&file_path)).context("解析EPUB文件失败")?;
        let content_ref = Self::normalize_content_ref(&content_ref);

        if let Some((content, _)) = doc.get_resource(&content_ref) {
            return String::from_utf8(content)
                .context("章节内容不是有效的UTF-8");
        }

        // 部分 EPUB 的目录路径带 OEBPS/ 前缀，资源键可能不含前缀
        if let Some(stripped) = content_ref.strip_prefix("OEBPS/") {
            if let Some((content, _)) = doc.get_resource(stripped) {
                return String::from_utf8(content)
                    .context("章节内容不是有效的UTF-8");
            }
        }

        // 回退：尝试在资源表中模糊匹配
        let file_name = Path::new(&content_ref)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("");
        let stripped_ref = content_ref.replace("OEBPS/", "");

        let resource_key = doc
            .resources
            .keys()
            .map(|key| key.replace('\\', "/"))
            .find(|key| {
                key.ends_with(&content_ref)
                    || key.ends_with(&stripped_ref)
                    || (!file_name.is_empty() && key.ends_with(file_name))
            });

        if let Some(key) = resource_key {
            if let Some((content, _)) = doc.get_resource(&key) {
                return String::from_utf8(content)
                    .context("章节内容不是有效的UTF-8");
            }
        }

        let mut keys: Vec<String> = doc
            .resources
            .keys()
            .map(|k| k.replace('\\', "/"))
            .collect();
        keys.sort();
        let preview = keys
            .iter()
            .take(15)
            .cloned()
            .collect::<Vec<_>>()
            .join(" | ");

        Err(anyhow::anyhow!(
            "无法获取章节内容: content_ref={} resources_preview={}",
            content_ref,
            preview
        ))
    }

    fn parse_metadata(book: &mut Book, doc: &EpubDoc<std::io::BufReader<std::fs::File>>) {
        for item in &doc.metadata {
            match item.property.as_str() {
                "title" => book.name = item.value.clone(),
                "creator" => book.author = item.value.clone(),
                "publisher" => book.publisher = Some(item.value.clone()),
                "description" => book.intro = Some(item.value.clone()),
                "date" => {
                    book.publish_date = item.value.parse::<i64>().ok();
                }
                "identifier" => book.isbn = Some(
                    item.value.strip_prefix("urn:isbn:").unwrap_or(&item.value).to_string(),
                ),
                "subject" => book.kind = Some(item.value.clone()),
                _ => {}
            }
        }
    }

    fn extract_cover(book: &mut Book, doc: &mut EpubDoc<std::io::BufReader<std::fs::File>>) {
        let Some((cover_data, cover_path)) = doc.get_cover() else { return };

        let ext = Path::new(&cover_path)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("");

        let cover_name = format!(
            "cover_{}.{}",
            book.name.replace(' ', "_").replace(|c: char| !c.is_alphanumeric(), ""),
            ext
        );

        let cover_path = env::temp_dir().join(cover_name);
        if fs::write(&cover_path, cover_data).is_ok() {
            book.cover_url = Some(cover_path.to_string_lossy().to_string());
        }
    }

    fn fill_defaults(book: &mut Book, path: &Path, doc: &EpubDoc<std::io::BufReader<std::fs::File>>) {
        if book.name.is_empty() {
            book.name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("未知书名")
                .to_string();
        }
        if book.author.is_empty() {
            book.author = "未知作者".to_string();
        }
        book.word_count = (doc.spine.len() as u32).saturating_mul(3000);
    }

    fn build_chapters(
        doc: &mut EpubDoc<std::io::BufReader<std::fs::File>>,
        book_id: &str,
    ) -> Vec<Chapter> {
        // EPUB2：优先使用 toc（通常来自 toc.ncx）
        if !doc.toc.is_empty() {
            return doc
                .toc
                .iter()
                .enumerate()
                .map(|(index, node)| {
                    let mut c = Chapter::default();
                    c.id = (index + 1) as u32;
                    c.book_id = book_id.to_string();
                    c.title = node.label.clone();
                    // 使用 toc 对应的内容路径，保证能按目录条目精确取章节
                    c.url = Self::normalize_toc_path(&node.content);
                    c.chapter_index = index as u32;
                    c
                })
                .collect();
        }

        // EPUB3/无目录：回退到 spine，并从 HTML 标题提取章节名
        (0..doc.spine.len())
            .enumerate()
            .map(|(index, _)| {
                let idref = doc.spine[index].idref.clone();
                let content = doc
                    .get_resource(&idref)
                    .and_then(|(content, _)| String::from_utf8(content).ok());

                let title = content
                    .as_deref()
                    .and_then(Self::extract_title_from_html)
                    .unwrap_or_else(|| format!("章节 {}", index + 1));

                let mut c = Chapter::default();
                c.id = (index + 1) as u32;
                c.book_id = book_id.to_string();
                c.title = title;
                // spine 条目以 idref 为准，读取内容时用 idref 精确定位
                c.url = idref;
                c.chapter_index = index as u32;
                c
            })
            .collect()
    }

    fn normalize_toc_path(path: &std::path::PathBuf) -> String {
        Self::normalize_content_ref(&path.to_string_lossy())
    }

    fn normalize_content_ref(content_ref: &str) -> String {
        content_ref
            .replace('\\', "/")
            .split('#')
            .next()
            .unwrap_or("")
            .to_string()
    }

    fn extract_title_from_html(html: &str) -> Option<String> {
        let candidates = ["<h1", "<h2", "<title"];

        for tag in candidates {
            if let Some(text) = Self::extract_tag_text(html, tag) {
                let cleaned = Self::strip_html_tags(&text);
                let trimmed = cleaned.trim();
                if !trimmed.is_empty() {
                    return Some(trimmed.to_string());
                }
            }
        }

        None
    }

    fn extract_tag_text(html: &str, tag: &str) -> Option<String> {
        let lower = html.to_lowercase();
        let tag_pos = lower.find(tag)?;
        let open_end = lower[tag_pos..].find('>')? + tag_pos;
        let close_tag = match tag {
            "<title" => "</title>",
            "<h1" => "</h1>",
            "<h2" => "</h2>",
            _ => return None,
        };
        let close_pos = lower[open_end..].find(close_tag)? + open_end;
        Some(html[open_end + 1..close_pos].to_string())
    }

    fn strip_html_tags(input: &str) -> String {
        let mut output = String::new();
        let mut in_tag = false;
        for ch in input.chars() {
            match ch {
                '<' => in_tag = true,
                '>' => in_tag = false,
                _ if !in_tag => output.push(ch),
                _ => {}
            }
        }
        output
    }
}

impl crate::parser::base_parser::BookParser for EpubParser {
    fn file_type(&self) -> &'static str {
        "epub"
    }

    fn parse(&self, file_path: String) -> Result<(Book, Vec<Chapter>, ReadRecord)> {
        Self::parse(file_path)
    }

    fn get_content(
        &self,
        file_path: String,
        _chapter_index: usize,
        chapter_url: Option<&str>,
    ) -> Result<String> {
        let content_ref = chapter_url.context("EPUB章节路径缺失")?;
        Self::get_content(file_path, content_ref.to_string())
    }
}
