use crate::error::DbResult;
use crate::models::url::parse_canonical_url;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CrawlStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

impl CrawlStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Processing => "processing",
            Self::Completed => "completed",
            Self::Failed => "failed",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CrawlTarget {
    pub id: Option<i64>,
    pub canonical_url: String,
    pub scheme: String,
    pub host: String,
    pub path: String,
    pub status: CrawlStatus,
    pub depth: i32,
    pub priority: i32,
    pub retry_count: i32,
}

impl CrawlTarget {
    pub fn new(canonical_url: impl Into<String>) -> DbResult<Self> {
        let canonical_url = canonical_url.into();
        let parts = parse_canonical_url(&canonical_url)?;

        Ok(Self {
            id: None,
            canonical_url: parts.canonical_url,
            scheme: parts.scheme,
            host: parts.host,
            path: parts.path,
            status: CrawlStatus::Pending,
            depth: 0,
            priority: 0,
            retry_count: 0,
        })
    }
}
