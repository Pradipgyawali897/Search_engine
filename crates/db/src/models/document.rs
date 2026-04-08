use crate::error::DbResult;
use crate::models::url::parse_canonical_url;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Document {
    pub id: Option<i64>,
    pub crawl_target_id: Option<i64>,
    pub canonical_url: String,
    pub scheme: String,
    pub host: String,
    pub path: String,
    pub title: Option<String>,
    pub content_type: Option<String>,
    pub http_status: Option<i16>,
    pub etag: Option<String>,
    pub content_length: i64,
    pub checksum: Option<String>,
    pub language: Option<String>,
}

impl Document {
    pub fn new(canonical_url: impl Into<String>) -> DbResult<Self> {
        let canonical_url = canonical_url.into();
        let parts = parse_canonical_url(&canonical_url)?;

        Ok(Self {
            id: None,
            crawl_target_id: None,
            canonical_url: parts.canonical_url,
            scheme: parts.scheme,
            host: parts.host,
            path: parts.path,
            title: None,
            content_type: None,
            http_status: None,
            etag: None,
            content_length: 0,
            checksum: None,
            language: None,
        })
    }
}
