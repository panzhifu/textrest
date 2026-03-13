use anyhow::{Context, Result};
use regex::Regex;
use scraper::{Html, Selector};

use crate::models::chapter::Chapter;
use crate::models::rules::{ContentRule, TocRule};
use crate::search::parser::rule_engine;

fn resolve_url(base: &str, link: &str) -> Option<String> {
    crate::search::book_info::resolve_url(base, link)
}

#[derive(Debug)]
pub struct TocParseResult {
    pub chapters: Vec<Chapter>,
    pub next_toc_url: Option<String>,
    pub next_index: u32,
}

pub fn parse_book_toc(html: &str, rule: &TocRule, start_index: u32, base_url: &str) -> Result<TocParseResult> {
    println!("[toc] 进入目录解析 start_index={}, base_url={}", start_index, base_url);
    let document = Html::parse_document(html);
    let chapter_list_rule = rule
        .chapter_list
        .as_ref()
        .context("缺少目录列表规则")?;

    let mut chapters = Vec::new();
    let mut chapter_index = start_index;
    let mut seen_urls = std::collections::HashSet::new();
    let mut seen_titles = std::collections::HashSet::new();

    if let Some((regex, reversed)) = parse_all_in_one_rule(chapter_list_rule) {
        println!("[toc] 使用 AllInOne 目录规则: {}", chapter_list_rule);
        let mut matches = Vec::new();
        for caps in regex.captures_iter(html) {
            let name = rule
                .chapter_name
                .as_deref()
                .map(|tpl| apply_capture_template(tpl, &caps))
                .unwrap_or_default();
            let url = rule
                .chapter_url
                .as_deref()
                .map(|tpl| apply_capture_template(tpl, &caps))
                .unwrap_or_default();
            if !name.trim().is_empty() && !url.trim().is_empty() {
                matches.push((name, url));
            }
        }

        if reversed {
            matches.reverse();
        }

        println!("[toc] AllInOne 匹配章节数: {}", matches.len());
        for (name, raw_url) in matches {
            let url = resolve_url(base_url, &raw_url).unwrap_or(raw_url.clone());
            
            if !seen_urls.insert(url.clone()) || !seen_titles.insert(name.clone()) {
                println!("[toc] 跳过重复章节: title={}, url={}", name, url);
                continue;
            }
            
            println!("[toc] 章节 #{} -> title: {}, url: {}", chapter_index, name, url);
            chapters.push(Chapter {
                id: 0,
                book_id: String::new(),
                title: name,
                url,
                chapter_index,
                word_count: 0,
                is_vip: false,
            });
            chapter_index += 1;
        }
    } else {
        let normalized_rule = chapter_list_rule.trim();

        let elements = if strip_css_prefix(normalized_rule) == ".section.chapter_list ul li a" {
            let section_selector = Selector::parse(".section.chapter_list")
                .map_err(|e| anyhow::anyhow!("解析目录分区选择器失败: {:?}", e))?;
            let title_selector = Selector::parse(".title")
                .map_err(|e| anyhow::anyhow!("解析目录标题选择器失败: {:?}", e))?;
            let link_selector = Selector::parse("ul li a")
                .map_err(|e| anyhow::anyhow!("解析目录链接选择器失败: {:?}", e))?;
            let sections = document.select(&section_selector).collect::<Vec<_>>();
            if sections.len() >= 2 {
                let first_title = sections
                    .get(0)
                    .and_then(|section| section.select(&title_selector).next())
                    .map(|title| title.text().collect::<String>())
                    .unwrap_or_default();
                if first_title.contains("最新章节") {
                    sections[1].select(&link_selector).collect::<Vec<_>>()
                } else {
                    crate::search::parser::selector::select_elements_from_document(&document, normalized_rule)
                }
            } else {
                crate::search::parser::selector::select_elements_from_document(&document, normalized_rule)
            }
        } else {
            crate::search::parser::selector::select_elements_from_document(&document, normalized_rule)
        };

        println!("[toc] 使用 chapter_list 选择器: {:?}", chapter_list_rule);
        println!("[toc] 匹配到目录元素数量: {}", elements.len());
        if let Some(first) = elements.first() {
            println!("[toc] 第一个目录元素HTML(前500): {}", first.html().chars().take(500).collect::<String>());
        }

        for (element_index, element) in elements.into_iter().enumerate() {
            let chapter_names = rule.chapter_name.as_deref()
                .map(|rule| crate::search::parser::selector::select_text_from_element(&element, rule))
                .unwrap_or_default();
            let chapter_urls = rule.chapter_url.as_deref()
                .map(|rule| crate::search::parser::selector::select_text_from_element(&element, rule))
                .unwrap_or_default();
            let count = chapter_names.len().min(chapter_urls.len());

            println!(
                "[toc] 目录元素 #{} - chapter_names: {}, chapter_urls: {}, 取最小: {}",
                element_index,
                chapter_names.len(),
                chapter_urls.len(),
                count
            );
            if count == 0 {
                println!(
                    "[toc] 目录元素 #{} 无匹配章节，names示例: {:?}, urls示例: {:?}",
                    element_index,
                    chapter_names.get(0),
                    chapter_urls.get(0)
                );
            }

            for idx in 0..count {
                let name = chapter_names[idx].clone();
                let url = resolve_url(base_url, &chapter_urls[idx])
                    .unwrap_or_else(|| chapter_urls[idx].clone());
                
                if !seen_urls.insert(url.clone()) || !seen_titles.insert(name.clone()) {
                    println!("[toc] 跳过重复章节: title={}, url={}", name, url);
                    continue;
                }
                
                println!(
                    "[toc] 章节 #{} -> title: {}, url: {}",
                    chapter_index,
                    name,
                    url
                );
                chapters.push(Chapter {
                    id: 0,
                    book_id: String::new(),
                    title: name,
                    url,
                    chapter_index,
                    word_count: 0,
                    is_vip: false,
                });
                chapter_index += 1;
            }
        }

        println!("[toc] 最终解析章节数: {}", chapters.len());
    }

    if let Some(next_rule) = rule.next_toc_url.as_ref() {
        println!("[toc] next_toc_url 规则: {}", next_rule);
        if let Ok(a_selector) = Selector::parse("a") {
            let mut candidates = Vec::new();
            for a in document.select(&a_selector) {
                let text = a.text().collect::<String>();
                if text.contains("下") || text.contains("页") || text.contains("下页") || text.contains("下一页") {
                    let href = a.value().attr("href").unwrap_or("");
                    candidates.push(format!("{} => {}", text.trim(), href));
                }
            }
            if !candidates.is_empty() {
                println!("[toc] 可能的翻页链接(前5): {:?}", candidates.into_iter().take(5).collect::<Vec<_>>());
            } else {
                println!("[toc] 未发现包含\"下\"或\"页\"字样的链接文本");
            }
        }
    }

    let mut next_toc_url = rule_engine::extract_text_from_document(&document, rule.next_toc_url.as_ref());
    if next_toc_url.is_none() {
        if let Ok(a_selector) = Selector::parse("a") {
            let mut fallback = None;
            for a in document.select(&a_selector) {
                let text = a.text().collect::<String>();
                if text.contains("下─页") || text.contains("下一页") || text.contains("下页") {
                    if let Some(href) = a.value().attr("href") {
                        fallback = Some(href.to_string());
                        break;
                    }
                }
            }
            if let Some(href) = fallback {
                next_toc_url = resolve_url(base_url, &href).or(Some(href));
                if let Some(next) = &next_toc_url {
                    println!("[toc] fallback 匹配到 next_toc_url: {}", next);
                }
            }
        }
    }

    if let Some(next) = &next_toc_url {
        println!("[toc] 匹配到 next_toc_url: {}", next);
    } else {
        println!("[toc] 未匹配到 next_toc_url");
    }

    Ok(TocParseResult {
        chapters,
        next_toc_url,
        next_index: chapter_index,
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

fn strip_css_prefix(rule: &str) -> &str {
    if let Some(rest) = rule.strip_prefix("@css:") {
        return rest.trim();
    }
    if let Some(rest) = rule.strip_prefix("@CSS:") {
        return rest.trim();
    }
    rule
}

pub fn parse_chapter_content(html: &str, rule: &ContentRule) -> Result<String> {
    let document = Html::parse_document(html);
    let content_rule = rule.content.as_ref().context("缺少内容规则")?;

    let content_list = rule_engine::extract_list_single_rule(&document, content_rule);
    if !content_list.is_empty() {
        return Ok(content_list.join("\n\n").trim().to_string());
    }

    Ok(String::new())
}
