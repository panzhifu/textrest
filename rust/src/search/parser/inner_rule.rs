use scraper::Html;

use super::rule_engine;

pub fn replace_inner_rule(document: &Html, input: &str) -> String {
    let mut output = String::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '{' && chars.peek() == Some(&'$') {
            if let Some(replaced) = read_inner_rule(document, &mut chars) {
                output.push_str(&replaced);
                continue;
            }
        }
        output.push(ch);
    }

    output
}

pub fn extract_inner_rule(document: &Html, rule: &str) -> Option<String> {
    let replaced = replace_inner_rule(document, rule);
    (replaced != rule).then_some(replaced)
}

fn read_inner_rule(document: &Html, chars: &mut std::iter::Peekable<std::str::Chars<'_>>) -> Option<String> {
    let mut depth = 1;
    let mut buffer = String::new();

    while let Some(ch) = chars.next() {
        if ch == '{' {
            depth += 1;
        } else if ch == '}' {
            depth -= 1;
            if depth == 0 {
                break;
            }
        }
        buffer.push(ch);
    }

    if !buffer.starts_with("$.") {
        return None;
    }

    let inner_rule = buffer.trim_start_matches("$.");
    let value = rule_engine::extract_single_rule(document, inner_rule).unwrap_or_default();
    Some(value)
}
