use crate::discovery::is_valid_url;
use crate::discovery::sanitize_url_candidate;

pub fn normalize_token(raw: &str) -> Vec<String> {
    if sanitize_url_candidate(raw)
        .as_ref()
        .is_some_and(|candidate| is_valid_url(candidate))
    {
        return Vec::new();
    }

    let canonical = canonicalize_token(raw);
    if canonical.is_empty() {
        return Vec::new();
    }

    let mut normalized = Vec::new();
    push_unique(&mut normalized, canonical.clone());

    if let Some(stripped) = canonical
        .strip_suffix("'s")
        .or_else(|| canonical.strip_suffix("’s"))
        .filter(|value| !value.is_empty())
    {
        push_unique(&mut normalized, stripped.to_string());
    }

    if canonical.contains('\'') || canonical.contains('’') {
        let compact = canonical.replace(['\'', '’'], "");
        if compact.len() > 1 {
            push_unique(&mut normalized, compact);
        }
    }

    if canonical.contains('-') || canonical.contains('_') {
        let joined = canonical.replace(['-', '_'], "");
        if joined.len() > 1 {
            push_unique(&mut normalized, joined);
        }

        for part in canonical.split(['-', '_']) {
            if part.len() > 1 {
                push_unique(&mut normalized, part.to_string());
            }
        }
    }

    normalized
}

fn canonicalize_token(raw: &str) -> String {
    let trimmed = if raw.chars().next().is_some_and(|c| c.is_numeric()) {
        raw.trim_matches(|c: char| !is_numeric_edge_char(c))
    } else {
        raw.trim_matches(|c: char| !is_token_edge_char(c))
    };

    if trimmed.is_empty() {
        return String::new();
    }

    let lower = trimmed
        .chars()
        .flat_map(char::to_lowercase)
        .collect::<String>();
    lower
        .trim_matches(|c: char| matches!(c, '-' | '_' | '\'' | '’'))
        .to_string()
}

fn is_token_edge_char(c: char) -> bool {
    c.is_alphanumeric() || matches!(c, '-' | '_' | '\'' | '’')
}

fn is_numeric_edge_char(c: char) -> bool {
    c.is_numeric() || matches!(c, '.' | ',' | ':' | '/' | '-')
}

fn push_unique(tokens: &mut Vec<String>, token: String) {
    if !token.is_empty() && !tokens.iter().any(|existing| existing == &token) {
        tokens.push(token);
    }
}
