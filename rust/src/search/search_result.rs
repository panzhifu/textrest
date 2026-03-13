use anyhow::{Context, Result};
use regex::Regex;
use scraper::Html;

use crate::models::book::Book;
use crate::models::book_source::BookSource;
use crate::models::rules::SearchRule;
use crate::search::parser::rule_engine;
use crate::search::parser::selector;

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub book: Book,
    pub book_url: String,
}

pub fn parse_search_result(html: &str, rule: &SearchRule, book_source: &BookSource) -> Result<Vec<SearchResult>> {
    let document = Html::parse_document(html);

    let list_selector_str = rule.list.as_ref()
        .context("缺少搜索列表规则")?;

    let mut results = Vec::new();

    if let Some((regex, reversed)) = parse_all_in_one_rule(list_selector_str) {
        let mut matches = regex.captures_iter(html).collect::<Vec<_>>();
        if reversed {
            matches.reverse();
        }

        println!(
            "[search] list_selector(AllInOne)={:?} elements={}",
            list_selector_str,
            matches.len()
        );

        for (idx, caps) in matches.into_iter().enumerate() {
            let name = extract_text_from_caps(rule.name.as_ref(), &caps);
            let url = extract_text_from_caps(rule.url.as_ref(), &caps);
            let cover = extract_text_from_caps(rule.cover.as_ref(), &caps);
            let author = extract_text_from_caps(rule.author.as_ref(), &caps);
            let intro = extract_text_from_caps(rule.intro.as_ref(), &caps);
            let kind = extract_text_from_caps(rule.kind.as_ref(), &caps);
            let last_chapter = extract_text_from_caps(rule.last_chapter.as_ref(), &caps);
            let update_time = extract_text_from_caps(rule.update_time.as_ref(), &caps);
            let word_count_str = extract_text_from_caps(rule.word_count.as_ref(), &caps);

            if idx < 3 {
                println!(
                    "[search] item[{idx}] name={:?} url={:?} cover={:?} author={:?} kind={:?} word_count={:?}",
                    name,
                    url,
                    cover,
                    author,
                    kind,
                    word_count_str
                );
            }

            let url = normalize_url(book_source, url);
            let cover = normalize_url(book_source, cover);

            if let (Some(name), Some(url)) = (name, url) {
                let word_count: u32 = word_count_str
                    .unwrap_or_default()
                    .replace("万", "")
                    .replace("字", "")
                    .parse::<u32>()
                    .unwrap_or(0);

                let toc_url = match book_source
                    .book_info_rule
                    .as_ref()
                    .and_then(|rule| rule.toc_url.as_ref())
                {
                    Some(value) if !value.trim().is_empty() => None,
                    _ => Some(url.clone()),
                };

                let book = Book {
                    book_id: url.clone(),
                    name,
                    author: author.unwrap_or_default(),
                    kind: kind.map(|k| k.to_string()),
                    cover_url: cover.map(|c| c.to_string()),
                    intro: intro.map(|i| i.to_string()),
                    origin: Some(if book_source.book_source_url.is_empty() {
                        book_source.book_source_name.clone()
                    } else {
                        book_source.book_source_url.clone()
                    }),
                    book_type: "网络小说".to_string(),
                    word_count,
                    latest_chapter_title: last_chapter.filter(|s| !s.is_empty()),
                    toc_url,
                    book_group: None,
                    add_time: update_time
                        .unwrap_or_default()
                        .parse::<i64>()
                        .unwrap_or(0),
                    last_read_time: None,
                    isbn: None,
                    publisher: None,
                    publish_date: None,
                    status: "连载中".to_string(),
                };

                results.push(SearchResult {
                    book_url: url.clone(),
                    book,
                });
            }
        }

        return Ok(results);
    }

    let elements = selector::select_elements_from_element(document.root_element(), list_selector_str);

    println!(
        "[search] list_selector={:?} elements={}",
        list_selector_str,
        elements.len()
    );

    for (idx, element) in elements.into_iter().enumerate() {
        let name = extract_text(&element, rule.name.as_ref());
        let url = extract_text(&element, rule.url.as_ref());
        let cover = extract_text(&element, rule.cover.as_ref());
        let author = extract_text(&element, rule.author.as_ref());
        let intro = extract_text(&element, rule.intro.as_ref());
        let kind = extract_text(&element, rule.kind.as_ref());
        let last_chapter = extract_text(&element, rule.last_chapter.as_ref());
        let update_time = extract_text(&element, rule.update_time.as_ref());
        let word_count_str = extract_text(&element, rule.word_count.as_ref());

        if idx < 3 {
            let url_list = rule
                .url
                .as_deref()
                .map(|rule| rule_engine::extract_list_from_element(&element, rule))
                .unwrap_or_default();
            println!("[search] item[{idx}] url_list={:?}", url_list);
        }

        if idx < 3 {
            println!(
                "[search] item[{idx}] name={:?} url={:?} cover={:?} author={:?} kind={:?} word_count={:?}",
                name,
                url,
                cover,
                author,
                kind,
                word_count_str
            );
        }

        let url = normalize_url(book_source, url);
        let cover = normalize_url(book_source, cover);

        if let (Some(name), Some(url)) = (name, url) {
            let word_count: u32 = word_count_str
                .unwrap_or_default()
                .replace("万", "")
                .replace("字", "")
                .parse::<u32>()
                .unwrap_or(0);

            let toc_url = match book_source
                .book_info_rule
                .as_ref()
                .and_then(|rule| rule.toc_url.as_ref())
            {
                Some(value) if !value.trim().is_empty() => None,
                _ => Some(url.clone()),
            };

            let book = Book {
                book_id: url.clone(),
                name,
                author: author.unwrap_or_default(),
                kind: kind.map(|k| k.to_string()),
                cover_url: cover.map(|c| c.to_string()),
                intro: intro.map(|i| i.to_string()),
                origin: Some(if book_source.book_source_url.is_empty() {
                    book_source.book_source_name.clone()
                } else {
                    book_source.book_source_url.clone()
                }),
                book_type: "网络小说".to_string(),
                word_count,
                latest_chapter_title: last_chapter.filter(|s| !s.is_empty()),
                toc_url,
                book_group: None,
                add_time: update_time
                    .unwrap_or_default()
                    .parse::<i64>()
                    .unwrap_or(0),
                last_read_time: None,
                isbn: None,
                publisher: None,
                publish_date: None,
                status: "连载中".to_string(),
            };

            results.push(SearchResult {
                book_url: url.clone(),
                book,
            });
        }
    }

    Ok(results)
}

