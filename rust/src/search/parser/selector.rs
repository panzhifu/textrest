use regex::Regex;
use scraper::{ElementRef, Html, Selector};

/// CSS 提取目标类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtractTarget {
    /// 元素文本（含子节点）
    Text,
    /// 文本节点（这里等价于 Text，保留兼容）
    TextNodes,
    /// 元素自身文本（不含子节点，scraper 不区分，保持兼容）
    OwnText,
    /// 元素 HTML
    Html,
    /// 元素完整 HTML（等价 Html）
    All,
    /// 读取属性
    Attr,
}

/// CSS 规则解析结果
#[derive(Debug, Clone)]
pub struct CssExtractRule<'a> {
    /// CSS 选择器
    pub selector: &'a str,
    /// 取值类型
    pub target: ExtractTarget,
    /// 属性名（仅 Attr 有效）
    pub attr: Option<&'a str>,
    /// 文本前缀裁剪
    pub trim_prefix: Option<&'a str>,
    /// 正则替换规则（pattern, replacement）
    pub replace_regex: Option<(String, String)>,
}

/// 索引筛选模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexMode {
    /// 选择指定索引
    Select,
    /// 排除指定索引
    Exclude,
    /// 不进行索引筛选
    None,
}

/// 索引规则
#[derive(Debug, Clone)]
pub struct IndexRule {
    pub mode: IndexMode,
    pub indexes: Vec<IndexToken>,
}

/// 索引 token：单个索引或区间
#[derive(Debug, Clone)]
pub enum IndexToken {
    Single(i32),
    Range { start: Option<i32>, end: Option<i32>, step: i32 },
}

/// 解析 `css@text/html/attr##前缀` 或 `css@text##正则##替换` 语法
pub fn parse_css_rule(rule: &str) -> CssExtractRule<'_> {
    let (selector_part, extract_part) = rule
        .split_once('@')
        .map(|(a, b)| (a, Some(b)))
        .unwrap_or((rule, None));

    let (extract_part, trim_prefix, replace_regex) = split_extract_and_replace(extract_part);

    let (target, attr) = match extract_part {
        "text" => (ExtractTarget::Text, None),
        "textNodes" => (ExtractTarget::TextNodes, None),
        "ownText" => (ExtractTarget::OwnText, None),
        "html" => (ExtractTarget::Html, None),
        "all" => (ExtractTarget::All, None),
        other => (ExtractTarget::Attr, Some(other)),
    };

    CssExtractRule {
        selector: selector_part,
        target,
        attr,
        trim_prefix,
        replace_regex,
    }
}

/// 从文档中提取文本，支持索引筛选和 `@` 链式规则
pub fn select_text(document: &Html, rule: &str) -> Vec<String> {
    let mut results = Vec::new();
    let rule = strip_css_prefix(rule);

    if let Some((left, right)) = split_or_rule(rule) {
        let left_results = select_text(document, left);
        if !left_results.is_empty() {
            return left_results;
        }
        return select_text(document, right);
    }

    if let Some((left, right)) = split_and_rule(rule) {
        let mut left_results = select_text(document, left);
        if left_results.is_empty() {
            return Vec::new();
        }
        let right_results = select_text(document, right);
        left_results.extend(right_results);
        return left_results;
    }

    if rule.contains('@') {
        return select_text_legacy(document, rule);
    }

    let (rule, reverse) = strip_reverse_prefix(rule);
    let (rule, index_rule) = split_index_rule(rule);
    let rule = parse_css_rule(rule);
    let mut elements: Vec<_> = match select_with_jsoup(document, rule.selector) {
        Ok(elements) => elements,
        Err(_) => return results,
    };
    if reverse {
        elements.reverse();
    }

    for element in apply_index_rule_if_needed(elements, index_rule) {
        if let Some(value) = extract_value_from_element(&element, &rule) {
            results.push(value);
        }
    }

    results
}

