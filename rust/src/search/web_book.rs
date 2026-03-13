use anyhow::{Context, Result};
use chrono;
use tokio::task::JoinSet;
use tokio::time::{sleep, Duration};

use crate::search::book_info;
use crate::search::chapter;
use crate::search::search_result::{self, SearchResult};
use crate::models::book_source::BookSource;
use crate::network::http_client::HttpClient;
use crate::models::book::Book;
use crate::models::chapter::Chapter;
use crate::models::read_record::ReadRecord;
use crate::models::chapter_content::ChapterContent;
use crate::api::data_base::get_db;
/// 网络书籍搜索和获取功能
pub struct WebBookParser;

impl WebBookParser {
    // ===== 搜索 =====
    /// 搜索书籍
    pub async fn search(keyword: &str) -> Result<Vec<Book>> {
        // 从数据库加载启用的书源
        let book_sources = get_db().lock().unwrap().book_sources.list_enabled_sources()?;
        if book_sources.is_empty() {
            return Err(anyhow::anyhow!("没有启用的书源"));
        }

        let mut join_set = JoinSet::new();
        let keyword = keyword.to_string();

        for book_source in book_sources {
            let keyword = keyword.clone();
            join_set.spawn(async move {
                let mut books = Vec::new();
                if let Ok(results) = Self::search_with_source(&book_source, &keyword).await {
                    let mut detail_set = JoinSet::new();
                    for result in results {
                        let book_source = book_source.clone();
                        detail_set.spawn(async move {
                            match Self::get_book_info_from_search(
                                &book_source,
                                &result.book_url,
                                &result.book,
                            )
                            .await
                            {
                                Ok((book, _toc_urls)) => book,
                                Err(_) => result.book,
                            }
                        });
                    }

                    while let Some(detail) = detail_set.join_next().await {
                        if let Ok(book) = detail {
                            books.push(book);
                        }
                    }
                }
                books
            });
        }

        let mut all_books = Vec::new();
        while let Some(result) = join_set.join_next().await {
            if let Ok(mut books) = result {
                all_books.append(&mut books);
            }
        }

        Ok(all_books)
    }
    
    /// 使用指定书源搜索书籍
    async fn search_with_source(book_source: &BookSource, keyword: &str) -> Result<Vec<SearchResult>> {
        let http_client = HttpClient::from_book_source(book_source, None)?;
        
        let search_rule = book_source.search_rule.as_ref()
            .context("书源未配置搜索规则")?;
        
        // 使用 build_search_url 方法构建搜索 URL
        let search_url = Self::build_search_url(keyword, book_source)
            .context("书源未配置搜索 URL")?;
        
        // 发送搜索请求
        let response = http_client.get(&search_url).await
            .context(format!("搜索请求失败: {}", search_url))?;

        let status = response.status();
        let final_url = response.url().to_string();
        println!("[search] status={:?} url={:?}", status, final_url);

        let mut html = response.text().await
            .context("读取搜索响应失败")?;

        println!("[search] body_length={}", html.len());
        
        // 执行网页替换操作
        html = Self::replace_html(&html);
        
        // 解析搜索结果
        let results = search_result::parse_search_result(&html, search_rule, book_source)?;
        println!(
            "[search] parsed source={:?} results={}",
            book_source.book_source_name,
            results.len()
        );
        if let Some(first) = results.first() {
            println!(
                "[search] first: name={:?} url={:?} cover={:?}",
                first.book.name,
                first.book_url,
                first.book.cover_url
            );
        }

        Ok(results)
    }

