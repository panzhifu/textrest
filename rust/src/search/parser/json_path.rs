use jsonpath_lib::select;
use scraper::Html;
use serde_json::Value;

pub fn is_json_path_rule(rule: &str) -> bool {
    rule.starts_with('$') || rule.starts_with('{')
}

pub fn extract_json_path_values(document: &Html, rule: &str) -> Vec<String> {
    let json_text = document.root_element().text().collect::<String>();
    extract_json_path_values_from_text(&json_text, rule)
}

pub fn extract_json_path_values_from_element(element: &scraper::ElementRef<'_>, rule: &str) -> Vec<String> {
    let json_text = element.text().collect::<String>();
    extract_json_path_values_from_text(&json_text, rule)
}

fn extract_json_path_values_from_text(json_text: &str, rule: &str) -> Vec<String> {
    if json_text.trim().is_empty() {
        return Vec::new();
    }

    let json_value: Value = match serde_json::from_str(json_text) {
        Ok(value) => value,
        Err(_) => return Vec::new(),
    };

    select(&json_value, rule)
        .ok()
        .into_iter()
        .flatten()
        .filter_map(|value| {
            if let Some(str_value) = value.as_str() {
                let trimmed = str_value.trim();
                if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed.to_string())
                }
            } else if value.is_null() {
                None
            } else {
                Some(value.to_string())
            }
        })
        .collect()
}