/// 从元素中提取文本，支持 `@text/html/attr` 简写
pub fn select_text_from_element(element: &ElementRef<'_>, rule: &str) -> Vec<String> {
    let mut results = Vec::new();
    let rule = strip_css_prefix(rule).trim();

    if let Some((left, right)) = split_or_rule(rule) {
        let left_results = select_text_from_element(element, left);
        if !left_results.is_empty() {
            return left_results;
        }
        return select_text_from_element(element, right);
    }

    if let Some((left, right)) = split_and_rule(rule) {
        let mut left_results = select_text_from_element(element, left);
        if left_results.is_empty() {
            return Vec::new();
        }
        let right_results = select_text_from_element(element, right);
        left_results.extend(right_results);
        return left_results;
    }

    if is_inline_value_rule(rule) {
        if let Some(value) = extract_value_by_rule(element, rule) {
            results.push(value);
        }
        return results;
    }

    if let Some(rest) = rule.strip_prefix('@') {
        if let Some(value) = extract_value_by_rule(element, rest) {
            results.push(value);
        }
        return results;
    }

    if let Some((selector_part, extract_part)) = rule.split_once('@') {
        if selector_part.is_empty() {
            if let Some(value) = extract_value_by_rule(element, extract_part) {
                results.push(value);
            }
            return results;
        }

        let (selector_part, reverse) = strip_reverse_prefix(selector_part);
        let (selector_part, index_rule) = split_index_rule(selector_part);
        let mut elements = select_elements_from_element(*element, selector_part);
        if reverse {
            elements.reverse();
        }
        if let Some(index_rule) = index_rule {
            elements = apply_index_rule(elements, &index_rule);
        }

        for child in elements {
            if let Some(value) = extract_value_by_rule(&child, extract_part) {
                results.push(value);
            }
        }
    } else {
        let (rule, reverse) = strip_reverse_prefix(rule);
        let mut elements = select_elements_from_element(*element, rule);
        if reverse {
            elements.reverse();
        }
        for child in elements {
            let value = child.text().collect::<String>().trim().to_string();
            if !value.is_empty() {
                results.push(value);
            }
        }
    }

    results
}

/// 按规则选择元素，支持 children/class/tag/id/text 分支
pub fn select_elements_from_element<'a>(element: ElementRef<'a>, rule: &str) -> Vec<ElementRef<'a>> {
    let rule = rule.trim();
    if rule.is_empty() {
        return Vec::new();
    }

    let rule = strip_css_prefix(rule);
    
    // 检查是否是索引简写（如 ".1", ".-1"）
    if rule.starts_with('.') && (rule[1..].chars().next().map_or(false, |c| c.is_ascii_digit() || c == '-')) {
        let index_str = &rule[1..];
        // 等价于 children[index]
        let elements = element.children().filter_map(ElementRef::wrap).collect::<Vec<_>>();
        // 把 .1 转换成 [1] 格式
        let index_rule = parse_index_list(index_str);
        return apply_index_rule_if_needed(elements, index_rule);
    }

    let (rule, index_rule) = split_index_rule(rule);

    let elements = if let Some((prefix, rest)) = rule.split_once('.') {
        // 检查是否有第三段（位置）
        let (value, position_str) = if let Some((v, p)) = rest.split_once('.') {
            (v, Some(p))
        } else {
            (rest, None)
        };

        let mut elements = match prefix {
            "children" => element.children().filter_map(ElementRef::wrap).collect(),
            "class" => {
                let selector = format!(".{}", value);
                select_with_jsoup_element(element, &selector).unwrap_or_default()
            }
            "tag" => select_with_jsoup_element(element, value).unwrap_or_default(),
            "id" => {
                let selector = format!("#{}", value);
                select_with_jsoup_element(element, &selector).unwrap_or_default()
            }
            "text" => {
                let target = value.trim();
                element
                    .select(&Selector::parse("*").unwrap())
                    .filter(|el| el.text().collect::<String>().contains(target))
                    .collect()
            }
            _ => select_with_jsoup_element(element, rule).unwrap_or_default(),
        };

        // 如果有第三段（位置），应用位置筛选
        if let Some(pos) = position_str {
            let pos_rule = parse_index_list(pos);
            elements = apply_index_rule_if_needed(elements, pos_rule);
        }

        elements
    } else {
        select_with_jsoup_element(element, rule).unwrap_or_default()
    };

    apply_index_rule_if_needed(elements, index_rule)
}