fn extract_text(element: &scraper::ElementRef<'_>, selector_rule: Option<&String>) -> Option<String> {
    selector_rule
        .as_deref()
        .and_then(|rule| rule_engine::extract_text_from_element(element, rule))
}

fn extract_text_from_caps(selector_rule: Option<&String>, caps: &regex::Captures<'_>) -> Option<String> {
    selector_rule
        .as_deref()
        .map(|rule| apply_capture_template(rule, caps))
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn normalize_url(book_source: &BookSource, url: Option<String>) -> Option<String> {
    url.map(|u| {
        if u.starts_with("http") {
            u
        } else {
            let base_url = if book_source.book_source_url.is_empty() {
                "http://www.haitaozhe.com"
            } else {
                &book_source.book_source_url
            };
            let base_url = base_url.trim_end_matches('/');
            let path = if u.starts_with('/') {
                u
            } else {
                format!("/{}", u)
            };
            format!("{}{}", base_url, path)
        }
    })
}

fn parse_all_in_one_rule(rule: &str) -> Option<(Regex, bool)> {
    let rule = rule.trim();
    let (pattern, reversed) = if let Some(rest) = rule.strip_prefix("-:") {
        (rest.trim(), true)
    } else if let Some(rest) = rule.strip_prefix(":") {
        (rest.trim(), false)
    } else {
        return None;
    };

    if pattern.is_empty() {
        return None;
    }

    Regex::new(pattern).ok().map(|re| (re, reversed))
}

fn apply_capture_template(template: &str, caps: &regex::Captures<'_>) -> String {
    let mut result = String::new();
    let mut chars = template.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '$' {
            let mut num = String::new();
            while let Some(next) = chars.peek() {
                if next.is_ascii_digit() {
                    num.push(*next);
                    chars.next();
                } else {
                    break;
                }
            }
            if let Ok(idx) = num.parse::<usize>() {
                if let Some(m) = caps.get(idx) {
                    result.push_str(m.as_str());
                    continue;
                }
            }
            result.push('$');
            result.push_str(&num);
        } else {
            result.push(ch);
        }
    }
    result
}
