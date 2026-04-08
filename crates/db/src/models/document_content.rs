use crate::error::{DbError, DbResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct DocumentContent {
    pub document_id: i64,
    pub raw_html: Option<String>,
    pub plain_text: String,
    pub extracted_links_count: i32,
}

impl DocumentContent {
    pub fn new(
        document_id: i64,
        plain_text: impl Into<String>,
        extracted_links_count: i32,
    ) -> DbResult<Self> {
        if extracted_links_count < 0 {
            return Err(DbError::Validation(
                "extracted links count cannot be negative".to_string(),
            ));
        }

        Ok(Self {
            document_id,
            raw_html: None,
            plain_text: plain_text.into(),
            extracted_links_count,
        })
    }
}