/// 使用 legacy 链式 `@` 规则执行（a@b@c）
pub fn select_text_legacy(document: &Html, rule: &str) -> Vec<String> {
    let root = document.root_element();
    select_text_from_elements(vec![root], rule)
}

/// 使用 legacy 链式 `@` 规则选择元素
pub fn select_elements_from_document<'a>(document: &'a Html, rule: &str) -> Vec<ElementRef<'a>> {
    let root = document.root_element();
    select_elements_from_elements(vec![root], rule)
}

/// 按链式规则依次选择元素，再从最后一级提取文本
pub fn select_text_from_elements<'a>(mut elements: Vec<ElementRef<'a>>, rule: &str) -> Vec<String> {
    let rule = rule.trim();
    if rule.is_empty() {
        return Vec::new();
    }

    let parts: Vec<&str> = rule.split('@').filter(|p| !p.trim().is_empty()).collect();
    if parts.is_empty() {
        return Vec::new();
    }

    for part in parts.iter().take(parts.len().saturating_sub(1)) {
        let mut next = Vec::new();
        for element in elements {
            next.extend(select_elements_from_element(element, part));
        }
        elements = next;
    }

    let last = parts.last().copied().unwrap_or("");
    let mut results = Vec::new();
    for element in elements {
        results.extend(select_text_from_element(&element, last));
    }
    results
}

pub fn select_elements_from_elements<'a>(mut elements: Vec<ElementRef<'a>>, rule: &str) -> Vec<ElementRef<'a>> {
    let rule = rule.trim();
    if rule.is_empty() {
        return Vec::new();
    }

    let parts: Vec<&str> = rule.split('@').filter(|p| !p.trim().is_empty()).collect();
    if parts.is_empty() {
        return Vec::new();
    }

    for part in parts.iter() {
        let mut next = Vec::new();
        for element in elements {
            next.extend(select_elements_from_element(element, part));
        }
        elements = next;
    }

    elements
}

/// 根据目标类型从元素中取值
fn extract_value_from_element(element: &ElementRef<'_>, rule: &CssExtractRule<'_>) -> Option<String> {
    let mut value = match rule.target {
        ExtractTarget::Text | ExtractTarget::OwnText => element.text().collect::<String>(),
        ExtractTarget::TextNodes => element
            .text()
            .map(|t| t.trim())
            .filter(|t| !t.is_empty())
            .collect::<Vec<&str>>()
            .join("\n"),
        ExtractTarget::Html | ExtractTarget::All => element.html(),
        ExtractTarget::Attr => element
            .value()
            .attr(rule.attr.unwrap_or_default())
            .unwrap_or_default()
            .to_string(),
    };

    value = value.trim().to_string();
    if let Some(prefix) = rule.trim_prefix {
        value = value.trim_start_matches(prefix).trim().to_string();
    }
    if let Some((pattern, replacement)) = &rule.replace_regex {
        if let Ok(re) = Regex::new(pattern) {
            value = re.replace_all(&value, replacement.as_str()).to_string();
        }
    }

    if value.is_empty() { None } else { Some(value) }
}

fn extract_value_by_rule(element: &ElementRef<'_>, rule: &str) -> Option<String> {
    let rule = rule.trim();
    if is_inline_value_rule(rule) {
        return extract_value_from_element(element, &parse_css_rule(rule));
    }
    if !rule.contains('@') && !looks_like_selector(rule) {
        let value = element.value().attr(rule).unwrap_or_default().to_string();
        return if value.is_empty() { None } else { Some(value) };
    }
    extract_value_from_element(element, &parse_css_rule(rule))
}

fn looks_like_selector(rule: &str) -> bool {
    rule.chars().any(|ch| matches!(ch, ' ' | '.' | '#' | '[' | ']' | ':' | '>' | '+' | '~' | '*'))
}

fn split_or_rule(rule: &str) -> Option<(&str, &str)> {
    rule.split_once("||")
}

fn split_and_rule(rule: &str) -> Option<(&str, &str)> {
    rule.split_once("&&")
}

