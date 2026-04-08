use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentListItem {
    pub id: i64,
    pub crawl_target_id: Option<i64>,
    pub canonical_url: String,
    pub host: String,
    pub path: String,
    pub title: Option<String>,
    pub summary: String,
    pub content_type: Option<String>,
    pub http_status: Option<i16>,
    pub language: Option<String>,
    pub content_length: i64,
    pub extracted_links_count: i32,
    pub fetched_at: i64,
    pub indexed_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentDetail {
    pub id: i64,
    pub crawl_target_id: Option<i64>,
    pub canonical_url: String,
    pub host: String,
    pub path: String,
    pub title: Option<String>,
    pub summary: String,
    pub plain_text: String,
    pub raw_html: Option<String>,
    pub content_type: Option<String>,
    pub http_status: Option<i16>,
    pub language: Option<String>,
    pub content_length: i64,
    pub extracted_links_count: i32,
    pub fetched_at: i64,
    pub indexed_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentListMeta {
    pub count: usize,
    pub limit: u32,
    pub cached: bool,
    pub state: String,
    pub refreshed_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentListEnvelope {
    pub items: Vec<ContentListItem>,
    pub meta: ContentListMeta,
}

impl ContentListEnvelope {
    pub fn fresh(items: Vec<ContentListItem>, limit: u32) -> Self {
        Self {
            meta: ContentListMeta {
                count: items.len(),
                limit,
                cached: false,
                state: "fresh".to_string(),
                refreshed_at: now_unix_seconds(),
            },
            items,
        }
    }

    pub fn cached(mut self) -> Self {
        self.meta.cached = true;
        self.meta.state = "cached".to_string();
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentDetailMeta {
    pub document_id: i64,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentDetailEnvelope {
    pub content: ContentDetail,
    pub meta: ContentDetailMeta,
}

impl ContentDetailEnvelope {
    pub fn fresh(content: ContentDetail) -> Self {
        Self {
            meta: ContentDetailMeta {
                document_id: content.id,
                state: "fresh".to_string(),
            },
            content,
        }
    }
}

pub fn summarize_text(text: &str, limit: usize) -> String {
    let normalized = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if normalized.is_empty() {
        return String::new();
    }

    if normalized.len() <= limit {
        return normalized;
    }

    let mut end = limit;
    while end > 0 && !normalized.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}...", &normalized[..end])
}

pub fn now_unix_seconds() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}
