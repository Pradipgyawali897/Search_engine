use crate::error::DbResult;
use crate::models::url::parse_canonical_url;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LinkCategory {
    Visitable,
    Junk,
}

impl LinkCategory {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Visitable => "visitable",
            Self::Junk => "junk",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiscoveredLink {
    pub url: String,
    pub category: LinkCategory,
    pub timestamp: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_document_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crawl_target_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anchor_text: Option<String>,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub depth: i32,
}

impl DiscoveredLink {
    pub fn new(
        url: impl Into<String>,
        category: LinkCategory,
        timestamp: i64,
    ) -> DbResult<Self> {
        let url = url.into();
        let canonical_url = parse_canonical_url(&url)?.canonical_url;

        Ok(Self {
            url: canonical_url,
            category,
            timestamp,
            source_document_id: None,
            crawl_target_id: None,
            anchor_text: None,
            depth: 0,
        })
    }
}

fn is_zero(value: &i32) -> bool {
    *value == 0
}
