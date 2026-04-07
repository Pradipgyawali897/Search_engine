use crate::globals::VISITED_URLS;
use spyder::normalize_url;
use std::collections::hash_map::DefaultHasher;
use std::fs::OpenOptions;
use std::hash::{Hash, Hasher};
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use url::Url;

use super::link_filter::LinkCategory;
use crate::storage::schema::schema::DiscoveredLink;

pub fn is_valid_url(s: &str) -> bool {
    let has_prefix = s.starts_with("http://") || s.starts_with("https://") || s.starts_with("www.");
    if !has_prefix {
        return false;
    }

    if s.starts_with("www.") {
        Url::parse(&format!("https://{}", s)).is_ok()
    } else {
        Url::parse(s).is_ok()
    }
}

pub fn sanitize_url_candidate(raw: &str) -> Option<String> {
    let trimmed =
        raw.trim_matches(|c: char| c.is_whitespace() || matches!(c, '"' | '\'' | '<' | '>'));
    let sanitized = trimmed.trim_end_matches(|c: char| {
        matches!(c, '.' | ',' | ';' | ':' | '!' | '?' | ')' | ']' | '}')
    });

    if sanitized.is_empty() {
        None
    } else {
        Some(sanitized.to_string())
    }
}

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

pub fn create_hash(s: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

pub fn save_url(url: &str, category: LinkCategory) {
    match category {
        LinkCategory::Visitable => {
            let normalized = match normalize_url(url) {
                Some(u) => u,
                None => return,
            };
            let hash = create_hash(&normalized);
            {
                let mut visited = VISITED_URLS.lock().unwrap();
                if !visited.insert(hash) {
                    return;
                }
            }
            if let Ok(mut file) = OpenOptions::new()
                .create(true)
                .append(true)
                .open("visitable_urls.txt")
            {
                let _ = writeln!(file, "{}", normalized);
            }
        }
        LinkCategory::Junk => {
            let discovered_link = DiscoveredLink {
                url: url.to_string(),
                category: "junk".to_string(),
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
            };
            if let Ok(mut file) = OpenOptions::new()
                .create(true)
                .append(true)
                .open("junk_urls.json")
            {
                if let Ok(json) = serde_json::to_string(&discovered_link) {
                    let _ = writeln!(file, "{}", json);
                }
            }
        }
    }
}

fn canonicalize_token(raw: &str) -> String {
    let trimmed = raw.trim_matches(|c: char| !is_token_edge_char(c));
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

fn push_unique(tokens: &mut Vec<String>, token: String) {
    if !token.is_empty() && !tokens.iter().any(|existing| existing == &token) {
        tokens.push(token);
    }
}
