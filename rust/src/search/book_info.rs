use anyhow::{Context, Result};
use scraper::{Html, Selector};

use crate::models::book::Book;
use crate::models::book_source::BookSource;
use crate::models::rules::BookInfoRule;
use crate::search::parser::rule_engine;

pub fn parse_book_info(
    html: &str,
    rule: &BookInfoRule,
    book_url: &str,
    book_source: &BookSource,
    base: Option<&Book>,
) -> Result<(Book, Vec<String>)> {
    let document = Html::parse_document(html);

    let title = extract_title(&document);
    let og_title = extract_meta_content(&document, r#"meta[property="og:title"]"#);

    let name = rule_engine::extract_text_from_document(&document, rule.name.as_ref()).context(format!(
        "无法提取书籍名称: url={}, rule.name={:?}, title={:?}, og:title={:?}",
        book_url, rule.name, title, og_title
    ))?;

    let author = rule_engine::extract_text_from_document(&document, rule.author.as_ref()).unwrap_or_default();
    let intro = rule_engine::extract_text_from_document(&document, rule.intro.as_ref()).unwrap_or_default();
    let kind = rule_engine::extract_text_from_document(&document, rule.kind.as_ref()).unwrap_or_default();
    let latest_chapter_title = rule_engine::extract_text_from_document(&document, rule.last_chapter.as_ref()).unwrap_or_default();

    let update_time = rule_engine::extract_text_from_document(
        &document,
        rule.update_time.as_ref(),
    )
    .unwrap_or_default();

    let mut cover_url = rule_engine::extract_text_from_document(&document, rule.cover_url.as_ref())
        .unwrap_or_default();
    if cover_url.trim().is_empty() {
        cover_url = rule_engine::extract_text_from_document(&document, Some(&"img@data-original".to_string()))
            .unwrap_or_default();
    }
    if cover_url.trim().is_empty() {
        cover_url = rule_engine::extract_text_from_document(&document, Some(&"img@data-src".to_string()))
            .unwrap_or_default();
    }
    if cover_url.trim().is_empty() {
        cover_url = rule_engine::extract_text_from_document(&document, Some(&"img@src".to_string()))
            .unwrap_or_default();
    }
    if cover_url.trim().is_empty() {
        cover_url = extract_meta_content(&document, r#"meta[property="og:image"]"#).unwrap_or_default();
    }
    let cover_url = resolve_url_with_source(book_source, book_url, &cover_url).unwrap_or(cover_url);
    let cover_url = if cover_url.trim().is_empty() { None } else { Some(cover_url) };

    let toc_rules = rule.toc_url.as_ref();
    let toc_urls = toc_rules
        .map(|rule| rule_engine::extract_list_single_rule(&document, rule))
        .unwrap_or_default();
    let mut toc_urls = if toc_urls.is_empty() {
        vec![book_url.to_string()]
    } else {
        toc_urls
    };
    toc_urls = toc_urls
        .into_iter()
        .filter_map(|url| resolve_url_with_source(book_source, book_url, &url).or_else(|| Some(url)))
        .collect::<Vec<_>>();

    let word_count_str = rule_engine::extract_text_from_document(
        &document,
        rule.word_count.as_ref(),
    )
    .unwrap_or_default();

    let word_count: u32 = word_count_str
        .replace("万", "")
        .replace("字", "")
        .parse::<u32>()
        .unwrap_or(0);

    let mut book = Book {
        book_id: book_url.to_string(),
        name,
        author,
        kind: Some(kind),
        cover_url,
        intro: Some(intro),
        origin: Some(if book_source.book_source_url.is_empty() {
            book_source.book_source_name.clone()
        } else {
            book_source.book_source_url.clone()
        }),
        book_type: "网络小说".to_string(),
        word_count,
        latest_chapter_title: if latest_chapter_title.is_empty() {
            None
        } else {
            Some(latest_chapter_title)
        },
        toc_url: toc_urls.first().cloned(),
        book_group: None,
        add_time: update_time.parse::<i64>().unwrap_or(0),
        last_read_time: None,
        isbn: None,
        publisher: None,
        publish_date: None,
        status: "连载中".to_string(),
    };

    if let Some(base) = base {
        book = merge_base_book(book, base);
    }

    Ok((book, toc_urls))
}

fn merge_base_book(mut parsed: Book, base: &Book) -> Book {
    if parsed.name.is_empty() && !base.name.is_empty() {
        parsed.name = base.name.clone();
    }
    if parsed.author.is_empty() && !base.author.is_empty() {
        parsed.author = base.author.clone();
    }
    if parsed.kind.as_deref().unwrap_or("").is_empty() {
        parsed.kind = base.kind.clone();
    }
    if parsed.cover_url.as_deref().unwrap_or("").is_empty() {
        parsed.cover_url = base.cover_url.clone();
    }
    if parsed.intro.as_deref().unwrap_or("").is_empty() {
        parsed.intro = base.intro.clone();
    }
    if parsed.latest_chapter_title.as_deref().unwrap_or("").is_empty() {
        parsed.latest_chapter_title = base.latest_chapter_title.clone();
    }
    if parsed.toc_url.as_deref().unwrap_or("").is_empty() {
        parsed.toc_url = base.toc_url.clone();
    }
    if parsed.origin.as_deref().unwrap_or("").is_empty() {
        parsed.origin = base.origin.clone();
    }
    if parsed.book_type.is_empty() {
        parsed.book_type = base.book_type.clone();
    }
    if parsed.word_count == 0 {
        parsed.word_count = base.word_count;
    }
    if parsed.add_time == 0 {
        parsed.add_time = base.add_time;
    }
    parsed
}

fn extract_title(document: &Html) -> Option<String> {
    let selector = Selector::parse("title").ok()?;
    let element = document.select(&selector).next()?;
    let text = element.text().collect::<String>().trim().to_string();
    if text.is_empty() { None } else { Some(text) }
}

fn extract_meta_content(document: &Html, selector_str: &str) -> Option<String> {
    let selector = Selector::parse(selector_str).ok()?;
    let element = document.select(&selector).next()?;
    let content = element.value().attr("content")?.trim().to_string();
    if content.is_empty() { None } else { Some(content) }
}

pub(crate) fn resolve_url(base: &str, link: &str) -> Option<String> {
    let link = link.trim();
    if link.is_empty() {
        return None;
    }
    match reqwest::Url::parse(base) {
        Ok(base_url) => base_url.join(link).ok().map(|u| u.to_string()),
        Err(_) => None,
    }
}

pub(crate) fn resolve_url_with_source(book_source: &BookSource, book_url: &str, link: &str) -> Option<String> {
    let link = link.trim();
    if link.is_empty() {
        return None;
    }
    if link.starts_with("http://") || link.starts_with("https://") {
        return Some(link.to_string());
    }

    if let Some(resolved) = resolve_url(book_url, link) {
        return Some(resolved);
    }

    if !book_source.book_source_url.trim().is_empty() {
        let base = book_source.book_source_url.trim_end_matches('/');
        let path = if link.starts_with('/') {
            link.to_string()
        } else {
            format!("/{}", link)
        };
        return Some(format!("{}{}", base, path));
    }

    None
}