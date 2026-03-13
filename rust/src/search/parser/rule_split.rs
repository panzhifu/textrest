pub fn split_rules(rule: &str, operators: &[&str]) -> Vec<String> {
    let mut results = Vec::new();
    let mut current = String::new();
    let mut depth = 0i32;
    let mut chars = rule.char_indices().peekable();

    while let Some((idx, ch)) = chars.next() {
        if ch == '{' {
            depth += 1;
        } else if ch == '}' && depth > 0 {
            depth -= 1;
        }

        if depth == 0 {
            if let Some(op) = operators.iter().find(|op| peek_operator(rule, idx, op)) {
                if !current.trim().is_empty() {
                    results.push(current.trim().to_string());
                }
                current.clear();
                for _ in 1..op.len() {
                    chars.next();
                }
                continue;
            }
        }

        current.push(ch);
    }

    if !current.trim().is_empty() {
        results.push(current.trim().to_string());
    }

    results
}

pub fn detect_operator<'a>(rule: &str, operators: &'a [&'a str]) -> Option<&'a str> {
    for op in operators {
        if rule.contains(op) {
            return Some(*op);
        }
    }
    None
}

fn peek_operator(source: &str, index: usize, op: &&str) -> bool {
    source[index..].starts_with(*op)
}
