use scraper::Html;

use super::inner_rule;
use super::json_path;
use super::js_engine;
use super::rule_split;
use super::selector;

#[derive(Clone, Copy)]
struct RuleContext<'a> {
    result: Option<&'a str>,
    base_url: Option<&'a str>,
    book_json: Option<&'a str>,
}

impl<'a> RuleContext<'a> {
    fn new(result: Option<&'a str>, base_url: Option<&'a str>, book_json: Option<&'a str>) -> Self {
        Self {
            result,
            base_url,
            book_json,
        }
    }

    fn empty() -> Self {
        Self {
            result: None,
            base_url: None,
            book_json: None,
        }
    }

    fn eval_js(&self, code: &str, result_override: Option<&str>) -> Option<String> {
        js_engine::eval_js_with_context(code, result_override.or(self.result), self.base_url, self.book_json)
    }
}

pub fn extract_text_from_document(document: &Html, selector: Option<&String>) -> Option<String> {
    extract_text_from_document_with_context(document, selector, None, None, None)
}

pub fn extract_text_from_document_with_context(
    document: &Html,
    selector: Option<&String>,
    result: Option<&str>,
    base_url: Option<&str>,
    book_json: Option<&str>,
) -> Option<String> {
    let selector_str = selector?.trim();
    if selector_str.is_empty() {
        return None;
    }

    let ctx = RuleContext::new(result, base_url, book_json);
    let rules = rule_split::split_rules(selector_str, &["&&", "||", "%%"]);
    let op = rule_split::detect_operator(selector_str, &["&&", "||", "%%"]);

    let mut results = Vec::new();
    for rule in rules {
        let values = extract_values(document, &rule, ctx);
        if !values.is_empty() {
            results.push(values);
            if op == Some("||") {
                break;
            }
        }
    }

    merge_rules_results(results, op)
}

pub fn extract_list_single_rule(document: &Html, rule: &str) -> Vec<String> {
    extract_list_single_rule_with_context(document, rule, None, None, None)
}

pub fn extract_text_from_element(element: &scraper::ElementRef<'_>, rule: &str) -> Option<String> {
    extract_values_from_element(element, rule, RuleContext::empty())
        .into_iter()
        .find(|v| !v.is_empty())
}

pub fn extract_list_from_element(element: &scraper::ElementRef<'_>, rule: &str) -> Vec<String> {
    extract_values_from_element(element, rule, RuleContext::empty())
}

fn extract_values_from_element(
    element: &scraper::ElementRef<'_>,
    rule: &str,
    ctx: RuleContext<'_>,
) -> Vec<String> {
    let rule = rule.trim();
    if rule.is_empty() {
        return Vec::new();
    }

    let (rule, js_code) = split_js_chain(rule);
    if let Some(result_js) = extract_js_rule(rule, ctx) {
        return vec![result_js];
    }

    let mut values = if json_path::is_json_path_rule(rule) {
        json_path::extract_json_path_values_from_element(element, rule)
    } else {
        selector::select_text_from_element(element, rule)
    };

    if let Some(code) = js_code {
        values = values
            .into_iter()
            .filter_map(|value| apply_js_chain(value, Some(code), ctx))
            .collect();
    }

    values
}

pub fn extract_list_single_rule_with_context(
    document: &Html,
    rule: &str,
    result: Option<&str>,
    base_url: Option<&str>,
    book_json: Option<&str>,
) -> Vec<String> {
    extract_values(document, rule, RuleContext::new(result, base_url, book_json))
}

pub fn extract_single_rule(document: &Html, rule: &str) -> Option<String> {
    extract_single_rule_with_context(document, rule, None, None, None)
}

pub fn extract_single_rule_with_context(
    document: &Html,
    rule: &str,
    result: Option<&str>,
    base_url: Option<&str>,
    book_json: Option<&str>,
) -> Option<String> {
    extract_values(document, rule, RuleContext::new(result, base_url, book_json))
        .into_iter()
        .find(|v| !v.is_empty())
}

fn extract_values(document: &Html, rule: &str, ctx: RuleContext<'_>) -> Vec<String> {
    let rule = rule.trim();
    if rule.is_empty() {
        return Vec::new();
    }

    let (rule, js_code) = split_js_chain(rule);

    if let Some(inner) = inner_rule::extract_inner_rule(document, rule) {
        return apply_js_chain(inner, js_code, ctx).into_iter().collect();
    }

    if let Some(result_js) = extract_js_rule(rule, ctx) {
        return vec![result_js];
    }

    let mut values = if json_path::is_json_path_rule(rule) {
        json_path::extract_json_path_values(document, rule)
    } else {
        selector::select_text(document, rule)
    };

    if let Some(code) = js_code {
        values = values
            .into_iter()
            .filter_map(|value| apply_js_chain(value, Some(code), ctx))
            .collect();
    }

    values
}

fn apply_js_chain(value: String, js_code: Option<&str>, ctx: RuleContext<'_>) -> Option<String> {
    if let Some(code) = js_code {
        ctx.eval_js(code, Some(&value))
    } else {
        Some(value)
    }
}

fn extract_js_rule(rule: &str, ctx: RuleContext<'_>) -> Option<String> {
    let rule = rule.trim();
    if let Some(code) = rule.strip_prefix("<js>").and_then(|r| r.strip_suffix("</js>")) {
        return ctx.eval_js(code, None);
    }

    if let Some(code) = rule.strip_prefix("@js:") {
        return ctx.eval_js(code, None);
    }

    None
}

fn split_js_chain(rule: &str) -> (&str, Option<&str>) {
    if let Some((base, js)) = rule.split_once("<js>") {
        if let Some((code, _rest)) = js.split_once("</js>") {
            return (base.trim(), Some(code));
        }
    }
    if let Some((base, code)) = rule.split_once("@js:") {
        return (base.trim(), Some(code.trim()));
    }
    (rule, None)
}

fn merge_rules_results(results: Vec<Vec<String>>, op: Option<&str>) -> Option<String> {
    if results.is_empty() {
        return None;
    }

    let merged = if op == Some("%%") {
        let mut merged = Vec::new();
        let max_len = results.iter().map(|v| v.len()).max().unwrap_or(0);
        for i in 0..max_len {
            for arr in &results {
                if let Some(value) = arr.get(i) {
                    merged.push(value.clone());
                }
            }
        }
        merged
    } else {
        results.into_iter().flatten().collect::<Vec<_>>()
    };

    merged.into_iter().find(|v| !v.is_empty())
}