    // ===== 书籍信息 =====
    /// 获取书籍信息
    pub async fn get_book_info(book_source: &BookSource, book_url: &str) -> Result<(Book, Vec<String>)> {
        println!(
            "[book_info] 请求书籍信息: url={:?}, source_name={:?}, source_url={:?}",
            book_url, book_source.book_source_name, book_source.book_source_url
        );
        let http_client = HttpClient::from_book_source(book_source, None)?;
        let book_info_rule = book_source
            .book_info_rule
            .as_ref()
            .context("书源未配置书籍信息规则")?;

        // 发送请求获取书籍信息页面
        let response = http_client.get(book_url).await
            .context(format!("获取书籍信息失败: {}", book_url))?;

        let html = response.text().await
            .context("读取书籍信息响应失败")?;

        println!(
            "[book_info] 响应HTML长度: {}, 前500字符: {}",
            html.len(),
            html.chars().take(500).collect::<String>()
        );

        // 解析书籍信息
        book_info::parse_book_info(&html, book_info_rule, book_url, book_source, None)
    }

    pub async fn get_book_info_from_search(
        book_source: &BookSource,
        book_url: &str,
        base_book: &Book,
    ) -> Result<(Book, Vec<String>)> {
        println!(
            "[book_info] 搜索结果详情: url={:?}, source_name={:?}",
            book_url, book_source.book_source_name
        );
        let http_client = HttpClient::from_book_source(book_source, None)?;
        let book_info_rule = book_source
            .book_info_rule
            .as_ref()
            .context("书源未配置书籍信息规则")?;

        let response = http_client.get(book_url).await
            .context(format!("获取书籍信息失败: {}", book_url))?;

        let html = response.text().await
            .context("读取书籍信息响应失败")?;

        book_info::parse_book_info(&html, book_info_rule, book_url, book_source, Some(base_book))
    }

    // ===== 目录 =====
    /// 获取书籍目录
    pub async fn get_book_toc(book_source: &BookSource, toc_url: &str) -> Result<Vec<Chapter>> {
        let http_client = HttpClient::from_book_source(book_source, None)?;
        let toc_rule = book_source.toc_rule.as_ref()
            .context("书源未配置目录规则")?;

        println!(
            "[toc] 请求目录: url={:?}, source_name={:?}, source_url={:?}",
            toc_url, book_source.book_source_name, book_source.book_source_url
        );
        println!("[toc] TocRule.chapter_list={:?}", toc_rule.chapter_list);
        println!("[toc] TocRule.chapter_name={:?}", toc_rule.chapter_name);
        println!("[toc] TocRule.chapter_url={:?}", toc_rule.chapter_url);
        println!("[toc] TocRule.next_toc_url={:?}", toc_rule.next_toc_url);

        let mut chapters = Vec::new();
        let mut chapter_index: u32 = 0;
        let mut next_url = Some(toc_url.to_string());
        let mut visited = std::collections::HashSet::new();

        while let Some(current_url) = next_url.take() {
            if !visited.insert(current_url.clone()) {
                break;
            }

            // 发送请求获取目录页面
            let response = http_client.get(&current_url).await
                .context(format!("获取目录失败: {}", current_url))?;
            
            let html = response.text().await
                .context("读取目录响应失败")?;

            println!(
                "[toc] 响应HTML长度: {}, 前300字符: {}",
                html.len(),
                html.chars().take(300).collect::<String>()
            );

            let parse_result = chapter::parse_book_toc(&html, toc_rule, chapter_index, &current_url)?;
            chapter_index = parse_result.next_index;
            chapters.extend(parse_result.chapters);

            next_url = parse_result
                .next_toc_url
                .as_deref()
                .and_then(|u| Self::resolve_url(&current_url, u));

            if next_url.is_some() {
                sleep(Duration::from_millis(800)).await;
            }
        }

        Ok(chapters)
    }


    // ===== 正文 =====
    /// 获取章节内容
    pub async fn get_chapter_content(
        book_id: &str,
        book_source: &BookSource,
        chapter_url: &str,
    ) -> Result<ChapterContent> {
        let http_client = HttpClient::from_book_source(book_source, None)?;
        let content_rule = book_source
            .content_rule
            .as_ref()
            .context("书源未配置内容规则")?;

        println!("[get_chapter_content] 请求章节内容: {}", chapter_url);

        let response = http_client
            .get(chapter_url)
            .await
            .context(format!("获取章节内容失败: {}", chapter_url))?;

        let html = response.text().await.context("读取章节内容响应失败")?;

        let content = chapter::parse_chapter_content(&html, content_rule)?;

        println!("[get_chapter_content] 解析完成，内容长度: {}", content.len());

        Ok(ChapterContent {
            book_id: book_id.to_string(),
            content,
        })
    }