fn strip_reverse_prefix(rule: &str) -> (&str, bool) {
    let trimmed = rule.trim_start();
    if let Some(rest) = trimmed.strip_prefix('-') {
        (rest.trim_start(), true)
    } else {
        (rule, false)
    }
}

fn split_extract_and_replace(extract_part: Option<&str>) -> (&str, Option<&str>, Option<(String, String)>) {
    let extract_part = extract_part.unwrap_or("text");
    let mut segments = extract_part.split("##");
    let base = segments.next().unwrap_or("text");
    let mut remainder: Vec<&str> = segments.collect();
    if remainder.is_empty() {
        return (base, None, None);
    }
    if remainder.len() == 1 {
        return (base, Some(remainder[0]), None);
    }
    let pattern = remainder.remove(0).to_string();
    let replacement = remainder.join("##");
    let replacement = if replacement.is_empty() {
        String::new()
    } else {
        replacement
    };
    (base, None, Some((pattern, replacement)))
}

fn is_inline_value_rule(rule: &str) -> bool {
    matches!(rule, "text" | "textNodes" | "ownText" | "html" | "all")
}

fn apply_index_rule_if_needed<'a>(elements: Vec<ElementRef<'a>>, rule: Option<IndexRule>) -> Vec<ElementRef<'a>> {
    match rule {
        Some(rule) => apply_index_rule(elements, &rule),
        None => elements,
    }
}

/// 识别 @CSS: 前缀（大小写兼容）
fn strip_css_prefix(rule: &str) -> &str {
    if let Some(rest) = rule.strip_prefix("@css:") {
        return rest.trim();
    }
    if let Some(rest) = rule.strip_prefix("@CSS:") {
        return rest.trim();
    }
    rule
}

/// 拆出索引规则（[] 或 legacy . / ! 语法）
fn split_index_rule(rule: &str) -> (&str, Option<IndexRule>) {
    if let Some(start) = rule.rfind('[') {
        if rule.ends_with(']') {
            let base = &rule[..start];
            let inner = &rule[start + 1..rule.len() - 1];
            let index_rule = parse_index_list(inner);
            return (base.trim(), index_rule);
        }
    }

    let mut idx = rule.len();
    for (i, ch) in rule.char_indices().rev() {
        if ch.is_ascii_digit() || ch == '-' || ch == ':' {
            idx = i;
        } else {
            break;
        }
    }
    if idx < rule.len() {
        let (base, rest) = rule.split_at(idx);
        if let Some(mode_char) = base.chars().last() {
            if mode_char == '.' || mode_char == '!' {
                let base = base[..base.len() - 1].trim();
                let index_rule = parse_index_legacy(rest, mode_char);
                return (base, index_rule);
            }
        }
    }

    (rule.trim(), None)
}

/// 解析 [] 形式索引
fn parse_index_list(inner: &str) -> Option<IndexRule> {
    let mut mode = IndexMode::Select;
    let inner = inner.trim();
    let inner = if let Some(rest) = inner.strip_prefix('!') {
        mode = IndexMode::Exclude;
        rest
    } else {
        inner
    };

    let mut indexes = Vec::new();
    for item in inner.split(',').map(|i| i.trim()).filter(|i| !i.is_empty()) {
        if let Some(token) = parse_range_token(item) {
            indexes.push(token);
        }
    }

    if indexes.is_empty() {
        None
    } else {
        Some(IndexRule { mode, indexes })
    }
}

/// 解析 legacy `.1:3` / `!1:3` 形式索引
fn parse_index_legacy(rest: &str, mode_char: char) -> Option<IndexRule> {
    let mut indexes = Vec::new();
    for item in rest.split(':').filter(|i| !i.is_empty()) {
        if let Ok(value) = item.trim().parse::<i32>() {
            indexes.push(IndexToken::Single(value));
        }
    }
    if indexes.is_empty() {
        None
    } else {
        Some(IndexRule {
            mode: if mode_char == '!' { IndexMode::Exclude } else { IndexMode::Select },
            indexes,
        })
    }
}

/// 解析单个索引 token（区间或单值）
fn parse_range_token(token: &str) -> Option<IndexToken> {
    if token.contains(':') {
        let mut parts = token.split(':').collect::<Vec<_>>();
        while parts.len() < 3 {
            parts.push("");
        }
        let start = parse_optional_i32(parts[0]);
        let end = parse_optional_i32(parts[1]);
        let step = parse_optional_i32(parts[2]).unwrap_or(1);
        Some(IndexToken::Range { start, end, step })
    } else {
        parse_optional_i32(token).map(IndexToken::Single)
    }
}

/// 解析可选的整数
fn parse_optional_i32(value: &str) -> Option<i32> {
    let value = value.trim();
    if value.is_empty() {
        None
    } else {
        value.parse::<i32>().ok()
    }
}

/// 根据索引规则过滤元素
fn apply_index_rule<'a>(elements: Vec<ElementRef<'a>>, rule: &IndexRule) -> Vec<ElementRef<'a>> {
    let len = elements.len() as i32;
    if len == 0 {
        return elements;
    }

    let mut indexes = Vec::new();
    for token in &rule.indexes {
        match token {
            IndexToken::Single(idx) => indexes.push(normalize_index(*idx, len)),
            IndexToken::Range { start, end, step } => {
                let start = start.map(|v| normalize_index(v, len)).unwrap_or(0);
                let end = end.map(|v| normalize_index(v, len)).unwrap_or(len - 1);
                let step = if *step == 0 { 1 } else { *step };
                if start <= end {
                    let mut i = start;
                    while i <= end {
                        indexes.push(i);
                        i += step;
                    }
                } else {
                    let mut i = start;
                    while i >= end {
                        indexes.push(i);
                        i -= step.abs();
                    }
                }
            }
        }
    }

    indexes.sort();
    indexes.dedup();

    match rule.mode {
        IndexMode::Select => indexes
            .into_iter()
            .filter_map(|i| elements.get(i as usize).cloned())
            .collect(),
        IndexMode::Exclude => elements
            .into_iter()
            .enumerate()
            .filter(|(i, _)| !indexes.contains(&(*i as i32)))
            .map(|(_, el)| el)
            .collect(),
        IndexMode::None => elements,
    }
}

fn normalize_index(index: i32, len: i32) -> i32 {
    if index >= 0 {
        index
    } else {
        len + index
    }
}

fn split_contains_selector(selector: &str) -> Option<(String, String)> {
    let marker = ":contains(";
    let start = selector.find(marker)?;
    if !selector.ends_with(')') {
        return None;
    }
    let base = selector[..start].trim();
    let needle = selector[start + marker.len()..selector.len() - 1].trim();
    if needle.is_empty() {
        return None;
    }
    let base = if base.is_empty() { "*" } else { base };
    Some((base.to_string(), needle.to_string()))
}

fn select_with_jsoup<'a>(document: &'a Html, selector: &str) -> Result<Vec<ElementRef<'a>>, ()> {
    if let Ok(selector) = Selector::parse(selector) {
        return Ok(document.select(&selector).collect());
    }

    if let Some((base, needle)) = split_contains_selector(selector) {
        if let Ok(base_selector) = Selector::parse(&base) {
            let elements = document
                .select(&base_selector)
                .filter(|el| el.text().collect::<String>().contains(&needle))
                .collect();
            return Ok(elements);
        }
    }

    Err(())
}

fn select_with_jsoup_element<'a>(element: ElementRef<'a>, selector: &str) -> Result<Vec<ElementRef<'a>>, ()> {
    if let Ok(selector) = Selector::parse(selector) {
        let mut elements: Vec<_> = element.select(&selector).collect();
        if selector.matches(&element) {
            elements.insert(0, element);
        }
        return Ok(elements);
    }

    if let Some((base, needle)) = split_contains_selector(selector) {
        if let Ok(base_selector) = Selector::parse(&base) {
            let mut elements: Vec<_> = element
                .select(&base_selector)
                .filter(|el| el.text().collect::<String>().contains(&needle))
                .collect();
            if base_selector.matches(&element) && element.text().collect::<String>().contains(&needle) {
                elements.insert(0, element);
            }
            return Ok(elements);
        }
    }

    Err(())
}