    /// 并发获取多个章节内容
    pub async fn get_chapter_contents(
        book_id: &str,
        book_source: &BookSource,
        chapter_urls: Vec<String>,
    ) -> Result<Vec<ChapterContent>> {
        let book_id = book_id.to_string();
        let mut join_set = JoinSet::new();

        for (index, chapter_url) in chapter_urls.into_iter().enumerate() {
            let book_source = book_source.clone();
            let book_id = book_id.clone();
            join_set.spawn(async move {
                let result = Self::get_chapter_content(&book_id, &book_source, &chapter_url).await;
                (index, result)
            });
        }

        let mut contents: Vec<Option<ChapterContent>> = vec![None; join_set.len()];
        let mut first_error: Option<anyhow::Error> = None;

        while let Some(joined) = join_set.join_next().await {
            let (index, result) = joined.context("章节并发任务失败")?;
            match result {
                Ok(content) => {
                    if index < contents.len() {
                        contents[index] = Some(content);
                    }
                }
                Err(err) => {
                    if first_error.is_none() {
                        first_error = Some(err);
                    }
                }
            }
        }

        if let Some(err) = first_error {
            return Err(err);
        }

        Ok(contents.into_iter().flatten().collect())
    }

    // ===== 阅读记录 =====
    /// 创建或更新阅读记录
    pub fn create_read_record(book_id: &str, chapter_index: u64, chapter_pos: u64) -> ReadRecord {
        ReadRecord {
            book_id: book_id.to_string(),
            dur_chapter_index: chapter_index,
            dur_chapter_pos: chapter_pos,
            last_chapter_index: chapter_index,
            last_chapter_pos: chapter_pos,
            total_read_time: 0,
            last_read_time: Some(chrono::Local::now().timestamp()),
        }
    }

    fn resolve_url(base: &str, link: &str) -> Option<String> {
        let link = link.trim();
        if link.is_empty() {
            return None;
        }
        match reqwest::Url::parse(base) {
            Ok(base_url) => base_url.join(link).ok().map(|u| u.to_string()),
            Err(_) => None,
        }
    }




    // ===== 内部工具 =====
    /// 执行网页替换操作
    fn replace_html(html: &str) -> String {
        let mut result = html.to_string();
        
        // 这里可以添加各种替换规则
        // 例如，替换一些常见的干扰内容
        result = result.replace("</script>", "</script>");
        result = result.replace("</style>", "</style>");
        
        // 可以根据需要添加更多替换规则
        
        result
    }


//===================辅助方法===============================================

    
    /// 构建搜索 URL
    fn build_search_url(keyword: &str, book_source: &BookSource) -> Option<String> {
        if let Some(search_url) = &book_source.search_url {
            // 清理搜索 URL 模板，移除所有不需要的字符
            let clean_search_url: String = search_url
                .chars()
                .filter(|c| !(*c == '`' || *c == '"' || *c == '\''))
                .collect::<String>()
                .trim()
                .to_string();

            let encoded_keyword = url::form_urlencoded::byte_serialize(keyword.as_bytes())
                .collect::<String>();

            // 打印调试信息
            println!("清理后的搜索 URL 模板: {:?}", clean_search_url);
            println!("关键词: {:?}", keyword);

            // 先替换长占位符，再替换短占位符，避免匹配截断
            let result = clean_search_url
                .replace("{{keyword}}", &encoded_keyword)
                .replace("{keyword}", &encoded_keyword)
                .replace("{{key}}", &encoded_keyword)
                .replace("{key}", &encoded_keyword)
                .replace("{{page}}", "1")
                .replace("{page}", "1");

            // 打印替换结果
            println!("替换后的搜索 URL: {:?}", result);

            Some(result)
        } else {
            None
        }
    }
}